//! Spaces between two tokens of the same line.

use xmake_syntax::{SyntaxKind, SyntaxKind::*, SyntaxToken};

/// Whether to put a space between `prev` and `next`. `had_space` tells
/// whether the source had one, which decides the cases without a rule.
pub(crate) fn space_between(prev: SyntaxToken<'_>, next: SyntaxToken<'_>, had_space: bool) -> bool {
    let wanted = rule(prev, next).unwrap_or(had_space);
    // Never glue tokens that would lex differently (`- -x` is not `--x`).
    wanted || would_merge(prev.text(), next.text())
}

fn parent_kind(token: SyntaxToken<'_>) -> Option<SyntaxKind> {
    token.parent().map(|p| p.kind())
}

fn is_binary_operator(token: SyntaxToken<'_>) -> bool {
    match token.kind() {
        EQ => matches!(
            parent_kind(token),
            Some(ASSIGN_STMT | LOCAL_STMT | GLOBAL_STMT | TABLE_FIELD | NUMERIC_FOR_STMT)
        ),
        AND_KW | OR_KW | LT | GT | LTEQ | GTEQ | NEQ | EQ2 | PIPE | TILDE | AMP | SHL | SHR
        | DOT2 | PLUS | MINUS | STAR | SLASH | SLASH2 | PERCENT | CARET => {
            parent_kind(token) == Some(BIN_EXPR)
        }
        _ => false,
    }
}

fn is_unary_operator(token: SyntaxToken<'_>) -> bool {
    matches!(token.kind(), MINUS | HASH | TILDE) && parent_kind(token) == Some(UNARY_EXPR)
}

/// The spacing rules, `None` when the source decides.
fn rule(prev: SyntaxToken<'_>, next: SyntaxToken<'_>) -> Option<bool> {
    // Comments keep their distance from code, at least one space.
    if next.kind().is_comment() {
        return Some(true);
    }
    if prev.kind().is_comment() {
        return None;
    }
    // Closing and separating punctuation sticks to what precedes it.
    if matches!(next.kind(), COMMA | SEMICOLON | R_PAREN | R_BRACK | R_BRACE) {
        return Some(false);
    }
    if matches!(prev.kind(), L_PAREN | L_BRACK | L_BRACE) {
        return Some(false);
    }
    if matches!(prev.kind(), COMMA | SEMICOLON) {
        return Some(true);
    }
    // Member access: `a.b`, `a:b()`.
    if matches!(prev.kind(), DOT | COLON) || matches!(next.kind(), DOT | COLON) {
        return Some(false);
    }
    // `::label::`
    if prev.kind() == COLON2 || next.kind() == COLON2 && parent_kind(next) == Some(LABEL_STMT) {
        let opening = prev.kind() != COLON2
            && next.text_range().start() == next.parent()?.text_range().start();
        return Some(opening);
    }
    // `local x <const>`
    if parent_kind(prev) == Some(ATTRIB) && prev.kind() == LT
        || parent_kind(next) == Some(ATTRIB) && next.kind() == GT
    {
        return Some(false);
    }
    if is_unary_operator(prev) {
        return Some(false);
    }
    if is_binary_operator(prev) || is_binary_operator(next) {
        return Some(true);
    }
    if next.kind() == L_PAREN {
        return match parent_kind(next) {
            // `f(x)`
            Some(ARG_LIST) => Some(false),
            // `function (target)` for anonymous functions, as in the manual;
            // `function name(x)` for named ones.
            Some(PARAM_LIST) => {
                let function = next.parent()?.parent()?;
                Some(function.kind() == FUNCTION_EXPR)
            }
            _ => Some(true),
        };
    }
    // `t[i]`
    if next.kind() == L_BRACK && parent_kind(next) == Some(INDEX_EXPR) {
        return Some(false);
    }
    // `f "x"`, `f {...}`: as written.
    if parent_kind(next).is_some_and(|k| k == ARG_LIST || k == LITERAL || k == TABLE_EXPR)
        && next.parent().and_then(|p| p.parent()).is_some_and(|p| p.kind() == ARG_LIST)
        && next.parent().and_then(|p| p.parent()).and_then(|a| a.child_token(L_PAREN)).is_none()
    {
        return None;
    }
    if prev.kind().is_keyword() || next.kind().is_keyword() {
        return Some(true);
    }
    None
}

/// Whether `a` directly followed by `b` would lex as other tokens.
fn would_merge(a: &str, b: &str) -> bool {
    let (Some(x), Some(y)) = (a.bytes().last(), b.bytes().next()) else {
        return false;
    };
    let word = |c: u8| c == b'_' || c.is_ascii_alphanumeric();
    match (x, y) {
        _ if word(x) && word(y) => true,
        // `1` `.5`, `a.` ... numbers continue with dots.
        (b'.', b'.' | b'0'..=b'9') => true,
        (b'0'..=b'9', b'.') => true,
        (b'-', b'-') | (b'/', b'/') | (b':', b':') | (b'<', b'<') | (b'>', b'>') => true,
        (b'[', b'[' | b'=') => true,
        (b'=' | b'<' | b'>' | b'~', b'=') => true,
        _ => false,
    }
}
