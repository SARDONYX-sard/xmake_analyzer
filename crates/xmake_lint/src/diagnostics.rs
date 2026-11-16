//! Diagnostics: syntax errors, dialect mismatches and misuses of the xmake
//! API.

use std::borrow::Cow;

use xmake_api::{ArgValues, Scope};
use xmake_syntax::{
    TextRange,
    ast::{AstNode, CallExpr, FieldExpr, Literal, MethodCallExpr, NameRef},
};

use crate::{
    AnalyzedFile, Project, context::CodeContext, outline::content_range, resolve::resolve,
};

/// How serious a diagnostic is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Error,
    Warning,
    Information,
    Hint,
}

/// A problem in a file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic<'a> {
    pub range: TextRange,
    pub severity: Severity,
    pub message: Cow<'a, str>,
    /// A stable identifier of the check.
    pub code: &'static str,
    /// Edits fixing the problem, offered as quick fixes, the best first.
    pub fixes: Vec<Fix<'a>>,
}

/// A replacement of `range` by `replacement`; an empty range inserts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fix<'a> {
    pub title: Cow<'a, str>,
    pub range: TextRange,
    pub replacement: Cow<'a, str>,
    /// Whether `lint --fix` applies it: the fix is certainly what is meant
    /// (not a guess such as a typo correction).
    pub safe: bool,
}

impl<'a> Fix<'a> {
    /// "Replace with `replacement`": a guess, not applied automatically.
    pub fn replace(range: TextRange, replacement: &'a str) -> Fix<'a> {
        let title = Cow::Owned(format!("Replace with `{replacement}`"));
        Fix { title, range, replacement: Cow::Borrowed(replacement), safe: false }
    }

    /// Marks the fix as safe to apply automatically.
    pub fn safe(mut self) -> Fix<'a> {
        self.safe = true;
        self
    }
}

/// Every diagnostic of `file`, at the `levels` of the lints.
pub fn diagnostics<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    levels: &crate::LintLevels,
) -> Vec<Diagnostic<'a>> {
    let mut out = Vec::new();
    for error in file.parse.errors() {
        let severity = Severity::Error;
        let message = Cow::Borrowed(error.message());
        out.push(Diagnostic {
            range: error.range(),
            severity,
            message,
            fixes: Vec::new(),
            code: "syntax",
        });
    }
    for error in &file.outline.errors {
        let message = Cow::Borrowed(&*error.message);
        let fix = error.replacement.map(|(range, end)| Fix::replace(range, end).safe());
        out.push(Diagnostic {
            range: error.range,
            severity: Severity::Warning,
            message,
            fixes: fix.into_iter().collect(),
            code: "mismatched-scope-end",
        });
    }
    check_xmakever(project, file, &mut out);

    let root = file.root();
    for node in root.descendants() {
        if let Some(call) = CallExpr::cast(node) {
            check_call(project, file, call, &mut out);
        } else if let Some(call) = MethodCallExpr::cast(node) {
            check_method_call(project, file, call, &mut out);
        }
    }
    check_tools(file, &mut out);
    crate::globals::undefined_globals(project, file, &mut out);
    crate::globals::unused_locals(file, &mut out);
    crate::relations::check(project, file, &mut out);
    crate::typecheck::check(project, file, &mut out);
    crate::annotations::check(project, file, &mut out);
    crate::lints::apply(levels, file, &mut out);
    out.sort_by_key(|d| (d.range.start(), d.severity));
    out
}

/// `set_xmakever` newer than the installed xmake.
fn check_xmakever(project: &Project, file: &AnalyzedFile, out: &mut Vec<Diagnostic<'_>>) {
    let (Some((required, range)), Some(installed)) = (file.outline.xmakever, project.xmake_version)
    else {
        return;
    };
    if installed >= required {
        return;
    }
    let message =
        format!("the project requires xmake {required}, but the installed xmake is {installed}");
    out.push(Diagnostic {
        range,
        severity: Severity::Warning,
        message: message.into(),
        fixes: Vec::new(),
        code: "xmake-version",
    });
}

