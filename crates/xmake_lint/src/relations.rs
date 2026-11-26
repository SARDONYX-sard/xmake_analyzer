//! Lints on the relations between the items of the project:
//! `add_requires` and `add_packages`, `add_deps` between targets, the names
//! given to `add_rules`, `set_toolchains`, `add_options`, ...

use std::borrow::Cow;

use rustc_hash::{FxHashMap, FxHashSet};
use xmake_api::Scope;
use xmake_syntax::{
    SyntaxKind, TextRange, TextSize,
    ast::{AstNode, CallExpr, Literal},
};

use crate::{
    AnalyzedFile, Diagnostic, Fix, Project, Severity, fuzzy,
    outline::{Definition, content_range},
};

/// A string argument of a call: its content range and text.
#[derive(Clone, Copy)]
struct StringArg<'a> {
    range: TextRange,
    value: &'a str,
    call: TextRange,
    /// The position among the arguments of the call.
    index: usize,
}

/// The string arguments of the calls to any of `apis` in `file`.
fn string_args<'a>(
    file: &'a AnalyzedFile,
    apis: &'a [&'a str],
) -> impl Iterator<Item = StringArg<'a>> + 'a {
    file.root()
        .descendants()
        .filter_map(CallExpr::cast)
        .filter(move |c| c.callee_name().is_some_and(|n| apis.contains(&n)))
        .flat_map(move |call| {
            let args = call.arg_list().into_iter().flat_map(|a| a.args()).enumerate();
            args.filter_map(|(index, arg)| Some((index, Literal::cast(arg)?.token()?)))
                .filter(|(_, t)| t.kind().is_string())
                .map(move |(index, token)| {
                    let range = content_range(token);
                    let call = call.syntax().text_range();
                    StringArg { range, value: &file.text[range], call, index }
                })
        })
}

fn warning<'a>(
    range: TextRange,
    code: &'static str,
    message: String,
    fix: Option<Fix<'a>>,
) -> Diagnostic<'a> {
    Diagnostic {
        range,
        severity: Severity::Warning,
        message: Cow::Owned(message),
        code,
        fixes: fix.into_iter().collect(),
    }
}

/// Every relation lint of `file`.
pub(crate) fn check<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    out: &mut Vec<Diagnostic<'a>>,
) {
    includes(project, file, out);
    unmatched_paths(project, file, out);
    repositories(project, file, out);
    dependencies(project, file, out);
    duplicates(project, file, out);
    // Names may come from files the project cannot load.
    if !project.scope_of(file).incomplete {
        packages(project, file, out);
        names(project, file, out);
    }
    deprecated(file, out);
}

/// `unresolved-include`: `includes("dir")` loading nothing.
fn includes<'a>(project: &'a Project, file: &'a AnalyzedFile, out: &mut Vec<Diagnostic<'a>>) {
    let Some(dir) = project.path_of(file).and_then(|p| p.parent()) else {
        return;
    };
    for &range in &file.outline.includes {
        let value = &file.text[range];
        let relative =
            value.strip_prefix("$(scriptdir)").map_or(value, |v| v.trim_start_matches(['/', '\\']));
        if relative.starts_with('@')
            || relative.contains('*')
            || !project.included_paths(dir, relative).is_empty()
        {
            continue;
        }
        let message =
            format!("`{value}` loads nothing: no such file, nor a directory with an `xmake.lua`");
        out.push(warning(range, "unresolved-include", message, None));
    }
}

/// What the path arguments of `name` name, if they must exist.
fn path_kind(name: &str) -> Option<crate::paths::PathKind> {
    use crate::paths::PathKind::{Dir, Files};
    let kind = match name {
        "add_files" | "remove_files" | "add_headerfiles" | "remove_headerfiles"
        | "add_installfiles" | "add_configfiles" | "add_extrafiles" | "set_pcheader"
        | "set_pcxxheader" | "set_pmheader" | "set_pmxxheader" => Files,
        "add_includedirs" | "add_sysincludedirs" | "add_linkdirs" | "add_frameworkdirs"
        | "add_moduledirs" | "add_plugindirs" | "add_packagedirs" | "add_toolchaindirs"
        | "add_platformdirs" => Dir,
        // Output directories (`set_targetdir`, ...) are created by the build.
        _ => return None,
    };
    Some(kind)
}

