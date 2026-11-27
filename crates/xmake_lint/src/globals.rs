//! Lints on names: `undefined-global` and `unused-local`.

use std::borrow::Cow;

use rustc_hash::FxHashSet;
use xmake_api::Scope;
use xmake_syntax::{
    SyntaxKind, SyntaxNode, SyntaxToken,
    ast::{AstNode, Name},
};

use crate::{
    AnalyzedFile, Diagnostic, FileKind, Fix, Project, Severity,
    context::CodeContext,
    fuzzy,
    resolve::{resolve, visible_bindings},
};

/// The globals of standard Lua that xmake keeps in both contexts.
const LUA_GLOBALS: &[&str] = &[
    "_G",
    "_VERSION",
    "_ENV",
    "assert",
    "collectgarbage",
    "error",
    "getmetatable",
    "ipairs",
    "next",
    "pairs",
    "pcall",
    "print",
    "rawequal",
    "rawget",
    "rawlen",
    "rawset",
    "select",
    "setmetatable",
    "tonumber",
    "tostring",
    "type",
    "unpack",
    "xpcall",
    "string",
    "table",
    "math",
    "coroutine",
    "utf8",
    "bit",
    "bit32",
    "require",
    "load",
    "loadfile",
    "dofile",
];

/// Whether `name` is predefined in `context`.
fn is_predefined(project: &Project, context: CodeContext, name: &str) -> bool {
    if LUA_GLOBALS.contains(&name) {
        return true;
    }
    let db = &project.db;
    match context {
        CodeContext::Description(_) => {
            db.is_description_function(name)
                || db.module(name, false).is_some()
                || db.scope_api(Scope::Root, name).is_some()
                || db.scopes_of(name).next().is_some()
                || Scope::from_opener(name).is_some()
                || matches!(name, "namespace" | "namespace_end")
        }
        // `_g`: the private table of a module.
        CodeContext::Script => {
            name == "_g"
                || db.script_functions().iter().any(|f| f == name)
                || db.module(name, true).is_some()
        }
    }
}

/// Names a file defines as globals: `function f()` anywhere, `x = ...` of
/// a name that is not a local.
pub(crate) fn file_globals(file: &AnalyzedFile) -> FxHashSet<&str> {
    let mut out = FxHashSet::default();
    for node in file.root().descendants() {
        match node.kind() {
            SyntaxKind::FUNCTION_STMT => {
                let name = node.child_node(SyntaxKind::FUNC_NAME).and_then(|n| n.first_token());
                out.extend(name.filter(|t| t.kind() == SyntaxKind::IDENT).map(|t| t.text()));
            }
            SyntaxKind::ASSIGN_STMT => {
                let targets = node.children().filter(|n| n.kind() == SyntaxKind::NAME_REF);
                for ident in targets.filter_map(|n| n.first_token()) {
                    if resolve(file, ident).is_none() {
                        out.insert(ident.text());
                    }
                }
            }
            _ => {}
        }
    }
    out
}

/// `undefined-global`: names neither declared nor predefined.
pub(crate) fn undefined_globals<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    out: &mut Vec<Diagnostic<'a>>,
) {
    let mut globals = file_globals(file);
    // Description files share their globals with the files they include.
    if file.kind == FileKind::Description {
        let project_scope = project.scope_of(file);
        // A file the project cannot load may define more globals.
        if project_scope.incomplete {
            return;
        }
        for (_, other) in project_scope.files().filter(|(_, f)| f.kind == FileKind::Description) {
            globals.extend(file_globals(other));
        }
    }
    let root = file.root();
    let name_refs = root.descendants().filter(|n| n.kind() == SyntaxKind::NAME_REF);
    for ident in name_refs.filter_map(|n| n.first_token()) {
        let name = ident.text();
        if globals.contains(name) || resolve(file, ident).is_some() {
            continue;
        }
        let context = CodeContext::at(file, ident.text_range().start());
        if is_predefined(project, context, name) {
            continue;
        }
        // The API checks report the description APIs better.
        if matches!(context, CodeContext::Description(_)) && looks_like_api(name) {
            continue;
        }
        out.push(undefined(project, file, ident, context));
    }
}

fn looks_like_api(name: &str) -> bool {
    ["set_", "add_", "del_", "remove_", "on_", "before_", "after_", "is_", "has_"]
        .iter()
        .any(|p| name.starts_with(p))
}

