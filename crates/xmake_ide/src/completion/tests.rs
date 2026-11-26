use std::{path::PathBuf, sync::Arc};

use xmake_api::ApiDb;
use xmake_syntax::Dialect;

use super::*;

/// Labels offered at `$` in an `xmake.lua` containing `text`.
fn complete(text: &str) -> Vec<String> {
    complete_in("xmake.lua", text)
}

fn complete_in(file_name: &str, text: &str) -> Vec<String> {
    let offset = text.find('$').expect("marker");
    let text = text.replacen('$', "", 1);
    let mut project = Project::new(ApiDb::builtin(), Dialect::XMAKE_DEFAULT);
    let file = project.set_file(PathBuf::from(file_name), Arc::from(text.as_str()));
    let items = completions(&project, &file, None, TextSize::new(offset as u32));
    items.into_iter().map(|i| i.label.into_owned()).collect()
}

fn has(labels: &[String], label: &str) -> bool {
    labels.iter().any(|l| l == label)
}

#[test]
fn scope_apis() {
    let target = complete("target('a')\n    set_$");
    assert!(has(&target, "set_kind"), "{target:?}");
    assert!(has(&target, "set_languages"));
    assert!(!has(&target, "add_files"), "filtered by prefix");

    let option = complete("option('o')\n    set_$");
    assert!(!has(&option, "set_kind"), "set_kind is a target API");
    assert!(has(&option, "set_default"));

    let root = complete("set_$");
    assert!(has(&root, "set_project"));
    assert!(has(&root, "set_kind"), "target APIs apply to every target at the root");
    let rule = complete("rule('r')\n    add_$");
    assert!(!has(&rule, "add_files"), "{rule:?}");
}

#[test]
fn scope_end() {
    let labels = complete("target('a')\n    target_$");
    assert!(has(&labels, "target_end"), "{labels:?}");
}

#[test]
fn script_context() {
    let labels = complete("target('a')\n    on_build(function (target)\n        o$\n    end)");
    assert!(has(&labels, "os"), "{labels:?}");
    assert!(!has(&labels, "on_build"), "description APIs are not available in scripts");
}

#[test]
fn instance_methods() {
    let labels = complete("target('a')\n    on_build(function (t)\n        t:$\n    end)");
    assert!(has(&labels, "targetfile"), "{labels:?}");
    assert!(has(&labels, "name"));

    let package =
        complete("package('p')\n    on_install(function (package)\n        package:$\n    end)");
    assert!(has(&package, "installdir"), "{package:?}");
}

#[test]
fn typed_members() {
    let dep = complete(
        "target('a')
on_load(function (t)
  local d = t:dep('core')
  d:$
end)",
    );
    assert!(has(&dep, "targetfile"), "{dep:?}");
    let pkg = complete(
        "target('a')
on_load(function (t)
  local p = t:pkg('zlib')
  p:$
end)",
    );
    assert!(has(&pkg, "installdir"), "{pkg:?}");
    let string = complete(
        "target('a')
on_load(function (t)
  local n = t:name()
  n:$
end)",
    );
    assert!(has(&string, "split") && has(&string, "upper"), "{string:?}");
    let json = complete_in(
        "build.lua",
        "function main()
  import('core.base.json')
  json.$
end",
    );
    assert!(has(&json, "loadfile"), "{json:?}");
}

#[test]
fn module_members() {
    let labels = complete("target('a')\non_load(function (t) os.$ end)");
    assert!(has(&labels, "cp"), "{labels:?}");
    let imported =
        complete_in("build.lua", "function main()\n    import('core.base.json')\n    json.$\nend");
    assert!(has(&imported, "decode"), "{imported:?}");
    let concat = complete("local s = 'a'..$");
    assert!(!has(&concat, "cp"));
}