/// `unmatched-path`: files and directories that do not exist, globs
/// matching nothing.
fn unmatched_paths<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    out: &mut Vec<Diagnostic<'a>>,
) {
    let Some(dir) = project.path_of(file).and_then(|p| p.parent()) else {
        return;
    };
    for call in file.root().descendants().filter_map(CallExpr::cast) {
        let Some(kind) = call.callee_name().and_then(path_kind) else {
            continue;
        };
        // In scripts, the current directory may differ.
        if crate::context::CodeContext::at(file, call.syntax().text_range().start()).is_script() {
            continue;
        }
        let args = call.arg_list().into_iter().flat_map(|a| a.args());
        for token in args
            .filter_map(Literal::cast)
            .filter_map(|l| l.token())
            .filter(|t| t.kind().is_string())
        {
            let range = content_range(token);
            let value = &file.text[range];
            if crate::paths::check_path(dir, value, kind) != Some(false) {
                continue;
            }
            let what = match kind {
                crate::paths::PathKind::Dir => "no such directory",
                crate::paths::PathKind::Files if value.contains('*') => {
                    "the pattern matches no file"
                }
                crate::paths::PathKind::Files => "no such file",
            };
            let message = format!("`{value}`: {what}");
            let fix = similar_name(dir, value, range);
            out.push(warning(range, "unmatched-path", message, fix));
        }
    }
}

/// A typo in the last part of a path: the closest entry of its directory.
fn similar_name<'a>(dir: &std::path::Path, value: &str, range: TextRange) -> Option<Fix<'a>> {
    if value.contains(['*', '$', '|', '(']) {
        return None;
    }
    let value = value.trim_end_matches('/');
    let (parent, name) = value.rsplit_once('/').unwrap_or(("", value));
    let entries = std::fs::read_dir(dir.join(parent)).ok()?;
    let names: Vec<String> =
        entries.flatten().filter_map(|e| e.file_name().into_string().ok()).collect();
    let close = fuzzy::closest(name, names.iter().map(String::as_str))?;
    let start = range.start() + TextSize::new((value.len() - name.len()) as u32);
    let name_range = TextRange::at(start, TextSize::new(name.len() as u32));
    Some(Fix {
        title: Cow::Owned(format!("Replace with `{close}`")),
        range: name_range,
        replacement: Cow::Owned(close.to_owned()),
        safe: false,
    })
}

/// `unused-require` and `missing-require`.
fn packages<'a>(project: &'a Project, file: &'a AnalyzedFile, out: &mut Vec<Diagnostic<'a>>) {
    let mut provided: FxHashSet<&str> = FxHashSet::default();
    let mut used: FxHashSet<&str> = FxHashSet::default();
    for (_, other) in project.scope_of(file).files() {
        let aliases = other.outline.require_aliases.iter().flatten();
        provided.extend(other.outline.requires.iter().chain(aliases).map(|&r| &other.text[r]));
        used.extend(string_args(other, &["add_packages"]).map(|a| a.value));
    }
    // Without any `add_packages`, the project may use the packages through
    // scripts (`target:pkg`, `find_package`): stay quiet.
    if !used.is_empty() {
        let aliases = &file.outline.require_aliases;
        for (&range, alias) in file.outline.requires.iter().zip(aliases) {
            let name = &file.text[range];
            let aliased = alias.is_some_and(|a| used.contains(&file.text[a]));
            if !used.contains(name) && !aliased {
                let message = format!(
                    "package `{name}` is required but no target uses it with `add_packages`"
                );
                let fix = crate::fixes::remove_argument(file, range, format!("Remove `{name}`"));
                out.push(warning(range, "unused-require", message, fix));
            }
        }
    }
    for arg in string_args(file, &["add_packages"]) {
        if !provided.contains(arg.value) {
            let message = format!(
                "package `{}` is not required: add `add_requires(\"{}\")`",
                arg.value, arg.value
            );
            let fix = fuzzy::closest(arg.value, provided.iter().copied())
                .map(|c| Fix::replace(arg.range, c));
            let mut diagnostic = warning(arg.range, "missing-require", message, fix);
            diagnostic.fixes.extend(crate::fixes::insert_require(file, arg.value));
            out.push(diagnostic);
        }
    }
}