fn check_call<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    call: CallExpr<'a>,
    out: &mut Vec<Diagnostic<'a>>,
) {
    let Some(callee) = call.callee() else {
        return;
    };
    if let Some(name_ref) = NameRef::cast(callee) {
        let Some(ident) = name_ref.ident() else {
            return;
        };
        if resolve(file, ident).is_some() {
            return;
        }
        let name = ident.text();
        let context = CodeContext::at(file, ident.text_range().start());
        if let CodeContext::Description(scope) = context {
            check_description_api(project, name, scope, ident.text_range(), out);
        }
        // `set_kind` of a rule, a package or a toolchain takes other kinds.
        let other_kinds = name == "set_kind"
            && !matches!(context, CodeContext::Description(Scope::Target | Scope::Root));
        if !other_kinds {
            check_arg_values(project, file, call, out);
        }
    } else if let Some(field) = FieldExpr::cast(callee) {
        check_module_function(project, file, field, out);
    }
}

/// A call in description code must be an API of the current scope.
fn check_description_api<'a>(
    project: &'a Project,
    name: &str,
    scope: Scope,
    range: TextRange,
    out: &mut Vec<Diagnostic<'a>>,
) {
    let db = &project.db;
    let known = db.visible_api(scope, name).is_some()
        || db.is_description_function(name)
        || Scope::from_opener(name).is_some();
    if known {
        return;
    }
    let owners: Vec<&str> = db.scopes_of(name).map(Scope::name).collect();
    if !owners.is_empty() {
        let message = format!(
            "`{name}` is not available in the {} scope, only in: {}",
            scope.name(),
            owners.join(", ")
        );
        out.push(Diagnostic {
            range,
            severity: Severity::Warning,
            message: message.into(),
            fixes: Vec::new(),
            code: "wrong-scope",
        });
        return;
    }
    let looks_like_api =
        ["set_", "add_", "del_", "remove_", "on_", "before_", "after_", "is_", "has_"]
            .iter()
            .any(|p| name.starts_with(p));
    if looks_like_api {
        let message = format!("unknown xmake API `{name}`");
        let visible =
            scope.visible_scopes().iter().flat_map(|&s| db.scope_apis(s)).map(|a| &*a.name);
        let candidates = visible.chain(db.description_functions().iter().map(|f| &**f));
        let fix = crate::fuzzy::closest(name, candidates).map(|c| Fix::replace(range, c));
        out.push(Diagnostic {
            range,
            severity: Severity::Warning,
            message: message.into(),
            fixes: fix.into_iter().collect(),
            code: "unknown-api",
        });
    }
}

/// String arguments outside of the accepted values.
fn check_arg_values<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    call: CallExpr<'a>,
    out: &mut Vec<Diagnostic<'a>>,
) {
    let Some(name) = call.callee_name() else {
        return;
    };
    let Some(args) = call.arg_list() else {
        return;
    };
    for (index, arg) in args.args().enumerate() {
        let Some(literal) = Literal::cast(arg) else {
            continue;
        };
        let (Some(token), Some(value)) = (literal.token(), literal.string_value()) else {
            continue;
        };
        let Some(values) = xmake_project::call_arg_values(file, call, index) else {
            continue;
        };
        let range = content_range(token);
        match values {
            ArgValues::Fixed(list) if !list.iter().any(|(v, _)| *v == value) => {
                let expected: Vec<&str> = list.iter().map(|(v, _)| *v).collect();
                let message = format!(
                    "unknown value `{value}` for `{name}`, expected one of: {}",
                    expected.join(", ")
                );
                let fix = crate::fuzzy::closest(&value, expected.iter().copied())
                    .map(|c| Fix::replace(range, c));
                out.push(Diagnostic {
                    range,
                    severity: Severity::Warning,
                    message: message.into(),
                    fixes: fix.into_iter().collect(),
                    code: "unknown-value",
                });
            }
            ArgValues::Targets => {
                let project_scope = project.scope_of(file);
                if project_scope.incomplete {
                    continue;
                }
                let mut targets = project_scope.definitions(Scope::Target).peekable();
                if targets.peek().is_none() {
                    continue;
                }
                // `ns::name` refers to `name` in namespace `ns`.
                let short = value.rsplit("::").next().unwrap_or(&value);
                if !targets.any(|t| t.name() == value || t.name() == short) {
                    let message = format!("unknown target `{value}`");
                    let names = project_scope.definitions(Scope::Target).map(|t| t.name());
                    let fix = crate::fuzzy::closest(&value, names).map(|c| Fix::replace(range, c));
                    out.push(Diagnostic {
                        range,
                        severity: Severity::Warning,
                        message: message.into(),
                        fixes: fix.into_iter().collect(),
                        code: "unknown-target",
                    });
                }
            }
            _ => {}
        }
    }
}

