//! The Lua grammar: blocks and statements. Expressions live in
//! [`expressions`].
//!
//! Every function documents the production it parses. The grammar accepts the
//! union of all dialects, see [`crate::dialect`].

mod expressions;

use crate::{Feature, SyntaxKind, SyntaxKind::*, parser::Parser};

pub(crate) use expressions::{expr, expr_list};

/// Tokens that end a block.
const BLOCK_END: &[SyntaxKind] = &[EOF, END_KW, ELSE_KW, ELSEIF_KW, UNTIL_KW];

/// `chunk ::= block`
pub(crate) fn source_file(p: &mut Parser<'_>) {
    let m = p.open();
    loop {
        block(p);
        if p.eof() {
            break;
        }
        // A stray block terminator such as `end`: report it and go on.
        p.advance_with_error(format!("unexpected `{}`", p.current_text()));
    }
    p.close(m, SOURCE_FILE);
}

/// `block ::= {stat} [retstat]`
fn block(p: &mut Parser<'_>) {
    let m = p.open();
    while !p.at_any(BLOCK_END) {
        if p.at(RETURN_KW) {
            return_stmt(p);
            if !p.at_any(BLOCK_END) {
                p.error_at_current("`return` must be the last statement of its block");
            }
            continue;
        }
        statement(p);
    }
    p.close(m, BLOCK);
}

fn statement(p: &mut Parser<'_>) {
    match p.current() {
        SEMICOLON => p.advance(),
        LOCAL_KW => local_stmt(p),
        FUNCTION_KW => function_stmt(p),
        GLOBAL_KW => global_stmt(p),
        DO_KW => do_stmt(p),
        WHILE_KW => while_stmt(p),
        REPEAT_KW => repeat_stmt(p),
        IF_KW => if_stmt(p),
        FOR_KW => for_stmt(p),
        BREAK_KW => {
            let m = p.open();
            p.advance();
            p.close(m, BREAK_STMT);
        }
        GOTO_KW => goto_stmt(p),
        COLON2 => label_stmt(p),
        IDENT if is_contextual_keyword(p, "goto") => goto_stmt(p),
        IDENT if is_contextual_keyword(p, "global") => global_stmt(p),
        _ if expressions::at_expr_start(p) => expr_stmt(p),
        _ => p.advance_with_error(format!("expected a statement, found `{}`", p.current_text())),
    }
}

/// `goto` and `global` are plain names in dialects without the feature, but
/// `goto name` and `global name` can only be meant as statements: they are
/// parsed as such, and [`Parser::require`] reports the dialect mismatch.
fn is_contextual_keyword(p: &Parser<'_>, word: &str) -> bool {
    p.current_text() == word && matches!(p.nth(1), IDENT | FUNCTION_KW)
}

/// `stat ::= varlist '=' explist | functioncall`
fn expr_stmt(p: &mut Parser<'_>) {
    let Some(first) = expressions::suffixed_expr(p) else {
        // A literal or an operator: an expression that cannot be a statement.
        p.advance_with_error("syntax error: a statement must be a call or an assignment");
        return;
    };
    if !p.at_any(&[EQ, COMMA]) {
        let m = p.open_before(first.mark);
        if !first.is_call {
            p.error_at_current("syntax error: a statement must be a call or an assignment");
        }
        p.close(m, CALL_STMT);
        return;
    }
    let m = p.open_before(first.mark);
    while p.eat(COMMA) {
        expressions::suffixed_expr(p);
    }
    p.expect(EQ);
    expr_list(p);
    p.close(m, ASSIGN_STMT);
}

/// `local function Name funcbody | local attnamelist ['=' explist]`
fn local_stmt(p: &mut Parser<'_>) {
    let m = p.open();
    p.advance();
    if p.eat(FUNCTION_KW) {
        name(p);
        function_body(p);
        p.close(m, LOCAL_FUNCTION_STMT);
        return;
    }
    attrib_name_list(p);
    if p.eat(EQ) {
        expr_list(p);
    }
    p.close(m, LOCAL_STMT);
}

/// Lua 5.5:
/// `global function Name funcbody | global [attrib] attnamelist ['=' explist] | global [attrib] '*'`
fn global_stmt(p: &mut Parser<'_>) {
    let m = p.open();
    p.require_here(Feature::GlobalDeclarations);
    p.advance();
    if p.eat(FUNCTION_KW) {
        name(p);
        function_body(p);
        p.close(m, GLOBAL_STMT);
        return;
    }
    if p.at(LT) {
        attrib(p);
    }
    if !p.eat(STAR) {
        attrib_name_list(p);
        if p.eat(EQ) {
            expr_list(p);
        }
    }
    p.close(m, GLOBAL_STMT);
}

/// `attnamelist ::= Name attrib {',' Name attrib}`
fn attrib_name_list(p: &mut Parser<'_>) {
    let m = p.open();
    loop {
        name(p);
        if p.at(LT) {
            attrib(p);
        }
        if !p.eat(COMMA) {
            break;
        }
    }
    p.close(m, NAME_LIST);
}