/// `unknown-package` and `unknown-package-version`, against the package
/// repositories.
fn repositories<'a>(project: &'a Project, file: &'a AnalyzedFile, out: &mut Vec<Diagnostic<'a>>) {
    let index = &project.packages;
    if index.is_empty() {
        return;
    }
    let defined: Vec<&str> =
        project.scope_of(file).definitions(Scope::Package).map(|d| d.name()).collect();
    for arg in string_args(file, &["add_requires"]) {
        let (name, version) = crate::packages::split_requirement(arg.value);
        // `conan::zlib`, `vcpkg::...`: other package managers.
        if name.is_empty() || name.contains("::") || defined.contains(&name) {
            continue;
        }
        let name_range = TextRange::at(
            arg.range.start() + offset_in(arg.value, name),
            (name.len() as u32).into(),
        );
        let Some(entry) = index.get(name) else {
            let message = format!("unknown package `{name}`: no repository has it");
            let names = index.entries().iter().map(|e| e.name.as_str());
            let fix = fuzzy::closest(name, names).map(|c| Fix::replace(name_range, c));
            out.push(warning(name_range, "unknown-package", message, fix));
            continue;
        };
        if !crate::packages::is_exact_version(version) {
            continue;
        }
        let Some(details) = entry.details() else {
            continue;
        };
        if details.versions.is_empty() || crate::packages::has_version(&details.versions, version) {
            continue;
        }
        let range = TextRange::at(
            arg.range.start() + offset_in(arg.value, version),
            (version.len() as u32).into(),
        );
        let latest = details
            .versions
            .iter()
            .max_by(|a, b| crate::compare_versions(a, b))
            .map_or("", String::as_str);
        let message = format!("package `{name}` has no version `{version}` (latest: `{latest}`)");
        let mut diagnostic = warning(range, "unknown-package-version", message, None);
        if let Some(close) = nearest_version(&details.versions, version) {
            diagnostic.fixes.push(Fix {
                title: Cow::Owned(format!("Replace with `{close}`")),
                range,
                replacement: Cow::Owned(close.to_owned()),
                safe: false,
            });
        }
        out.push(diagnostic);
    }
}

/// The version sharing the most leading numbers with `wanted`, the newest on
/// ties: `v1.3` for `1.4` among `v1.2.13` and `v1.3`.
fn nearest_version<'v>(versions: &'v [String], wanted: &str) -> Option<&'v str> {
    let numbers = |v: &str| -> Vec<u64> {
        v.trim_start_matches('v').split('.').map(|p| p.parse().unwrap_or(0)).collect()
    };
    let wanted = numbers(wanted);
    let shared = |v: &str| numbers(v).iter().zip(&wanted).take_while(|(a, b)| a == b).count();
    versions
        .iter()
        .max_by(|a, b| shared(a).cmp(&shared(b)).then_with(|| crate::compare_versions(a, b)))
        .map(String::as_str)
}

/// The byte offset of the subslice `part` in `whole`.
fn offset_in(whole: &str, part: &str) -> TextSize {
    TextSize::new((part.as_ptr() as usize).saturating_sub(whole.as_ptr() as usize) as u32)
}

/// The target whose scope contains `range`.
fn owner(file: &AnalyzedFile, range: TextRange) -> Option<&Definition> {
    file.outline
        .definitions
        .iter()
        .filter(|d| d.scope == Scope::Target)
        .find(|d| d.full_range.contains_range(range))
}

/// `self-dependency` and `dependency-cycle`.
fn dependencies<'a>(project: &'a Project, file: &'a AnalyzedFile, out: &mut Vec<Diagnostic<'a>>) {
    // The graph of the whole project: target -> its dependencies.
    let mut graph: FxHashMap<&str, Vec<&str>> = FxHashMap::default();
    for (_, other) in project.scope_of(file).files() {
        for arg in string_args(other, &["add_deps"]) {
            if let Some(def) = owner(other, arg.call) {
                graph.entry(&other.text[def.name_range]).or_default().push(arg.value);
            }
        }
    }
    for arg in string_args(file, &["add_deps"]) {
        let Some(def) = owner(file, arg.call) else {
            continue;
        };
        let target = &file.text[def.name_range];
        if arg.value == target {
            let message = format!("target `{target}` depends on itself");
            out.push(Diagnostic {
                range: arg.range,
                severity: Severity::Error,
                message: Cow::Owned(message),
                code: "self-dependency",
                fixes: crate::fixes::remove_argument(file, arg.range, format!("Remove `{target}`"))
                    .into_iter()
                    .collect(),
            });
        } else if reaches(&graph, arg.value, target) {
            let message = format!("dependency cycle: `{}` depends on `{target}` again", arg.value);
            out.push(warning(arg.range, "dependency-cycle", message, None));
        }
    }
}

