//! Expressions, parsed with precedence climbing.

use crate::{
    Feature, SyntaxKind,
    SyntaxKind::*,
    grammar::function_body,
    parser::{MarkClosed, Parser},
};

/// Tokens that can start an expression.
const EXPR_START: &[SyntaxKind] = &[
    NIL_KW,
    TRUE_KW,
    FALSE_KW,
    INT,
    FLOAT,
    STRING,
    LONG_STRING,
    DOT3,
    FUNCTION_KW,
    L_BRACE,
    IDENT,
    L_PAREN,
    NOT_KW,
    MINUS,
    HASH,
    TILDE,
];

pub(crate) fn at_expr_start(p: &Parser<'_>) -> bool {
    p.at_any(EXPR_START)
}

/// `explist ::= exp {',' exp}`
pub(crate) fn expr_list(p: &mut Parser<'_>) {
    let m = p.open();
    expr(p);
    while p.eat(COMMA) {
        expr(p);
    }
    p.close(m, EXPR_LIST);
}

/// Parses one expression, reporting when there is none.
pub(crate) fn expr(p: &mut Parser<'_>) {
    if expr_bp(p, 0).is_none() {
        p.error_expected("an expression");
    }
}

/// Binding powers `(left, right)` of a binary operator. Right associative
/// operators (`..`, `^`) bind tighter on their left.
fn infix_binding_power(kind: SyntaxKind) -> Option<(u8, u8)> {
    let bp = match kind {
        OR_KW => (1, 2),
        AND_KW => (3, 4),
        LT | GT | LTEQ | GTEQ | NEQ | EQ2 => (5, 6),
        PIPE => (7, 8),
        TILDE => (9, 10),
        AMP => (11, 12),
        SHL | SHR => (13, 14),
        DOT2 => (16, 15),
        PLUS | MINUS => (17, 18),
        STAR | SLASH | SLASH2 | PERCENT => (19, 20),
        CARET => (24, 23),
        _ => return None,
    };
    Some(bp)
}

/// Unary operators bind tighter than every binary operator but `^`.
const UNARY_BP: u8 = 21;

/// The dialect feature an operator needs, if any.
fn operator_feature(kind: SyntaxKind) -> Option<Feature> {
    match kind {
        SLASH2 => Some(Feature::IntegerDivision),
        AMP | PIPE | TILDE | SHL | SHR => Some(Feature::BitwiseOperators),
        _ => None,
    }
}

fn expr_bp(p: &mut Parser<'_>, min_bp: u8) -> Option<MarkClosed> {
    let mut lhs = unary_or_simple(p)?;
    loop {
        let op = p.current();
        let Some((left, right)) = infix_binding_power(op) else {
            break;
        };
        if left < min_bp {
            break;
        }
        if let Some(feature) = operator_feature(op) {
            p.require_here(feature);
        }
        let m = p.open_before(lhs);
        p.advance();
        if expr_bp(p, right).is_none() {
            p.error_expected("an expression");
        }
        lhs = p.close(m, BIN_EXPR);
    }
    Some(lhs)
}

fn unary_or_simple(p: &mut Parser<'_>) -> Option<MarkClosed> {
    if !p.at_any(&[NOT_KW, MINUS, HASH, TILDE]) {
        return simple_expr(p);
    }
    if p.at(TILDE) {
        p.require_here(Feature::BitwiseOperators);
    }
    let m = p.open();
    p.advance();
    if expr_bp(p, UNARY_BP).is_none() {
        p.error_expected("an expression");
    }
    Some(p.close(m, UNARY_EXPR))
}

/// `nil | false | true | Numeral | LiteralString | '...' | functiondef |
/// tableconstructor | suffixedexp`
fn simple_expr(p: &mut Parser<'_>) -> Option<MarkClosed> {
    match p.current() {
        NIL_KW | TRUE_KW | FALSE_KW | INT | FLOAT | STRING | LONG_STRING | DOT3 => {
            let m = p.open();
            p.advance();
            Some(p.close(m, LITERAL))
        }
        FUNCTION_KW => {
            let m = p.open();
            p.advance();
            function_body(p);
            Some(p.close(m, FUNCTION_EXPR))
        }
        L_BRACE => Some(table(p)),
        _ => suffixed_expr(p).map(|s| s.mark),
    }
}

