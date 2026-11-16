//! Indentation levels from the syntax tree.
//!
//! A line is indented one level deeper than the line where its innermost
//! enclosing construct starts: a block (`function`, `if`, `for`, ...), a table,
//! a parenthesized argument or parameter list, or a statement continued over
//! several lines. Constructs opening and closing on the same line do not
//! count, so `on_build(function (target)` indents its body once, not twice.
//! Closing tokens (`end`, `}`, `)`, `else`, ...) line up with the line of
//! their opening.

use xmake_api::{Scope, ScopeEdge};
use xmake_syntax::{
    SyntaxKind, SyntaxNode, SyntaxToken, TextSize,
    ast::{AstNode, CallExpr},
};

pub(crate) struct Levels {
    /// Byte offset of the start of each line.
    line_starts: Vec<u32>,
    /// The level of each line, once its first token is emitted.
    levels: Vec<Option<u32>>,
    /// Start of the top level statements inside a scope, sorted.
    scoped_statements: Vec<TextSize>,
}

impl Levels {
    pub(crate) fn new(root: SyntaxNode<'_>, text: &str, indent_scopes: bool) -> Levels {
        let mut line_starts = vec![0];
        line_starts.extend(
            text.bytes().enumerate().filter(|&(_, b)| b == b'\n').map(|(i, _)| i as u32 + 1),
        );
        let levels = vec![None; line_starts.len()];
        let scoped_statements = if indent_scopes { scoped_statements(root) } else { Vec::new() };
        Levels { line_starts, levels, scoped_statements }
    }

    fn line_of(&self, offset: TextSize) -> usize {
        self.line_starts.partition_point(|&s| s <= u32::from(offset)) - 1
    }

    /// The level of `line`, or of the closest line above with one (a line
    /// starting inside a long string has none).
    fn line_level(&self, line: usize) -> u32 {
        self.levels[..=line].iter().rev().find_map(|l| *l).unwrap_or(0)
    }

    /// The level of `token`, the first token of its line, and records it.
    /// `rest` are the tokens after it, to place comments.
    pub(crate) fn level_of(&mut self, token: SyntaxToken<'_>, rest: &[SyntaxToken<'_>]) -> u32 {
        let level = if token.kind().is_trivia() {
            // A comment goes with the code after it; before a closing token
            // it still belongs to the block being closed.
            let next = rest.iter().find(|t| !t.kind().is_trivia());
            match next {
                Some(&next) => self.level(next) + u32::from(is_closer(next.kind())),
                None => 0,
            }
        } else {
            self.level(token)
        };
        let line = self.line_of(token.text_range().start());
        self.levels[line] = Some(level);
        level
    }

    fn level(&self, token: SyntaxToken<'_>) -> u32 {
        let line = self.line_of(token.text_range().start());
        if let Some(opening) = opening_of_closer(token) {
            return self.line_level(self.line_of(opening));
        }
        for ancestor in token.ancestors() {
            let Some(start) = container_start(ancestor) else {
                continue;
            };
            let start_line = self.line_of(start);
            if start_line < line {
                return self.line_level(start_line) + 1;
            }
        }
        let top_level = token.ancestors().find(|n| n.parent().is_some_and(|p| is_root_block(p)));
        let scoped = top_level
            .is_some_and(|s| self.scoped_statements.binary_search(&s.text_range().start()).is_ok());
        u32::from(scoped)
    }
}

fn is_closer(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        SyntaxKind::END_KW
            | SyntaxKind::ELSE_KW
            | SyntaxKind::ELSEIF_KW
            | SyntaxKind::UNTIL_KW
            | SyntaxKind::R_BRACE
            | SyntaxKind::R_PAREN
            | SyntaxKind::R_BRACK
    )
}

fn is_root_block(node: SyntaxNode<'_>) -> bool {
    node.kind() == SyntaxKind::BLOCK
        && node.parent().is_some_and(|p| p.kind() == SyntaxKind::SOURCE_FILE)
}

/// For a closing token, the start of the construct it closes.
fn opening_of_closer(token: SyntaxToken<'_>) -> Option<TextSize> {
    let parent = token.parent()?;
    match token.kind() {
        SyntaxKind::END_KW | SyntaxKind::UNTIL_KW => Some(parent.text_range().start()),
        // `else`/`elseif` start their clause: align with the `if`.
        SyntaxKind::ELSE_KW | SyntaxKind::ELSEIF_KW => Some(parent.parent()?.text_range().start()),
        SyntaxKind::R_BRACE | SyntaxKind::R_PAREN | SyntaxKind::R_BRACK => {
            let open = match token.kind() {
                SyntaxKind::R_BRACE => SyntaxKind::L_BRACE,
                SyntaxKind::R_PAREN => SyntaxKind::L_PAREN,
                _ => SyntaxKind::L_BRACK,
            };
            Some(parent.child_token(open)?.text_range().start())
        }
        _ => None,
    }
}

/// Where a construct indenting its inner lines starts, if `node` is one.
fn container_start(node: SyntaxNode<'_>) -> Option<TextSize> {
    match node.kind() {
        // A block is indented relative to the statement owning it.
        SyntaxKind::BLOCK if !is_root_block(node) => Some(node.parent()?.text_range().start()),
        SyntaxKind::TABLE_EXPR | SyntaxKind::PARAM_LIST | SyntaxKind::PAREN_EXPR => {
            Some(node.text_range().start())
        }
        SyntaxKind::ARG_LIST if node.child_token(SyntaxKind::L_PAREN).is_some() => {
            Some(node.text_range().start())
        }
        // A statement continued on the next lines.
        _ if node.parent().is_some_and(|p| p.kind() == SyntaxKind::BLOCK) => {
            Some(node.text_range().start())
        }
        _ => None,
    }
}

/// The top level statements between a scope call (`target("x")`) and the end
/// of its scope.
fn scoped_statements(root: SyntaxNode<'_>) -> Vec<TextSize> {
    let Some(block) = root.child_node(SyntaxKind::BLOCK) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    let mut in_scope = false;
    for stmt in block.children() {
        let call = (stmt.kind() == SyntaxKind::CALL_STMT)
            .then(|| stmt.children().find_map(CallExpr::cast))
            .flatten();
        let edge = call.and_then(|c| Some((c, Scope::from_opener(c.callee_name()?)?.1)));
        match edge {
            Some((call, ScopeEdge::Begin)) => {
                // `target("x", {...})` / `target("x", function () end)` close at once.
                let one_shot = call.arg(1).is_some_and(|a| {
                    matches!(a.kind(), SyntaxKind::TABLE_EXPR | SyntaxKind::FUNCTION_EXPR)
                });
                in_scope = !one_shot;
            }
            Some((_, ScopeEdge::End)) => in_scope = false,
            None if in_scope => out.push(stmt.text_range().start()),
            None => {}
        }
    }
    out
}