/// Whether `to` is reachable from `from` through `graph`.
fn reaches(graph: &FxHashMap<&str, Vec<&str>>, from: &str, to: &str) -> bool {
    let mut seen: FxHashSet<&str> = FxHashSet::default();
    let mut stack = vec![from];
    while let Some(node) = stack.pop() {
        if node == to {
            return true;
        }
        if seen.insert(node) {
            stack.extend(graph.get(node).into_iter().flatten().copied());
        }
    }
    false
}

/// `duplicate-definition`.
fn duplicates<'a>(project: &'a Project, file: &'a AnalyzedFile, out: &mut Vec<Diagnostic<'a>>) {
    let project_scope = project.scope_of(file);
    for def in &file.outline.definitions {
        let name = &file.text[def.name_range];
        let other = project_scope.definitions(def.scope).find(|o| {
            o.name() == name && !(std::ptr::eq(o.file, file) && o.item.name_range == def.name_range)
        });
        if let Some(other) = other {
            let line = other
                .file
                .line_index
                .line_col(&other.file.text, other.item.name_range.start())
                .line
                + 1;
            let message = format!(
                "{} `{name}` is also defined in {}:{line}",
                def.scope.name(),
                other.path.display()
            );
            out.push(warning(def.name_range, "duplicate-definition", message, None));
        }
    }
}

/// `unknown-rule`, `unknown-toolchain` and `unknown-option`.
fn names<'a>(project: &'a Project, file: &'a AnalyzedFile, out: &mut Vec<Diagnostic<'a>>) {
    let project_scope = project.scope_of(file);
    let known = &project.db.values;
    let defined = |scope: Scope| -> Vec<&'a str> {
        project_scope.definitions(scope).map(|d| d.name()).collect()
    };
    let rules = defined(Scope::Rule);
    let toolchains = defined(Scope::Toolchain);
    let options = defined(Scope::Option);

    // `has_config` and friends also read the configurations of `xmake f`.
    let configs: Vec<&str> = options
        .iter()
        .copied()
        .chain(xmake_api::builtin_configs().iter().map(|(n, _)| *n))
        .collect();
    let checks: [(&[&str], &'static str, &str, Vec<&str>); 4] = [
        (
            &["add_rules", "set_rules"],
            "unknown-rule",
            "rule",
            known.rules.iter().map(|r| &**r).chain(rules.iter().copied()).collect(),
        ),
        (
            &["set_toolchains", "add_toolchains"],
            "unknown-toolchain",
            "toolchain",
            known.toolchains.iter().map(|t| &*t.name).chain(toolchains.iter().copied()).collect(),
        ),
        (&["add_options", "set_options"], "unknown-option", "option", options.clone()),
        (&["has_config", "get_config", "is_config"], "unknown-option", "option", configs),
    ];
    for (apis, code, what, candidates) in checks {
        // `@pkg/rule` and `@pkg` come from packages; namespaced names too.
        // `is_config("name", "value")`: only the name is a configuration.
        let first_only = apis.contains(&"is_config");
        let args = string_args(file, apis).filter(|a| !first_only || a.index == 0);
        for arg in args.filter(|a| !a.value.starts_with('@')) {
            let short = arg.value.rsplit("::").next().unwrap_or(arg.value);
            if candidates.contains(&arg.value) || candidates.contains(&short) {
                continue;
            }
            let message = format!("unknown {what} `{}`", arg.value);
            let fix = fuzzy::closest(arg.value, candidates.iter().copied())
                .map(|c| Fix::replace(arg.range, c));
            out.push(warning(arg.range, code, message, fix));
        }
    }
}