/// The result of [`suffixed_expr`].
pub(crate) struct Suffixed {
    pub(crate) mark: MarkClosed,
    /// Whether the outermost suffix is a call, i.e. it is a valid statement.
    pub(crate) is_call: bool,
}

/// `suffixedexp ::= primaryexp { '.' Name | '[' exp ']' | ':' Name args | args }`
pub(crate) fn suffixed_expr(p: &mut Parser<'_>) -> Option<Suffixed> {
    let mut lhs = primary_expr(p)?;
    let mut is_call = false;
    loop {
        match p.current() {
            DOT => {
                let m = p.open_before(lhs);
                p.advance();
                p.expect(IDENT);
                lhs = p.close(m, FIELD_EXPR);
                is_call = false;
            }
            L_BRACK => {
                let m = p.open_before(lhs);
                p.advance();
                expr(p);
                p.expect(R_BRACK);
                lhs = p.close(m, INDEX_EXPR);
                is_call = false;
            }
            COLON => {
                let m = p.open_before(lhs);
                p.advance();
                p.expect(IDENT);
                if at_args(p) {
                    args(p);
                } else {
                    p.error_expected("method arguments");
                }
                lhs = p.close(m, METHOD_CALL_EXPR);
                is_call = true;
            }
            _ if at_args(p) => {
                let m = p.open_before(lhs);
                args(p);
                lhs = p.close(m, CALL_EXPR);
                is_call = true;
            }
            _ => break,
        }
    }
    Some(Suffixed { mark: lhs, is_call })
}

/// `primaryexp ::= Name | '(' exp ')'`
fn primary_expr(p: &mut Parser<'_>) -> Option<MarkClosed> {
    match p.current() {
        IDENT => {
            let m = p.open();
            p.advance();
            Some(p.close(m, NAME_REF))
        }
        L_PAREN => {
            let m = p.open();
            p.advance();
            expr(p);
            p.expect(R_PAREN);
            Some(p.close(m, PAREN_EXPR))
        }
        _ => None,
    }
}

fn at_args(p: &Parser<'_>) -> bool {
    p.at_any(&[L_PAREN, L_BRACE, STRING, LONG_STRING])
}

/// `args ::= '(' [explist] ')' | tableconstructor | LiteralString`
fn args(p: &mut Parser<'_>) {
    let m = p.open();
    match p.current() {
        L_PAREN => {
            p.advance();
            while !p.at_any(&[R_PAREN, EOF]) {
                expr(p);
                if !p.eat(COMMA) {
                    break;
                }
            }
            p.expect(R_PAREN);
        }
        L_BRACE => {
            table(p);
        }
        _ => {
            let lit = p.open();
            p.advance();
            p.close(lit, LITERAL);
        }
    }
    p.close(m, ARG_LIST);
}

/// `tableconstructor ::= '{' [fieldlist] '}'`
fn table(p: &mut Parser<'_>) -> MarkClosed {
    let m = p.open();
    p.advance();
    while !p.at_any(&[R_BRACE, EOF]) {
        let before = p.current_range();
        table_field(p);
        if !p.eat(COMMA) && !p.eat(SEMICOLON) {
            break;
        }
        debug_assert!(p.current_range() != before || p.eof());
    }
    p.expect(R_BRACE);
    p.close(m, TABLE_EXPR)
}

/// `field ::= '[' exp ']' '=' exp | Name '=' exp | exp`
fn table_field(p: &mut Parser<'_>) {
    let m = p.open();
    if p.at(L_BRACK) {
        p.advance();
        expr(p);
        p.expect(R_BRACK);
        p.expect(EQ);
        expr(p);
    } else if p.at(IDENT) && p.nth(1) == EQ {
        p.advance();
        p.advance();
        expr(p);
    } else if at_expr_start(p) {
        expr(p);
    } else {
        p.advance_with_error(format!("expected a table field, found `{}`", p.current_text()));
    }
    p.close(m, TABLE_FIELD);
}