/// `os.cpp(...)`: a function the builtin module does not have.
fn check_module_function<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    field: FieldExpr<'a>,
    out: &mut Vec<Diagnostic<'a>>,
) {
    let (Some(base), Some(func)) = (field.base().and_then(NameRef::cast), field.field()) else {
        return;
    };
    let Some(base_ident) = base.ident() else {
        return;
    };
    if resolve(file, base_ident).is_some() {
        return;
    }
    let script = CodeContext::at(file, base_ident.text_range().start()).is_script();
    let Some(module) = project.db.module(base.text(), script) else {
        return;
    };
    if module.functions.iter().any(|f| f == func.text()) {
        return;
    }
    let message = format!("unknown function `{}.{}`", module.name, func.text());
    let range = func.text_range();
    let fix = crate::fuzzy::closest(func.text(), module.functions.iter().map(|f| &**f))
        .map(|c| Fix::replace(range, c));
    out.push(Diagnostic {
        range,
        severity: Severity::Warning,
        message: message.into(),
        fixes: fix.into_iter().collect(),
        code: "unknown-function",
    });
}

/// The tools of the flags: `"cl::/W4"` and `{tools = {"cl", "gcc"}}` name
/// tools of xmake; a flag for another one is never given.
fn check_tools<'a>(file: &'a AnalyzedFile, out: &mut Vec<Diagnostic<'a>>) {
    let mut check = |name: &'a str, range: TextRange| {
        if xmake_api::TOOLS.contains(&name) {
            return;
        }
        let fix = crate::fuzzy::closest(name, xmake_api::TOOLS.iter().copied())
            .map(|t| Fix::replace(range, t));
        out.push(Diagnostic {
            range,
            severity: Severity::Warning,
            message: Cow::Owned(format!("unknown tool `{name}`: the flags would never be used")),
            code: "unknown-value",
            fixes: fix.into_iter().collect(),
        });
    };
    // The name in a quoted string: `"cl"`, the `cl` of `"cl::/W4"`.
    let quoted = |node: xmake_syntax::SyntaxNode<'a>, prefix_only: bool| {
        let token = node.first_token().filter(|t| t.kind() == xmake_syntax::SyntaxKind::STRING)?;
        let content = token.text().get(1..token.text().len().saturating_sub(1))?;
        let name = if prefix_only { content.split_once("::")?.0 } else { content };
        let start = token.text_range().start() + xmake_syntax::TextSize::new(1);
        Some((name, TextRange::at(start, xmake_syntax::TextSize::new(name.len() as u32))))
    };
    for call in file.root().descendants().filter_map(CallExpr::cast) {
        let (Some(api), Some(args)) = (call.callee_name(), call.arg_list()) else {
            continue;
        };
        if !xmake_project::is_flags_api(api) {
            continue;
        }
        for arg in args.args() {
            if let Some((name, range)) = quoted(arg, true) {
                check(name, range);
            }
            let tools = crate::tables::field(arg, "tools").and_then(|f| f.value());
            let Some(tools) = tools else {
                continue;
            };
            let values: Vec<_> = if tools.kind() == xmake_syntax::SyntaxKind::TABLE_EXPR {
                crate::tables::fields(tools).into_iter().filter_map(|(_, f)| f.value()).collect()
            } else {
                vec![tools]
            };
            for value in values {
                if let Some((name, range)) = quoted(value, false) {
                    check(name, range);
                }
            }
        }
    }
}