/// `deprecated-api`.
fn deprecated<'a>(file: &'a AnalyzedFile, out: &mut Vec<Diagnostic<'a>>) {
    const REPLACED: &[(&str, &str)] = &[("add_subdirs", "includes"), ("add_subfiles", "includes")];
    for call in file.root().descendants().filter_map(CallExpr::cast) {
        let Some(callee) = call.callee().filter(|c| c.kind() == SyntaxKind::NAME_REF) else {
            continue;
        };
        let name = callee.text().trim();
        if let Some((_, replacement)) = REPLACED.iter().find(|(old, _)| *old == name) {
            let range = callee.trimmed_range();
            let message = format!("`{name}` is deprecated, use `{replacement}`");
            out.push(warning(
                range,
                "deprecated-api",
                message,
                Some(Fix::replace(range, replacement).safe()),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, sync::Arc};

    use xmake_api::ApiDb;
    use xmake_syntax::Dialect;

    use super::*;

    fn lint(files: &[(&str, &str)], path: &str) -> Vec<String> {
        let mut project = Project::new(ApiDb::builtin(), Dialect::XMAKE_DEFAULT);
        for (p, text) in files {
            project.set_file(PathBuf::from(p), Arc::from(*text));
        }
        let file = project.file(std::path::Path::new(path)).expect("file");
        let mut out = Vec::new();
        check(&project, file, &mut out);
        out.iter().map(|d| format!("{}: {}", d.code, d.message)).collect()
    }

    #[test]
    fn packages() {
        let text = "add_requires('zlib', 'fmt 10.x')
add_requires('conan::boost/1.84', {alias = 'boost'})\ntarget('app')\n  add_packages('zlib', 'boost', 'openssl')\n";
        assert_eq!(
            lint(&[("xmake.lua", text)], "xmake.lua"),
            [
                "unused-require: package `fmt` is required but no target uses it with `add_packages`",
                "missing-require: package `openssl` is not required: add `add_requires(\"openssl\")`",
            ]
        );
    }

    #[test]
    fn dependencies() {
        let text = "target('a')\n  add_deps('b')\ntarget('b')\n  add_deps('a')\ntarget('c')\n  add_deps('c')\n";
        assert_eq!(
            lint(&[("xmake.lua", text)], "xmake.lua"),
            [
                "dependency-cycle: dependency cycle: `b` depends on `a` again",
                "dependency-cycle: dependency cycle: `a` depends on `b` again",
                "self-dependency: target `c` depends on itself",
            ]
        );
    }

    #[test]
    fn names_and_duplicates() {
        let a = "includes('b')\nrule('mine')\ntarget('app')\n  add_rules('mode.debug', 'mine', 'mode.debgu', '@pkg/rule')\n  set_toolchains('clang', 'clagn')\n  add_options('nope')\n";
        let b = "target('app')\nadd_subdirs('.')\n";
        let found = lint(&[("xmake.lua", a), ("b/xmake.lua", b)], "xmake.lua");
        assert_eq!(found.len(), 4, "{found:?}");
        assert!(
            found[0].starts_with("duplicate-definition: target `app` is also defined in b"),
            "{found:?}"
        );
        assert_eq!(found[1], "unknown-rule: unknown rule `mode.debgu`");
        assert_eq!(found[2], "unknown-toolchain: unknown toolchain `clagn`");
        assert_eq!(found[3], "unknown-option: unknown option `nope`");
        // Another project of the workspace: not a duplicate.
        let separate = lint(
            &[("xmake.lua", "target('app')\n"), ("other/xmake.lua", "target('app')\n")],
            "xmake.lua",
        );
        assert!(separate.is_empty(), "{separate:?}");
        let deprecated = lint(&[("xmake.lua", a), ("b/xmake.lua", b)], "b/xmake.lua");
        assert!(
            deprecated
                .iter()
                .any(|d| d == "deprecated-api: `add_subdirs` is deprecated, use `includes`"),
            "{deprecated:?}"
        );
    }

    #[test]
    fn unmatched_paths() {
        let dir = std::env::temp_dir().join(format!("xmake-ide-unmatched-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("src")).expect("dirs");
        std::fs::create_dir_all(dir.join("include")).expect("dirs");
        std::fs::write(dir.join("src/main.cpp"), "").expect("file");
        let text = "target('a')
    add_files('src/*.cpp', 'src/*.c', 'src/mian.cpp', '$(builddir)/gen.c')
    add_includedirs('include', 'inc')
    add_headerfiles('include/(**.h)')
    set_targetdir('bin')
";
        let mut project = Project::new(ApiDb::builtin(), Dialect::XMAKE_DEFAULT);
        let file = project.set_file(dir.join("xmake.lua"), Arc::from(text));
        let mut out = Vec::new();
        check(&project, &file, &mut out);
        let found: Vec<_> = out
            .iter()
            .filter(|d| d.code == "unmatched-path")
            .map(|d| (d.message.to_string(), d.fixes.first().map(|f| f.replacement.to_string())))
            .collect();
        assert_eq!(
            found,
            [
                ("`src/*.c`: the pattern matches no file".to_owned(), None),
                ("`src/mian.cpp`: no such file".to_owned(), Some("main.cpp".to_owned())),
                ("`inc`: no such directory".to_owned(), Some("include".to_owned())),
                ("`include/(**.h)`: the pattern matches no file".to_owned(), None),
            ]
        );
        let _ = std::fs::remove_dir_all(&dir);
    }
}
