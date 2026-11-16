use expect_test::{Expect, expect};

use crate::{Dialect, LuaVersion, parse};

fn check(text: &str, expect: Expect) {
    let parse = parse(text, Dialect::XMAKE_DEFAULT);
    expect.assert_eq(&parse.debug_dump(text));
}

fn errors(text: &str, lua: LuaVersion) -> Vec<String> {
    let parse = parse(text, Dialect { lua });
    parse.errors().iter().map(|e| format!("{:?} {}", e.range(), e.message())).collect()
}

#[test]
fn target_scope() {
    check(
        "target(\"foo\")\n    set_kind(\"binary\")\n",
        expect![[r#"
            SOURCE_FILE@0..37
              BLOCK@0..36
                CALL_STMT@0..13
                  CALL_EXPR@0..13
                    NAME_REF@0..6
                      IDENT@0..6 "target"
                    ARG_LIST@6..13
                      L_PAREN@6..7 "("
                      LITERAL@7..12
                        STRING@7..12 "\"foo\""
                      R_PAREN@12..13 ")"
                WHITESPACE@13..18 "\n    "
                CALL_STMT@18..36
                  CALL_EXPR@18..36
                    NAME_REF@18..26
                      IDENT@18..26 "set_kind"
                    ARG_LIST@26..36
                      L_PAREN@26..27 "("
                      LITERAL@27..35
                        STRING@27..35 "\"binary\""
                      R_PAREN@35..36 ")"
              WHITESPACE@36..37 "\n"
        "#]],
    );
}

#[test]
fn script_function() {
    check(
        "on_build(function (target) print(target:name()) end)",
        expect![[r#"
            SOURCE_FILE@0..52
              BLOCK@0..52
                CALL_STMT@0..52
                  CALL_EXPR@0..52
                    NAME_REF@0..8
                      IDENT@0..8 "on_build"
                    ARG_LIST@8..52
                      L_PAREN@8..9 "("
                      FUNCTION_EXPR@9..51
                        FUNCTION_KW@9..17 "function"
                        WHITESPACE@17..18 " "
                        PARAM_LIST@18..26
                          L_PAREN@18..19 "("
                          NAME@19..25
                            IDENT@19..25 "target"
                          R_PAREN@25..26 ")"
                        WHITESPACE@26..27 " "
                        BLOCK@27..47
                          CALL_STMT@27..47
                            CALL_EXPR@27..47
                              NAME_REF@27..32
                                IDENT@27..32 "print"
                              ARG_LIST@32..47
                                L_PAREN@32..33 "("
                                METHOD_CALL_EXPR@33..46
                                  NAME_REF@33..39
                                    IDENT@33..39 "target"
                                  COLON@39..40 ":"
                                  IDENT@40..44 "name"
                                  ARG_LIST@44..46
                                    L_PAREN@44..45 "("
                                    R_PAREN@45..46 ")"
                                R_PAREN@46..47 ")"
                        WHITESPACE@47..48 " "
                        END_KW@48..51 "end"
                      R_PAREN@51..52 ")"
        "#]],
    );
}

#[test]
fn precedence() {
    check(
        "x = -a ^ 2 .. b or c",
        expect![[r#"
            SOURCE_FILE@0..20
              BLOCK@0..20
                ASSIGN_STMT@0..20
                  NAME_REF@0..1
                    IDENT@0..1 "x"
                  WHITESPACE@1..2 " "
                  EQ@2..3 "="
                  WHITESPACE@3..4 " "
                  EXPR_LIST@4..20
                    BIN_EXPR@4..20
                      BIN_EXPR@4..15
                        UNARY_EXPR@4..10
                          MINUS@4..5 "-"
                          BIN_EXPR@5..10
                            NAME_REF@5..6
                              IDENT@5..6 "a"
                            WHITESPACE@6..7 " "
                            CARET@7..8 "^"
                            WHITESPACE@8..9 " "
                            LITERAL@9..10
                              INT@9..10 "2"
                        WHITESPACE@10..11 " "
                        DOT2@11..13 ".."
                        WHITESPACE@13..14 " "
                        NAME_REF@14..15
                          IDENT@14..15 "b"
                      WHITESPACE@15..16 " "
                      OR_KW@16..18 "or"
                      WHITESPACE@18..19 " "
                      NAME_REF@19..20
                        IDENT@19..20 "c"
        "#]],
    );
}

#[test]
fn recovers_from_errors() {
    let errs = errors(
        "if x then
  set_kind(
end
local y = )
z = 1",
        LuaVersion::Lua54,
    );
    assert_eq!(
        errs,
        [
            "21..21 expected an expression, found `end`",
            "35..35 expected an expression, found `)`",
            "36..37 expected a statement, found `)`",
        ]
    );
    // The statement after the errors is still parsed.
    let text = "local y = )
z = 1";
    let parse = parse(text, Dialect::XMAKE_DEFAULT);
    assert!(parse.root(text).descendants().any(|n| n.kind() == crate::SyntaxKind::ASSIGN_STMT));
}

#[test]
fn dialect_errors() {
    let text = "local a <const> = 1 // 2\ngoto done\n::done::\nlocal b = ~a & 1";
    assert_eq!(errors(text, LuaVersion::Lua54), Vec::<String>::new());
    assert_eq!(
        errors(text, LuaVersion::LuaJit),
        [
            "8..9 variable attributes `<const>` / `<close>` requires Lua 5.4 or newer (current: LuaJIT)",
            "20..22 integer division `//` requires Lua 5.3 or newer (current: LuaJIT)",
            "54..55 bitwise operators requires Lua 5.3 or newer (current: LuaJIT)",
            "57..58 bitwise operators requires Lua 5.3 or newer (current: LuaJIT)",
        ]
    );
    assert_eq!(
        errors("goto done\n::done::", LuaVersion::Lua51),
        [
            "0..4 `goto` and labels requires Lua 5.2 or newer (current: Lua 5.1)",
            "10..12 `goto` and labels requires Lua 5.2 or newer (current: Lua 5.1)",
        ]
    );
}

#[test]
fn global_declarations() {
    assert_eq!(
        errors("global x = 1\nglobal function f() end", LuaVersion::Lua55),
        Vec::<String>::new()
    );
    assert_eq!(
        errors("global x = 1", LuaVersion::Lua54),
        ["0..6 `global` declarations requires Lua 5.5 or newer (current: Lua 5.4)"]
    );
    // Still a plain name where it is not a declaration.
    assert_eq!(errors("global = 1\nprint(global)", LuaVersion::Lua54), Vec::<String>::new());
}

#[test]
fn stray_end() {
    assert_eq!(errors("end\nprint(1)", LuaVersion::Lua54), ["0..3 unexpected `end`"]);
}