#[test]
fn string_values() {
    let kinds = complete("target('a')\n    set_kind('$')");
    assert!(has(&kinds, "binary"), "{kinds:?}");
    let partial = complete("target('a')\n    set_kind(\"st$");
    assert_eq!(partial, ["static"]);
    let rules = complete("add_rules('mode.$')");
    assert!(has(&rules, "mode.debug"), "{rules:?}");
    let deps = complete("target('core')\ntarget('app')\n    add_deps('$')");
    assert!(has(&deps, "core"), "{deps:?}");
    let imports = complete_in("build.lua", "import('core.base.$')");
    assert!(has(&imports, "core.base.json"), "{imports:?}");
}

#[test]
fn options_and_configs() {
    let text = "option('skyrim_se', function()
    set_default(true)
end)
if has_config('$') then end";
    let labels = complete(text);
    assert!(has(&labels, "skyrim_se") && has(&labels, "plat"), "{labels:?}");
    let get = complete(
        "option('rex_ini')
option_end()
local v = get_config('re$')",
    );
    assert_eq!(get.first().map(String::as_str), Some("rex_ini"));
    let rule_deps = complete(
        "rule('base')
rule('plugin', function()
    add_deps('$')
end)",
    );
    assert!(has(&rule_deps, "base") && has(&rule_deps, "mode.debug"), "{rule_deps:?}");
}

#[test]
fn api_option_keys() {
    let tests = complete(
        "local x = 1
target('a')
    add_tests('t', {$})",
    );
    assert!(has(&tests, "runargs") && has(&tests, "pass_outputs"), "{tests:?}");
    assert!(!has(&tests, "x") && !has(&tests, "set_kind"), "only keys: {tests:?}");
    let files = complete(
        "target('a')
    add_files('a.c', {def$})",
    );
    assert_eq!(files, ["defines"]);
    let dirs = complete(
        "target('a')
    add_includedirs('inc', {pu$})",
    );
    assert_eq!(dirs, ["public"]);
    // A plain table: values may be names.
    let plain = complete(
        "local value = 1
local t = {va$}",
    );
    assert!(has(&plain, "value"), "{plain:?}");
}

#[test]
fn names_of_the_project_only() {
    let mut project = Project::new(ApiDb::builtin(), Dialect::XMAKE_DEFAULT);
    project.set_file(
        PathBuf::from("lib/xmake.lua"),
        Arc::from(
            "option('lib_opt')
option_end()
",
        ),
    );
    project.set_file(
        PathBuf::from("other/xmake.lua"),
        Arc::from(
            "option('other_opt')
option_end()
",
        ),
    );
    let text = "includes('lib')
if has_config('') then end
";
    let file = project.set_file(PathBuf::from("xmake.lua"), Arc::from(text));
    let offset = TextSize::new(text.find("'')").expect("quote") as u32 + 1);
    let labels: Vec<_> = completions(&project, &file, None, offset)
        .into_iter()
        .map(|i| i.label.into_owned())
        .collect();
    assert!(has(&labels, "lib_opt"), "{labels:?}");
    assert!(!has(&labels, "other_opt"), "another project: {labels:?}");
}

#[test]
fn locals_first() {
    let labels = complete("local my_value = 1\nprint(my_$)");
    assert_eq!(labels.first().map(String::as_str), Some("my_value"));
}

#[test]
fn nothing_in_comments() {
    assert!(complete("-- set_$").is_empty());
}

#[test]
fn annotations() {
    let tags = complete("---@pa$\nlocal function f(a) end");
    assert!(has(&tags, "param"), "{tags:?}");
    assert!(!has(&tags, "return"), "filtered by prefix");

    let params = complete("---@param $\nlocal function f(path, opt, ...) end");
    assert_eq!(params, ["path", "opt", "..."]);

    let types = complete("---@param path st$\nlocal function f(path) end");
    assert!(has(&types, "string"), "{types:?}");
    let types = complete("---@class Point\n---@return string|tar$\nfunction f() end");
    assert!(has(&types, "target"), "classes of the stubs: {types:?}");
    let types = complete("---@class Point\n---@type P$\nlocal p = {}");
    assert!(has(&types, "Point"), "classes of the file: {types:?}");
    assert!(!complete("---@type $").iter().any(|l| l.starts_with("xmake.")));

    assert!(complete("-- plain comment $").is_empty());
    assert!(complete("---@class Name$").is_empty(), "a new name");
}

