//! The parser state machine, in the style of matklad's
//! [resilient LL parsing tutorial](https://matklad.github.io/2023/05/21/resilient-ll-parsing-tutorial.html).
//!
//! The parser only sees significant tokens: trivia is filtered out beforehand
//! and re-attached when the tree is built. Grammar functions emit a flat list
//! of [`Event`]s, which [`crate::build_tree`] turns into a tree.

use std::{borrow::Cow, cell::Cell};

use text_size::{TextRange, TextSize};

use crate::{Dialect, Feature, SyntaxError, SyntaxKind};

/// A significant (non-trivia) token as seen by the parser.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Tok {
    pub(crate) kind: SyntaxKind,
    pub(crate) range: TextRange,
}

#[derive(Debug)]
pub(crate) enum Event {
    Open { kind: SyntaxKind },
    Close,
    Advance,
}

/// A node that has been started but not closed yet.
pub(crate) struct MarkOpened {
    index: usize,
}

/// A closed node, which [`Parser::open_before`] can still wrap in a parent.
#[derive(Clone, Copy)]
pub(crate) struct MarkClosed {
    index: usize,
}

pub(crate) struct Parser<'t> {
    text: &'t str,
    tokens: &'t [Tok],
    dialect: Dialect,
    pos: usize,
    /// Guards against grammar bugs that would loop without consuming input.
    fuel: Cell<u32>,
    pub(crate) events: Vec<Event>,
    pub(crate) errors: Vec<SyntaxError>,
}

const FUEL: u32 = 256;

impl<'t> Parser<'t> {
    pub(crate) fn new(text: &'t str, tokens: &'t [Tok], dialect: Dialect) -> Parser<'t> {
        Parser {
            text,
            tokens,
            dialect,
            pos: 0,
            fuel: Cell::new(FUEL),
            events: Vec::with_capacity(tokens.len() * 2),
            errors: Vec::new(),
        }
    }

    // === Tree structure ===

    pub(crate) fn open(&mut self) -> MarkOpened {
        let mark = MarkOpened { index: self.events.len() };
        self.events.push(Event::Open { kind: SyntaxKind::ERROR });
        mark
    }

    /// Starts a node that becomes the parent of the already closed `m`.
    pub(crate) fn open_before(&mut self, m: MarkClosed) -> MarkOpened {
        self.events.insert(m.index, Event::Open { kind: SyntaxKind::ERROR });
        MarkOpened { index: m.index }
    }

    pub(crate) fn close(&mut self, m: MarkOpened, kind: SyntaxKind) -> MarkClosed {
        self.events[m.index] = Event::Open { kind };
        self.events.push(Event::Close);
        MarkClosed { index: m.index }
    }

    // === Lookahead ===

    pub(crate) fn nth(&self, lookahead: usize) -> SyntaxKind {
        let fuel = self.fuel.get();
        assert!(fuel != 0, "parser is stuck at token {}", self.pos);
        self.fuel.set(fuel - 1);
        self.tokens.get(self.pos + lookahead).map_or(SyntaxKind::EOF, |t| t.kind)
    }

    pub(crate) fn current(&self) -> SyntaxKind {
        self.nth(0)
    }

    pub(crate) fn at(&self, kind: SyntaxKind) -> bool {
        self.nth(0) == kind
    }

    pub(crate) fn at_any(&self, kinds: &[SyntaxKind]) -> bool {
        kinds.contains(&self.nth(0))
    }

    pub(crate) fn eof(&self) -> bool {
        self.pos == self.tokens.len()
    }

    /// The text of the current token, empty at the end of input.
    pub(crate) fn current_text(&self) -> &'t str {
        self.tokens.get(self.pos).map_or("", |t| &self.text[t.range])
    }

    // === Consuming ===

    pub(crate) fn advance(&mut self) {
        assert!(!self.eof(), "advance past the end of input");
        self.fuel.set(FUEL);
        self.events.push(Event::Advance);
        self.pos += 1;
    }

    pub(crate) fn eat(&mut self, kind: SyntaxKind) -> bool {
        if !self.at(kind) {
            return false;
        }
        self.advance();
        true
    }

    /// Consumes `kind` or reports that it is missing.
    pub(crate) fn expect(&mut self, kind: SyntaxKind) -> bool {
        if self.eat(kind) {
            return true;
        }
        self.error_expected(kind.describe());
        false
    }

    /// Wraps the current token in an `ERROR` node and reports `message` on it.
    pub(crate) fn advance_with_error(&mut self, message: impl Into<Cow<'static, str>>) {
        let m = self.open();
        self.error_at_current(message);
        self.advance();
        self.close(m, SyntaxKind::ERROR);
    }

    // === Dialect ===

    /// Reports `range` when the dialect lacks `feature`. The construct is
    /// parsed either way.
    pub(crate) fn require(&mut self, feature: Feature, range: TextRange) {
        if self.dialect.has(feature) {
            return;
        }
        let message = format!(
            "{} requires {} or newer (current: {})",
            feature.describe(),
            feature.since(),
            self.dialect.lua
        );
        self.push_error(range, Cow::Owned(message));
    }

    /// [`Parser::require`] on the current token.
    pub(crate) fn require_here(&mut self, feature: Feature) {
        let range = self.current_range();
        self.require(feature, range);
    }

    // === Errors ===

    /// Reports that `what` was expected, pointing just after the last consumed
    /// token (or at the current token when nothing was consumed yet).
    pub(crate) fn error_expected(&mut self, what: &'static str) {
        let message = match self.tokens.get(self.pos) {
            None => Cow::Owned(format!("expected {what}")),
            Some(_) => Cow::Owned(format!("expected {what}, found {}", self.current_desc())),
        };
        let range = match self.pos.checked_sub(1).and_then(|i| self.tokens.get(i)) {
            Some(prev) => TextRange::empty(prev.range.end()),
            None => self.current_range(),
        };
        self.push_error(range, message);
    }

    /// Reports `message` on the current token.
    pub(crate) fn error_at_current(&mut self, message: impl Into<Cow<'static, str>>) {
        let range = self.current_range();
        self.push_error(range, message.into());
    }

    fn push_error(&mut self, range: TextRange, message: Cow<'static, str>) {
        // Avoid cascades of errors at the same spot.
        if self.errors.last().is_some_and(|e| e.range() == range) {
            return;
        }
        self.errors.push(SyntaxError::new(message, range));
    }

    pub(crate) fn current_range(&self) -> TextRange {
        let eof = TextSize::of(self.text);
        self.tokens.get(self.pos).map_or(TextRange::empty(eof), |t| t.range)
    }

    fn current_desc(&self) -> String {
        let text = self.current_text();
        if text.len() <= 32 {
            return format!("`{text}`");
        }
        let cut = (0..=32).rev().find(|&i| text.is_char_boundary(i)).unwrap_or(0);
        format!("`{}...`", &text[..cut])
    }
}