/// `target:foo()` on a script instance without such a method.
fn check_method_call<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    call: MethodCallExpr<'a>,
    out: &mut Vec<Diagnostic<'a>>,
) {
    let (Some(receiver), Some(method)) = (call.receiver(), call.method()) else {
        return;
    };
    // Any receiver typed as an instance: `t`, `local d = t:dep("x")`, ...
    let Some(class) = crate::type_of(project, file, receiver).class() else {
        return;
    };
    let Some(instance) = xmake_api::Instance::parse(class) else {
        return;
    };
    let methods = project.db.instance_methods(instance);
    if methods.is_empty() || methods.iter().any(|m| m == method.text()) {
        return;
    }
    let message = format!("unknown method `{}` of {}", method.text(), instance.name());
    let range = method.text_range();
    let fix = crate::fuzzy::closest(method.text(), methods.iter().map(|m| &**m))
        .map(|c| Fix::replace(range, c));
    out.push(Diagnostic {
        range,
        severity: Severity::Warning,
        message: message.into(),
        fixes: fix.into_iter().collect(),
        code: "unknown-method",
    });
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, sync::Arc};

    use xmake_api::ApiDb;
    use xmake_syntax::{Dialect, LuaVersion};

    use super::*;

    fn check(text: &str, lua: LuaVersion) -> Vec<String> {
        let mut project = Project::new(ApiDb::builtin(), Dialect { lua });
        let file = project.set_file(PathBuf::from("xmake.lua"), Arc::from(text));
        let diags = diagnostics(&project, &file, &crate::LintLevels::default());
        diags.iter().map(|d| format!("{} {}", &text[d.range], d.message)).collect()
    }

    #[test]
    fn clean_project() {
        let text = r#"
set_project("demo")
add_rules("mode.debug", "mode.release")
add_requires("zlib")
target("lib")
    set_kind("static")
    -- Tests run in the directory of the crate: its sources exist.
    add_files("src/*.rs")
target("app")
    set_kind("binary")
    add_deps("lib")
    add_packages("zlib")
    on_build(function (target)
        import("core.base.json")
        print(target:name(), json.encode({}))
        os.cp("a", "b")
    end)
"#;
        assert_eq!(check(text, LuaVersion::Lua54), Vec::<String>::new());
    }

    #[test]
    fn api_misuse() {
        let text = r#"
option("o")
    set_kind("binary")
target("app")
    set_kind("bin")
    add_deps("nope")
    set_magic("x")
    on_load(function (target) target:frobnicate() target:dep("x"):targetfil() os.cpp() end)
"#;
        assert_eq!(
            check(text, LuaVersion::Lua54),
            [
                "set_kind `set_kind` is not available in the option scope, only in: target, rule, package, toolchain",
                "bin unknown value `bin` for `set_kind`, expected one of: binary, static, shared, object, headeronly, moduleonly, phony",
                "nope unknown target `nope`",
                "set_magic unknown xmake API `set_magic`",
                "frobnicate unknown method `frobnicate` of target",
                "targetfil unknown method `targetfil` of target",
                "cpp unknown function `os.cpp`",
            ]
        );
    }

    #[test]
    fn names() {
        let text = "local used = 1
print(used)
local unused = 2
foo()
-- xmake-analyzer: allow(undefined-global)
bar()
target('a')
  on_build(function (target)
    import('core.base.json')
    add_files('x')
    prnit(json)
  end)
";
        assert_eq!(
            check(text, LuaVersion::Lua54),
            [
                "unused unused local `unused`",
                "foo undefined global `foo`",
                "add_files `add_files` is only available in description code",
                "prnit undefined global `prnit`",
            ]
        );
    }

    #[test]
    fn flag_tools() {
        let text = "target('a')
  add_cxxflags('/W4', {tools = 'clx'})
  add_cxxflags('-Wall', {tools = {'gcc', 'clangg'}})
  add_cxxflags('cll::/W4', 'gcc::-Wall', {tools = 'cl'})
";
        assert_eq!(
            check(text, LuaVersion::Lua54),
            [
                "clx unknown tool `clx`: the flags would never be used",
                "clangg unknown tool `clangg`: the flags would never be used",
                "cll unknown tool `cll`: the flags would never be used",
            ]
        );
    }

    #[test]
    fn levels() {
        let mut project = Project::new(ApiDb::builtin(), Dialect::XMAKE_DEFAULT);
        let mut levels = crate::LintLevels::default();
        levels.set("unused-local", crate::Level::Deny);
        let file = project.set_file(
            PathBuf::from("xmake.lua"),
            Arc::from(
                "local x = 1
",
            ),
        );
        let diags = diagnostics(&project, &file, &levels);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Error);
        assert!(!diags[0].fixes.is_empty());
        levels.set("unused-local", crate::Level::Allow);
        let file = project.set_file(
            PathBuf::from("xmake.lua"),
            Arc::from(
                "local x = 1
",
            ),
        );
        assert!(diagnostics(&project, &file, &levels).is_empty());
    }

    #[test]
    fn configs() {
        let text = "option('ssl')
option_end()
if has_config('ssl', 'plat') and is_config('mode', 'debug') and get_config('sll') then end
";
        assert_eq!(check(text, LuaVersion::Lua54), ["sll unknown option `sll`"]);
    }

    #[test]
    fn locals_shadow_apis() {
        let text = "local function set_magic() end\nset_magic()\n";
        assert_eq!(check(text, LuaVersion::Lua54), Vec::<String>::new());
    }

    #[test]
    fn dialect() {
        let errs = check("local _x = 1 // 2\n", LuaVersion::LuaJit);
        assert_eq!(errs, ["// integer division `//` requires Lua 5.3 or newer (current: LuaJIT)"]);
    }
}