#[test]
fn nothing_but_values_in_strings_and_comments() {
    let functions = "local function helper() end\n";
    // A string without known values, closed or not.
    assert!(complete(&format!("{functions}print('he$')")).is_empty());
    assert!(complete(&format!("{functions}print('he$")).is_empty());
    assert!(complete(&format!("{functions}local s = [[he$]]")).is_empty());
    // Comments, annotations at a name.
    assert!(complete(&format!("{functions}-- he$")).is_empty());
    assert!(complete(&format!("{functions}---@class He$")).is_empty());
    assert!(complete(&format!("{functions}---@see he$")).is_empty());
    let tags = complete(&format!("{functions}---@$"));
    assert!(tags.iter().all(|l| l != "helper"), "{tags:?}");
}

#[test]
fn string_literal_types() {
    let declared = "---@alias Std \"c++20\"|\"c++23\"\n---@param std Std\n---@param mode? \"debug\"|\"release\"\nlocal function use(std, mode) end\n";
    assert_eq!(complete(&format!("{declared}use('c$')")), ["c++20", "c++23"]);
    assert_eq!(complete(&format!("{declared}use('c++20', '$')")), ["debug", "release"]);
    let kinds =
        complete("target('a')\non_load(function (t)\n  if t:kind() == 'sh$' then end\nend)");
    assert_eq!(kinds, ["shared"]);
    let modes = complete("target('a')\non_load(function (t)\n  io.open('f', 'w$')\nend)");
    assert!(modes.iter().any(|m| m == "wb"), "{modes:?}");
}

#[test]
fn flag_tools() {
    let tools = complete("target('a')\n  add_cxxflags('/W4', {tools = 'c$'})");
    assert!(has(&tools, "cl") && has(&tools, "clang"), "{tools:?}");
    let list = complete("target('a')\n  add_cxxflags('/W4', {tools = {'gcc', 'cla$'}})");
    assert!(has(&list, "clang") && has(&list, "clangxx"), "{list:?}");
    // `"cl::/W4"`: the tool before the flag.
    let prefixes = complete("target('a')\n  add_cxxflags('cl$')");
    assert!(has(&prefixes, "cl::") && has(&prefixes, "clang_cl::"), "{prefixes:?}");
    assert!(complete("target('a')\n  add_cxxflags('/W$')").is_empty());
    assert!(complete("target('a')\n  add_cxxflags('cl::/W$')").is_empty());
}

#[test]
fn module_functions_of_the_scope() {
    // Scripts have all of `os`; the description only some.
    let script = complete(
        "target('a')
 on_load(function (t)
 os.io$
 end)
",
    );
    assert!(has(&script, "iorunv"), "{script:?}");
    let description = complete(
        "os.io$
",
    );
    assert!(!has(&description, "iorunv"), "{description:?}");
    assert!(has(
        &complete(
            "os.get$
"
        ),
        "getenv"
    ));
    assert!(has(
        &complete(
            "table.jo$
"
        ),
        "join2"
    ));
}

#[test]
fn members_of_imported_modules() {
    // The call itself, a local bound to it, the name `import` binds.
    for text in [
        "target('a')
 on_install(function (t)
 import('package.tools.cmake').$
 end)
",
        "target('a')
 on_install(function (t)
 import('package.tools.cmake')
 cmake.$
 end)
",
        "target('a')
 on_install(function (t)
 local c = import('package.tools.cmake')
 c.$
 end)
",
    ] {
        let labels = complete(text);
        assert!(has(&labels, "install"), "{text}: {labels:?}");
    }
}
