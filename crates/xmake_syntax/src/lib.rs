//! Resilient, lossless parser for Lua as used by xmake.
//!
//! [`parse`] turns source text into a [`SyntaxTree`]: every byte of the input
//! belongs to exactly one token, and malformed input produces `ERROR` nodes and
//! [`SyntaxError`]s while the surrounding code still gets a sensible tree.
//!
//! The pipeline is:
//!
//! 1. [`lexer`] splits the text into tokens (no allocation per token);
//! 2. trivia is filtered out;
//! 3. the grammar emits open/close/advance events;
//! 4. the events are replayed together with the trivia into a flat tree.
//!
//! Which Lua version the file targets is a [`Dialect`]: the grammar accepts
//! every version, and reports what the selected one does not support.

pub mod ast;
pub mod dialect;
mod grammar;
pub mod lexer;
mod parser;
mod syntax_kind;
mod tree;

#[cfg(test)]
mod tests;

use std::{borrow::Cow, fmt};

pub use text_size::{TextRange, TextSize};

pub use crate::{
    dialect::{Dialect, Feature, Features, LuaVersion},
    syntax_kind::SyntaxKind,
    tree::{Children, SyntaxElement, SyntaxNode, SyntaxToken, SyntaxTree, TokenAtOffset},
};

use crate::{
    parser::{Event, Parser, Tok},
    tree::TreeBuilder,
};

/// A syntax or lexical error with its location.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SyntaxError {
    /// Most messages are static: only those quoting the source allocate.
    message: Cow<'static, str>,
    range: TextRange,
}

impl SyntaxError {
    /// Creates a new error.
    pub fn new(message: impl Into<Cow<'static, str>>, range: TextRange) -> SyntaxError {
        SyntaxError { message: message.into(), range }
    }

    /// The error message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// The location of the error. Empty for a missing token.
    pub fn range(&self) -> TextRange {
        self.range
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.range, self.message)
    }
}

/// The result of parsing: an owned tree plus errors.
///
/// The tree does not borrow the text: pair it with the text it was built from
/// through [`Parse::root`].
#[derive(Clone, Debug)]
pub struct Parse {
    tree: SyntaxTree,
    errors: Vec<SyntaxError>,
}

impl Parse {
    /// The root node. `text` must be the parsed source.
    pub fn root<'a>(&'a self, text: &'a str) -> SyntaxNode<'a> {
        self.tree.root(text)
    }

    /// Lexical, syntax and dialect errors, ordered by position.
    pub fn errors(&self) -> &[SyntaxError] {
        &self.errors
    }

    /// Renders the tree followed by the errors, used in tests.
    pub fn debug_dump(&self, text: &str) -> String {
        use std::fmt::Write as _;

        let mut out = self.root(text).debug_dump();
        for e in &self.errors {
            let _ = writeln!(out, "error {:?}: {}", e.range, e.message);
        }
        out
    }
}

/// Parses a Lua chunk written for `dialect`.
///
/// The text must be smaller than 4 GiB.
pub fn parse(text: &str, dialect: Dialect) -> Parse {
    let mut all: Vec<(SyntaxKind, u32)> = Vec::with_capacity(text.len() / 4);
    let mut significant: Vec<Tok> = Vec::with_capacity(text.len() / 6);
    let mut errors = Vec::new();

    for token in lexer::Lexer::new(text, dialect.lua.features()) {
        let range = TextRange::at(token.offset.into(), token.len.into());
        if let Some(err) = token.error {
            errors.push(SyntaxError::new(err.message(), range));
        }
        all.push((token.kind, token.len));
        if !token.kind.is_trivia() {
            significant.push(Tok { kind: token.kind, range });
        }
    }

    let mut p = Parser::new(text, &significant, dialect);
    grammar::source_file(&mut p);
    let Parser { events, errors: parse_errors, .. } = p;

    errors.extend(parse_errors);
    errors.sort_by_key(|e| (e.range.start(), e.range.end()));

    Parse { tree: build_tree(&all, events), errors }
}

/// Replays parser `events` over all tokens, re-inserting trivia.
///
/// Trivia preceding a node is attached to the node's parent, so that node
/// ranges start at their first significant token. Trailing trivia ends up in
/// the innermost node open at that point.
fn build_tree(tokens: &[(SyntaxKind, u32)], events: Vec<Event>) -> SyntaxTree {
    let mut builder = TreeBuilder::default();
    let mut next = 0usize;
    let mut depth = 0usize;

    let flush_trivia = |builder: &mut TreeBuilder, next: &mut usize| {
        while let Some(&(kind, len)) = tokens.get(*next) {
            if !kind.is_trivia() {
                break;
            }
            builder.token(kind, len);
            *next += 1;
        }
    };

    for event in events {
        match event {
            Event::Open { kind } => {
                if builder.is_inside_node() {
                    flush_trivia(&mut builder, &mut next);
                }
                builder.start_node(kind);
                depth += 1;
            }
            Event::Close => {
                if depth == 1 {
                    // Closing the root: everything left is trailing trivia.
                    while let Some(&(kind, len)) = tokens.get(next) {
                        builder.token(kind, len);
                        next += 1;
                    }
                }
                builder.finish_node();
                depth -= 1;
            }
            Event::Advance => {
                flush_trivia(&mut builder, &mut next);
                let (kind, len) = tokens[next];
                builder.token(kind, len);
                next += 1;
            }
        }
    }
    debug_assert_eq!(next, tokens.len(), "not all tokens were consumed");
    builder.finish()
}