fn undefined<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    ident: SyntaxToken<'a>,
    context: CodeContext,
) -> Diagnostic<'a> {
    let name = ident.text();
    let range = ident.text_range();
    let other = if context.is_script() {
        CodeContext::Description(Scope::Root)
    } else {
        CodeContext::Script
    };
    let message = if is_predefined(project, other, name) {
        let place = if other.is_script() {
            "script code (`on_*` callbacks, modules)"
        } else {
            "description code"
        };
        format!("`{name}` is only available in {place}")
    } else {
        format!("undefined global `{name}`")
    };
    // Suggest a visible local or a builtin of the context.
    let db = &project.db;
    let locals = visible_bindings(file, range.start());
    let builtins =
        if context.is_script() { db.script_functions() } else { db.description_functions() };
    let candidates = locals
        .iter()
        .map(|b| b.name)
        .chain(LUA_GLOBALS.iter().copied())
        .chain(builtins.iter().map(|f| &**f));
    let fix = fuzzy::closest(name, candidates).map(|c| Fix::replace(range, c));
    // `json` without `import("core.base.json")`.
    let import = context.is_script().then(|| importable(project, name)).flatten();
    let import = import.map(|module| crate::fixes::insert_import(file, range.start(), module));
    Diagnostic {
        range,
        severity: Severity::Warning,
        message: Cow::Owned(message),
        code: "undefined-global",
        fixes: import.into_iter().chain(fix).collect(),
    }
}

/// The module `import(...)` binds to `name`: `core.base.json` for `json`,
/// `lib.detect.find_tool` for `find_tool`. The shortest wins.
fn importable<'a>(project: &'a Project, name: &str) -> Option<&'a str> {
    let modules = project.db.extension_apis().iter().flat_map(|full| {
        // The module of a function, and the function itself (callable modules).
        let module = full.rsplit_once('.').map(|(m, _)| m);
        module.into_iter().chain(std::iter::once(&**full))
    });
    modules.filter(|m| m.rsplit('.').next() == Some(name)).min_by_key(|m| m.len())
}

/// `unused-local`: locals, local functions and imports never used.
pub(crate) fn unused_locals<'a>(file: &'a AnalyzedFile, out: &mut Vec<Diagnostic<'a>>) {
    let root = file.root();
    let mut used: FxHashSet<(u32, u32)> = FxHashSet::default();
    for ident in root
        .descendants()
        .filter(|n| n.kind() == SyntaxKind::NAME_REF)
        .filter_map(|n| n.first_token())
    {
        if let Some(binding) = resolve(file, ident) {
            used.insert((binding.range.start().into(), binding.range.end().into()));
        }
    }
    // `import("core.base.option")` and no `option` after.
    for stmt in root.descendants().filter(|n| n.kind() == SyntaxKind::CALL_STMT) {
        let Some(binding) = xmake_project::import_binding(stmt) else {
            continue;
        };
        let range = binding.range;
        if binding.name.starts_with('_')
            || used.contains(&(range.start().into(), range.end().into()))
        {
            continue;
        }
        // The line of the statement, removed.
        let text = &*file.text;
        let start = text[..usize::from(stmt.text_range().start())].rfind('\n').map_or(0, |i| i + 1);
        let end = usize::from(stmt.text_range().end());
        let end = text[end..].find('\n').map_or(text.len(), |i| end + i + 1);
        let only = text[start..usize::from(stmt.text_range().start())].trim().is_empty()
            && text[usize::from(stmt.text_range().end())..end].trim().is_empty();
        let fix = only.then(|| Fix {
            title: Cow::Owned(format!("Remove the import of `{}`", binding.name)),
            range: xmake_syntax::TextRange::new((start as u32).into(), (end as u32).into()),
            replacement: Cow::Borrowed(""),
            safe: false,
        });
        out.push(Diagnostic {
            range,
            severity: Severity::Warning,
            message: Cow::Owned(format!("unused import `{}`", binding.name)),
            code: "unused-import",
            fixes: fix.into_iter().collect(),
        });
    }
    for node in root.descendants() {
        let declared: Vec<Name<'_>> = match node.kind() {
            SyntaxKind::LOCAL_STMT => local_names(node),
            SyntaxKind::LOCAL_FUNCTION_STMT => node.children().filter_map(Name::cast).collect(),
            _ => continue,
        };
        for ident in declared.into_iter().filter_map(|n| n.ident()) {
            let range = ident.text_range();
            if ident.text().starts_with('_')
                || used.contains(&(range.start().into(), range.end().into()))
            {
                continue;
            }
            let message = Cow::Owned(format!("unused local `{}`", ident.text()));
            let title = Cow::Owned(format!("Rename to `_{}`", ident.text()));
            let replacement = Cow::Owned(format!("_{}", ident.text()));
            let fix = Some(Fix { title, range, replacement, safe: false });
            out.push(Diagnostic {
                range,
                severity: Severity::Warning,
                message,
                code: "unused-local",
                fixes: fix.into_iter().collect(),
            });
        }
    }
}

/// The names of `local a, b <close>`, without the to-be-closed ones, which
/// are used by going out of scope.
fn local_names(statement: SyntaxNode<'_>) -> Vec<Name<'_>> {
    let Some(list) = statement.child_node(SyntaxKind::NAME_LIST) else {
        return Vec::new();
    };
    list.children()
        .filter_map(Name::cast)
        .filter(|n| {
            let attrib = n.syntax().next_sibling().filter(|a| a.kind() == SyntaxKind::ATTRIB);
            !attrib.is_some_and(|a| {
                a.child_token(SyntaxKind::IDENT).is_some_and(|t| t.text() == "close")
            })
        })
        .collect()
}