/// `attrib ::= ['<' Name '>']`
fn attrib(p: &mut Parser<'_>) {
    let m = p.open();
    p.require_here(Feature::Attributes);
    p.advance();
    if p.at(IDENT) {
        let known = matches!(p.current_text(), "const" | "close");
        if known {
            p.advance();
        } else {
            p.advance_with_error("unknown attribute, expected `const` or `close`");
        }
    } else {
        p.error_expected("an attribute name");
    }
    p.expect(GT);
    p.close(m, ATTRIB);
}

/// `function funcname funcbody` with `funcname ::= Name {'.' Name} [':' Name]`
fn function_stmt(p: &mut Parser<'_>) {
    let m = p.open();
    p.advance();
    let name = p.open();
    p.expect(IDENT);
    while p.at(DOT) {
        p.advance();
        p.expect(IDENT);
    }
    if p.eat(COLON) {
        p.expect(IDENT);
    }
    p.close(name, FUNC_NAME);
    function_body(p);
    p.close(m, FUNCTION_STMT);
}

/// `funcbody ::= '(' [parlist] ')' block end`
pub(crate) fn function_body(p: &mut Parser<'_>) {
    param_list(p);
    block(p);
    p.expect(END_KW);
}

/// `'(' [Name {',' Name} [',' '...'] | '...'] ')'`
fn param_list(p: &mut Parser<'_>) {
    let m = p.open();
    if p.expect(L_PAREN) {
        while !p.at_any(&[R_PAREN, EOF]) {
            if p.eat(DOT3) {
                break;
            }
            if !p.at(IDENT) {
                p.error_expected("a parameter name");
                break;
            }
            name(p);
            if !p.eat(COMMA) {
                break;
            }
        }
        p.expect(R_PAREN);
    }
    p.close(m, PARAM_LIST);
}

/// A declared name.
fn name(p: &mut Parser<'_>) {
    if !p.at(IDENT) {
        p.error_expected("a name");
        return;
    }
    let m = p.open();
    p.advance();
    p.close(m, NAME);
}

/// `do block end`
fn do_stmt(p: &mut Parser<'_>) {
    let m = p.open();
    p.advance();
    block(p);
    p.expect(END_KW);
    p.close(m, DO_STMT);
}

/// `while exp do block end`
fn while_stmt(p: &mut Parser<'_>) {
    let m = p.open();
    p.advance();
    expr(p);
    p.expect(DO_KW);
    block(p);
    p.expect(END_KW);
    p.close(m, WHILE_STMT);
}

/// `repeat block until exp`
fn repeat_stmt(p: &mut Parser<'_>) {
    let m = p.open();
    p.advance();
    block(p);
    if p.expect(UNTIL_KW) {
        expr(p);
    }
    p.close(m, REPEAT_STMT);
}

/// `if exp then block {elseif exp then block} [else block] end`
fn if_stmt(p: &mut Parser<'_>) {
    let m = p.open();
    p.advance();
    expr(p);
    p.expect(THEN_KW);
    block(p);
    while p.at(ELSEIF_KW) {
        let clause = p.open();
        p.advance();
        expr(p);
        p.expect(THEN_KW);
        block(p);
        p.close(clause, ELSEIF_CLAUSE);
    }
    if p.at(ELSE_KW) {
        let clause = p.open();
        p.advance();
        block(p);
        p.close(clause, ELSE_CLAUSE);
    }
    p.expect(END_KW);
    p.close(m, IF_STMT);
}

/// `for Name '=' exp ',' exp [',' exp] do block end`
/// `for namelist in explist do block end`
fn for_stmt(p: &mut Parser<'_>) {
    let m = p.open();
    p.advance();
    let numeric = p.nth(1) == EQ;
    if numeric {
        name(p);
        p.advance();
        expr_list(p);
    } else {
        let names = p.open();
        loop {
            name(p);
            if !p.eat(COMMA) {
                break;
            }
        }
        p.close(names, NAME_LIST);
        p.expect(IN_KW);
        expr_list(p);
    }
    p.expect(DO_KW);
    block(p);
    p.expect(END_KW);
    p.close(m, if numeric { NUMERIC_FOR_STMT } else { GENERIC_FOR_STMT });
}

/// `retstat ::= return [explist] [';']`
fn return_stmt(p: &mut Parser<'_>) {
    let m = p.open();
    p.advance();
    if expressions::at_expr_start(p) {
        expr_list(p);
    }
    p.eat(SEMICOLON);
    p.close(m, RETURN_STMT);
}

/// `goto Name`
fn goto_stmt(p: &mut Parser<'_>) {
    let m = p.open();
    p.require_here(Feature::Goto);
    p.advance();
    p.expect(IDENT);
    p.close(m, GOTO_STMT);
}

/// `label ::= '::' Name '::'`
fn label_stmt(p: &mut Parser<'_>) {
    let m = p.open();
    p.require_here(Feature::Goto);
    p.advance();
    name(p);
    p.expect(COLON2);
    p.close(m, LABEL_STMT);
}
