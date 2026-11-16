use expect_test::{Expect, expect};
use xmake_syntax::Dialect;

use crate::{FormatError, FormatOptions, Indent, format};

fn check_with(input: &str, options: FormatOptions, expect: Expect) {
    let output = format(input, Dialect::XMAKE_DEFAULT, options).expect("formats");
    expect.assert_eq(&output);
    let again = format(&output, Dialect::XMAKE_DEFAULT, options).expect("formats again");
    assert_eq!(again, output, "formatting is not idempotent");
}

fn check(input: &str, expect: Expect) {
    check_with(input, FormatOptions::default(), expect);
}

#[test]
fn description_file() {
    check(
        r#"add_rules("mode.debug","mode.release")


target("app")
set_kind( "binary" )
add_files("src/*.c")   -- sources
if is_plat("windows") then
add_links("user32")
end
on_build(function(target)
print(target:name())
end)
target_end()
"#,
        expect![[r#"
            add_rules("mode.debug", "mode.release")

            target("app")
                set_kind("binary")
                add_files("src/*.c") -- sources
                if is_plat("windows") then
                    add_links("user32")
                end
                on_build(function (target)
                    print(target:name())
                end)
            target_end()
        "#]],
    );
}

#[test]
fn tables_and_operators() {
    check(
        "local t={a=1,b={c=2},[\"k\"]=3;4}\nlocal x=-a+b*2..'s'..#t\nlocal y = not x and y or z\nadd_requires(\"zlib\",{\nconfigs={shared=true},\n})\n",
        expect![[r#"
            local t = {a = 1, b = {c = 2}, ["k"] = 3; 4}
            local x = -a + b * 2 .. 's' .. #t
            local y = not x and y or z
            add_requires("zlib", {
                configs = {shared = true},
            })
        "#]],
    );
}

#[test]
fn comments_before_closers() {
    check(
        "function f()\nx()\n-- trailing comment\nend\n--[[ block\n   kept ]]\n",
        expect![[r#"
            function f()
                x()
                -- trailing comment
            end
            --[[ block
               kept ]]
        "#]],
    );
}

#[test]
fn tabs_per_level() {
    let options = FormatOptions { indent: Indent::Tabs(2), ..FormatOptions::default() };
    check_with("if x then\ny()\nend\n", options, expect!["if x then\n\t\ty()\nend\n"]);
}

#[test]
fn never_glues_tokens() {
    check(
        "local a = - -b\nlocal c = 1 .. 2\n",
        expect![[r#"
        local a = - -b
        local c = 1 .. 2
    "#]],
    );
}

#[test]
fn keeps_crlf() {
    let out = format("x = 1\r\ny = 2\r\n", Dialect::XMAKE_DEFAULT, FormatOptions::default())
        .expect("formats");
    assert_eq!(out, "x = 1\r\ny = 2\r\n");
}

#[test]
fn refuses_broken_files() {
    let err = format("if x then", Dialect::XMAKE_DEFAULT, FormatOptions::default());
    assert_eq!(err, Err(FormatError::SyntaxErrors));
}
