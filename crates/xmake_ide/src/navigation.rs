//! Go to definition, find references and rename.
//!
//! Two kinds of names are navigable: Lua names (locals, parameters, imports,
//! file-level functions) and project names in strings (`target("app")` and
//! `add_deps("app")`, options, rules, toolchains, required packages).

use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use xmake_api::{ArgValues, Scope};
use xmake_syntax::{
    SyntaxKind, SyntaxToken, TextRange, TextSize,
    ast::{AstNode, CallExpr, Literal},
};

use crate::{
    AnalyzedFile, Project, outline::content_range, resolve::resolve, xmake_project::annotations,
    xmake_project::enclosing_call,
};

/// A location in the project.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NavTarget<'a> {
    pub path: Cow<'a, Path>,
    /// The range to select, empty for a whole file.
    pub range: TextRange,
}

/// What the cursor is on.
enum Symbol<'a> {
    /// A Lua name declared at `range` of the current file.
    Lua { range: TextRange },
    /// A project name used in a string.
    Project { kind: ProjectName, name: Cow<'a, str> },
    /// A global function of the project: `function name() end`.
    Global { name: &'a str },
}

/// The kinds of project names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ProjectName {
    Defined(Scope),
    Package,
}

impl ProjectName {
    fn of_values(values: ArgValues) -> Option<ProjectName> {
        let kind = match values {
            ArgValues::Targets => ProjectName::Defined(Scope::Target),
            ArgValues::Options | ArgValues::Configs => ProjectName::Defined(Scope::Option),
            ArgValues::Rules => ProjectName::Defined(Scope::Rule),
            ArgValues::Toolchains => ProjectName::Defined(Scope::Toolchain),
            ArgValues::Packages => ProjectName::Package,
            _ => return None,
        };
        Some(kind)
    }

    /// The kind of names a string argument of `call` holds.
    fn of_arg(file: &AnalyzedFile, call: CallExpr<'_>, index: usize) -> Option<ProjectName> {
        let name = call.callee_name()?;
        if let Some((scope, xmake_api::ScopeEdge::Begin)) = Scope::from_opener(name) {
            return (index == 0).then_some(ProjectName::Defined(scope));
        }
        if name == "add_requires" {
            return Some(ProjectName::Package);
        }
        ProjectName::of_values(xmake_project::call_arg_values(file, call, index)?)
    }
}

fn token_at(file: &AnalyzedFile, offset: TextSize) -> Option<SyntaxToken<'_>> {
    file.root().token_at_offset(offset).max_by_key(|t| {
        matches!(t.kind(), SyntaxKind::IDENT | SyntaxKind::STRING | SyntaxKind::LONG_STRING)
    })
}

fn symbol_at(file: &AnalyzedFile, offset: TextSize) -> Option<(Symbol<'_>, TextRange)> {
    let token = token_at(file, offset)?;
    match token.kind() {
        SyntaxKind::IDENT => {
            let parent = token.parent()?;
            let range = match parent.kind() {
                SyntaxKind::NAME => token.text_range(),
                SyntaxKind::NAME_REF => match resolve(file, token) {
                    // `t()` of `function t()`: a global, maybe used elsewhere.
                    Some(binding) if is_function_name(file, binding.range) => {
                        return Some((Symbol::Global { name: token.text() }, token.text_range()));
                    }
                    Some(binding) => binding.range,
                    None => return global_function_at(file, token),
                },
                // `t` of `function t() end`
                SyntaxKind::FUNC_NAME => {
                    let statement = parent.parent()?;
                    let name = xmake_types::global_function_name(statement)?;
                    return Some((Symbol::Global { name }, token.text_range()));
                }
                _ => return None,
            };
            Some((Symbol::Lua { range }, token.text_range()))
        }
        // `para` of `---@param para string`: the parameter.
        SyntaxKind::LINE_COMMENT => {
            let annotation = annotations::Annotation::parse(token)?;
            let (name, range, _) =
                annotation.name_and_rest().filter(|_| annotation.tag == "param")?;
            let name = name.trim_end_matches('?');
            let range = TextRange::at(range.start(), TextSize::new(name.len() as u32));
            if !range.contains_inclusive(offset) {
                return None;
            }
            let statement = annotations::documented_statement(token)?;
            let params = annotations::function_params(statement)?;
            let param = annotations::param_names(params).find(|p| p.text() == name)?;
            Some((Symbol::Lua { range: param.text_range() }, range))
        }
        SyntaxKind::STRING | SyntaxKind::LONG_STRING => {
            let (call, index) = enclosing_call(token)?;
            let kind = ProjectName::of_arg(file, call, index)?;
            let value = Literal::cast(token.parent()?)?.string_value()?;
            let name = match kind {
                // `add_requires("zlib 1.2.x")`: the package is `zlib`.
                ProjectName::Package => match value {
                    Cow::Borrowed(v) => Cow::Borrowed(v.split_whitespace().next().unwrap_or(v)),
                    Cow::Owned(v) => {
                        Cow::Owned(v.split_whitespace().next().unwrap_or("").to_owned())
                    }
                },
                ProjectName::Defined(_) => value,
            };
            Some((Symbol::Project { kind, name }, content_range(token)))
        }
        _ => None,
    }
}

/// Whether the name declared at `range` is that of `function name() end`.
fn is_function_name(file: &AnalyzedFile, range: TextRange) -> bool {
    let token = file.root().token_at_offset(range.start()).right_biased();
    token.and_then(|t| t.parent()).is_some_and(|n| n.kind() == SyntaxKind::FUNC_NAME)
}

/// `t` in `t()` when it calls the global function `t` of the file.
fn global_function_at<'a>(
    file: &'a AnalyzedFile,
    token: SyntaxToken<'a>,
) -> Option<(Symbol<'a>, TextRange)> {
    let name = token.text();
    let declared = file.root().descendants().any(|n| {
        n.kind() == SyntaxKind::FUNCTION_STMT && xmake_types::global_function_name(n) == Some(name)
    });
    declared.then_some((Symbol::Global { name }, token.text_range()))
}

/// The declarations and uses of the global function `name` in `file`.
fn global_uses(file: &AnalyzedFile, name: &str) -> Vec<TextRange> {
    let idents = file
        .root()
        .descendant_tokens()
        .filter(|t| t.kind() == SyntaxKind::IDENT && t.text() == name);
    let mut out: Vec<TextRange> = idents
        .filter(|ident| match ident.parent() {
            // Unresolved, or resolved to the `function name()` itself.
            Some(p) if p.kind() == SyntaxKind::NAME_REF => {
                resolve(file, *ident).is_none_or(|b| is_function_name(file, b.range))
            }
            Some(p) if p.kind() == SyntaxKind::FUNC_NAME => {
                p.parent().is_some_and(|s| xmake_types::global_function_name(s) == Some(name))
            }
            _ => false,
        })
        .map(|t| t.text_range())
        .collect();
    out.sort_by_key(|r| r.start());
    out
}

/// Where the item at `offset` is defined.
pub fn goto_definition<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    path: &'a Path,
    offset: TextSize,
) -> Vec<NavTarget<'a>> {
    let files = goto_files(project, file, path, offset);
    if !files.is_empty() {
        return files;
    }
    // `utils.greet` of `import("utils")`: the function in `utils.lua`.
    if let Some(target) = module_member_at(project, file, offset) {
        return vec![target];
    }
    // `set_kind`, `os.cp`, `target:name`: their stubs.
    if let Some(target) = stub_at(project, file, offset) {
        return vec![target];
    }
    // `xmake.value.kind` of `---@param k xmake.value.kind`: its stub.
    if let Some(target) = stub_type_at(file, offset) {
        return vec![target];
    }
    let Some((symbol, _)) = symbol_at(file, offset) else {
        return Vec::new();
    };
    let project_scope = project.scope_of(file);
    match symbol {
        Symbol::Lua { range } => vec![NavTarget { path: Cow::Borrowed(path), range }],
        Symbol::Global { name } => {
            let mut out = Vec::new();
            for (path, other) in project_scope.files() {
                let declared = other.root().descendants().filter(|n| {
                    n.kind() == SyntaxKind::FUNCTION_STMT
                        && xmake_types::global_function_name(*n) == Some(name)
                });
                for function_name in declared.filter_map(|s| s.child_node(SyntaxKind::FUNC_NAME)) {
                    let range = function_name.trimmed_range();
                    out.push(NavTarget { path: Cow::Borrowed(path), range });
                }
            }
            out
        }
        Symbol::Project { kind: ProjectName::Defined(scope), name } => project_scope
            .definitions(scope)
            .filter(|d| d.name() == name)
            .map(|d| NavTarget { path: Cow::Borrowed(d.path), range: d.item.name_range })
            .collect(),
        Symbol::Project { kind: ProjectName::Package, name } => project_scope
            .requires()
            .filter(|r| r.text() == name)
            .map(|r| NavTarget { path: Cow::Borrowed(r.path), range: r.item })
            .collect(),
    }
}

/// Path arguments lead to files: `includes("../")`, `add_files("src/*.c")`
/// (every match), `import("a.b")` (a module next to the script).
fn goto_files<'a>(
    project: &Project,
    file: &AnalyzedFile,
    path: &Path,
    offset: TextSize,
) -> Vec<NavTarget<'a>> {
    let Some(token) = token_at(file, offset).filter(|t| t.kind().is_string()) else {
        return Vec::new();
    };
    let Some(dir) = path.parent() else {
        return Vec::new();
    };
    string_files(project, file, dir, token, true)
        .map(|f| f.files)
        .unwrap_or_default()
        .into_iter()
        .map(|p| NavTarget { path: Cow::Owned(p), range: TextRange::default() })
        .collect()
}

/// The stub of the API, function or method at `offset`, in the files of the
/// stubs.
fn stub_at<'a>(project: &Project, file: &AnalyzedFile, offset: TextSize) -> Option<NavTarget<'a>> {
    use xmake_api::Stubs;
    let token = token_at(file, offset).filter(|t| t.kind() == SyntaxKind::IDENT)?;
    let parent = token.parent()?;
    let stubs = Stubs::get();
    let function = match parent.kind() {
        SyntaxKind::NAME_REF if resolve(file, token).is_none() => {
            let in_field = parent
                .parent()
                .and_then(xmake_syntax::ast::FieldExpr::cast)
                .is_some_and(|f| f.field().is_some_and(|t| t == token));
            if in_field {
                return None;
            }
            let scope = match crate::CodeContext::at(file, offset) {
                crate::CodeContext::Description(scope) => Some(scope),
                crate::CodeContext::Script => None,
            };
            xmake_api::api_stub(scope, token.text())?
        }
        // `os.cp`, `json.decode`, `import("a.b").f`
        SyntaxKind::FIELD_EXPR => {
            let field = xmake_syntax::ast::FieldExpr::cast(parent)?;
            if field.field()? != token {
                return None;
            }
            let owner = xmake_types::type_of(project, file, field.base()?).owner()?;
            stubs.find(owner, token.text())?
        }
        // `target:name()`
        SyntaxKind::METHOD_CALL_EXPR => {
            let call = xmake_syntax::ast::MethodCallExpr::cast(parent)?;
            let owner = xmake_types::type_of(project, file, call.receiver()?).owner()?;
            stubs.find(owner, token.text())?
        }
        _ => return None,
    };
    let path = stubs.path_of(function.location)?;
    Some(NavTarget { path: Cow::Owned(path), range: function.location.range })
}

/// The class or alias of the stubs named at `offset` in an annotation:
/// `xmake.value.kind`, `target`.
fn stub_type_at<'a>(file: &AnalyzedFile, offset: TextSize) -> Option<NavTarget<'a>> {
    let token = token_at(file, offset).filter(|t| t.kind() == SyntaxKind::LINE_COMMENT)?;
    let text = token.text();
    if !text.trim_start_matches('-').trim_start().starts_with('@') {
        return None;
    }
    let at = usize::from(offset - token.text_range().start());
    let is_name = |c: char| c.is_ascii_alphanumeric() || c == '_' || c == '.';
    let start = text[..at].rfind(|c: char| !is_name(c)).map_or(0, |i| i + 1);
    let end = text[at..].find(|c: char| !is_name(c)).map_or(text.len(), |i| at + i);
    let name = text.get(start..end)?.trim_matches('.');
    if name.is_empty() {
        return None;
    }
    let stubs = xmake_api::Stubs::get();
    let location =
        stubs.alias(name).map(|a| a.location).or_else(|| stubs.class(name).map(|c| c.location))?;
    let path = stubs.path_of(location)?;
    Some(NavTarget { path: Cow::Owned(path), range: location.range })
}

/// `greet` in `utils.greet` with `utils` a module of the project.
fn module_member_at<'a>(
    project: &Project,
    file: &AnalyzedFile,
    offset: TextSize,
) -> Option<NavTarget<'a>> {
    let token = token_at(file, offset).filter(|t| t.kind() == SyntaxKind::IDENT)?;
    let field = xmake_syntax::ast::FieldExpr::cast(token.parent()?)?;
    if field.field()? != token {
        return None;
    }
    let module = xmake_types::type_of(project, file, field.base()?).module()?;
    let local = crate::modules::local_module(project, file, module)?;
    let functions = crate::modules::exported_functions(&local.file);
    let (_, _, range) = functions.into_iter().find(|(_, name, _)| *name == token.text())?;
    Some(NavTarget { path: Cow::Owned(local.path.clone()), range })
}

/// The files a string argument names.
pub(crate) struct StringFiles {
    /// The package of `add_requires`, not a path.
    pub package: bool,
    /// A glob: `src/*.c`.
    pub glob: bool,
    /// `import("core.base.json")` and others not of the project: xmake has
    /// them, not the disk.
    pub module: bool,
    /// The files, or the directory a path names (`add_includedirs("inc")`).
    pub files: Vec<PathBuf>,
}

/// What the string `token`, an argument of a call in a script of `dir`,
/// names on the disk. Globs are expanded only when `globs`.
pub(crate) fn string_files(
    project: &Project,
    file: &AnalyzedFile,
    dir: &Path,
    token: SyntaxToken<'_>,
    globs: bool,
) -> Option<StringFiles> {
    let (call, index) = enclosing_call(token)?;
    let name = call.callee_name()?;
    let value = token.parent().and_then(Literal::cast).and_then(|l| l.string_value())?;
    let glob = value.contains('*');
    if glob && !globs {
        return None;
    }
    let (package, files) = match name {
        // A package of the repositories, unless the project defines it.
        "add_requires" => {
            let (package, _) = crate::packages::split_requirement(&value);
            if project.scope_of(file).definitions(Scope::Package).any(|d| d.name() == package) {
                return None;
            }
            (true, project.packages.get(package).map(|e| e.path.clone()).into_iter().collect())
        }
        "includes" | "add_subdirs" | "add_subfiles" => {
            (false, crate::paths::resolve_include(dir, &value))
        }
        "import" if index == 0 => {
            let found: Vec<PathBuf> = crate::modules::module_path(dir, &value)
                .into_iter()
                .chain(stub_module(&value))
                .collect();
            if found.is_empty() {
                // `lib.detect.find_program`: a module of xmake.
                return Some(StringFiles { package: false, glob, module: true, files: found });
            }
            (false, found)
        }
        _ if is_paths_api(project, name) => (false, crate::paths::resolve_paths(dir, &value)),
        _ => return None,
    };
    // `add_includedirs("inc")`, `includes("sub")` without its xmake.lua: the
    // directory.
    let files = if files.is_empty() && !glob && !package {
        let path = crate::paths::normalize(&dir.join(value.split('|').next().unwrap_or("").trim()));
        if path.is_dir() { vec![path] } else { files }
    } else {
        files
    };
    Some(StringFiles { package, glob, module: false, files })
}

/// The stub of the importable module `module`: `modules/core/base/json.lua`.
fn stub_module(module: &str) -> Option<PathBuf> {
    let dir = xmake_api::Stubs::get().dir()?;
    let path = dir.join(xmake_api::render::module_file(module));
    path.is_file().then_some(path)
}

/// The strings naming one file (`includes("sub")`, `import("utils")`,
/// `add_files("main.c")`, a package of `add_requires`), for the client to
/// underline and open. Globs are left to go-to-definition.
pub fn document_links(
    project: &Project,
    file: &AnalyzedFile,
    path: &Path,
) -> Vec<(TextRange, PathBuf)> {
    let Some(dir) = path.parent() else {
        return Vec::new();
    };
    let strings = file.root().descendant_tokens().filter(|t| t.kind().is_string());
    strings
        .filter_map(|token| {
            let found = string_files(project, file, dir, token, false)?;
            let [target] = <[PathBuf; 1]>::try_from(found.files).ok()?;
            // A directory opens nothing in an editor.
            target.is_file().then(|| (content_range(token), target))
        })
        .collect()
}

/// Whether `name` takes paths: `add_files`, `add_headerfiles`, ...
fn is_paths_api(project: &Project, name: &str) -> bool {
    let db = &project.db;
    db.scopes_of(name)
        .filter_map(|s| db.scope_api(s, name))
        .any(|a| a.kind == xmake_api::ApiKind::Paths)
}

/// Every use of the item at `offset`, the declaration included.
pub fn references<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    path: &'a Path,
    offset: TextSize,
) -> Vec<NavTarget<'a>> {
    // `greet` of `utils.greet`: the function in `utils.lua` and its uses.
    if let Some(target) = module_member_at(project, file, offset)
        && let Some(token) = token_at(file, offset)
    {
        let uses = crate::modules::member_uses(project, &target.path, token.text());
        let uses =
            uses.into_iter().map(|(path, range)| NavTarget { path: Cow::Borrowed(path), range });
        return std::iter::once(target).chain(uses).collect();
    }
    let Some((symbol, _)) = symbol_at(file, offset) else {
        return Vec::new();
    };
    match symbol {
        Symbol::Lua { range } => lua_references(file, range)
            .into_iter()
            .map(|range| NavTarget { path: Cow::Borrowed(path), range })
            .collect(),
        Symbol::Project { kind, name } => name_uses(project, file, kind, &name)
            .into_iter()
            .map(|(path, range)| NavTarget { path: Cow::Borrowed(path), range })
            .collect(),
        Symbol::Global { name } => {
            let mut out = Vec::new();
            for (path, file) in project.scope_of(file).files() {
                for range in global_uses(file, name) {
                    out.push(NavTarget { path: Cow::Borrowed(path), range });
                }
            }
            // A function of a module: `m.name` where it is imported.
            for (path, range) in crate::modules::member_uses(project, path, name) {
                out.push(NavTarget { path: Cow::Borrowed(path), range });
            }
            out
        }
    }
}

/// The declaration at `declaration` and the names referring to it.
fn lua_references(file: &AnalyzedFile, declaration: TextRange) -> Vec<TextRange> {
    let root = file.root();
    let name = &file.text[declaration];
    let mut out = vec![declaration];
    let idents =
        root.descendant_tokens().filter(|t| t.kind() == SyntaxKind::IDENT && t.text() == name);
    for ident in idents {
        let is_ref = ident.parent().is_some_and(|p| p.kind() == SyntaxKind::NAME_REF);
        if is_ref && resolve(file, ident).is_some_and(|b| b.range == declaration) {
            out.push(ident.text_range());
        }
    }
    out.extend(param_annotation(file, declaration));
    out.sort_by_key(|r| r.start());
    out.dedup();
    out
}

/// `para` of `---@param para string` above the function whose parameter is
/// declared at `declaration`.
fn param_annotation(file: &AnalyzedFile, declaration: TextRange) -> Option<TextRange> {
    let token = file.root().token_at_offset(declaration.start()).right_biased()?;
    let params = token.parent()?.parent().filter(|p| p.kind() == SyntaxKind::PARAM_LIST)?;
    let statement = annotations::statement_of_params(params)?;
    let name = token.text();
    annotations::above(statement).into_iter().filter(|a| a.tag == "param").find_map(|a| {
        let (declared, range, _) = a.name_and_rest()?;
        (declared.trim_end_matches('?') == name)
            .then(|| TextRange::at(range.start(), TextSize::new(name.len() as u32)))
    })
}

/// The strings of `file` naming the project item `name` of `kind`.
fn project_name_uses(file: &AnalyzedFile, kind: ProjectName, name: &str) -> Vec<TextRange> {
    let root = file.root();
    let mut out = Vec::new();
    for token in root.descendant_tokens().filter(|t| t.kind().is_string()) {
        let Some((call, index)) = enclosing_call(token) else {
            continue;
        };
        if ProjectName::of_arg(file, call, index) != Some(kind) {
            continue;
        }
        let content = content_range(token);
        let text = &file.text[content];
        let used = match kind {
            ProjectName::Package => text.split_whitespace().next() == Some(name),
            ProjectName::Defined(_) => text == name || text.rsplit("::").next() == Some(name),
        };
        if used {
            let len = if kind == ProjectName::Package { name.len() } else { text.len() };
            out.push(TextRange::at(content.start(), TextSize::new(len as u32)));
        }
    }
    out
}

/// The uses of a project name in the files it is visible from. A package is
/// both defined (`package("x")`, in the project or in one of its
/// repositories) and used (`add_requires("x")`, `add_packages("x")`): every
/// file of the workspace counts, a repository being a project of its own.
fn name_uses<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    kind: ProjectName,
    name: &str,
) -> Vec<(&'a Path, TextRange)> {
    let is_package = matches!(kind, ProjectName::Package | ProjectName::Defined(Scope::Package));
    let files: Vec<(&'a Path, &'a std::sync::Arc<AnalyzedFile>)> = if is_package {
        project.files().collect()
    } else {
        project.scope_of(file).files().collect()
    };
    let kinds: &[ProjectName] = if is_package {
        &[ProjectName::Defined(Scope::Package), ProjectName::Package]
    } else {
        std::slice::from_ref(&kind)
    };
    let mut out = Vec::new();
    for (path, other) in files {
        for &kind in kinds {
            out.extend(project_name_uses(other, kind, name).into_iter().map(|r| (path, r)));
        }
    }
    out.sort_by(|a, b| a.0.cmp(b.0).then(a.1.start().cmp(&b.1.start())));
    out.dedup();
    out
}

/// The definitions of `file` (targets, options, ...) with their uses in the
/// project, for reference counts.
pub fn code_lenses<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    path: &'a Path,
) -> Vec<(TextRange, Vec<NavTarget<'a>>)> {
    let mut out = Vec::with_capacity(file.outline.definitions.len());
    for def in &file.outline.definitions {
        let name = &file.text[def.name_range];
        let uses = name_uses(project, file, ProjectName::Defined(def.scope), name)
            .into_iter()
            .filter(|&(other, range)| !(other == path && range == def.name_range))
            .map(|(other, range)| NavTarget { path: Cow::Borrowed(other), range })
            .collect();
        out.push((def.name_range, uses));
    }
    // The functions of a module: their uses here and where it is imported.
    if file.kind == crate::FileKind::Script {
        for (_, name, range) in crate::modules::exported_functions(file) {
            let local = global_uses(file, name).into_iter().filter(|r| *r != range);
            let mut uses: Vec<NavTarget<'a>> =
                local.map(|range| NavTarget { path: Cow::Borrowed(path), range }).collect();
            for (other, range) in crate::modules::member_uses(project, path, name) {
                uses.push(NavTarget { path: Cow::Borrowed(other), range });
            }
            out.push((range, uses));
        }
    }
    out
}

/// The uses of the item at `offset` in its own file, for highlighting.
pub fn highlights(file: &AnalyzedFile, offset: TextSize) -> Vec<TextRange> {
    match symbol_at(file, offset) {
        Some((Symbol::Lua { range }, _)) => lua_references(file, range),
        Some((Symbol::Project { kind, name }, _)) => project_name_uses(file, kind, &name),
        Some((Symbol::Global { name }, _)) => global_uses(file, name),
        None => Vec::new(),
    }
}

/// The range a rename at `offset` would change, if renaming is possible.
/// Imports are not renamed: their name comes from the module path.
pub fn prepare_rename(file: &AnalyzedFile, offset: TextSize) -> Option<TextRange> {
    let (symbol, range) = symbol_at(file, offset)?;
    match symbol {
        Symbol::Lua { range: declaration } => {
            let token = token_at(file, declaration.start())?;
            let is_import = token.kind().is_string();
            (!is_import).then_some(range)
        }
        Symbol::Project { kind: ProjectName::Defined(_), .. } | Symbol::Global { .. } => {
            Some(range)
        }
        Symbol::Project { kind: ProjectName::Package, .. } => None,
    }
}

/// The edits renaming the item at `offset`: every range gets the new name.
pub fn rename<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    path: &'a Path,
    offset: TextSize,
) -> Vec<NavTarget<'a>> {
    if prepare_rename(file, offset).is_none() {
        return Vec::new();
    }
    references(project, file, path, offset)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use xmake_api::ApiDb;
    use xmake_syntax::Dialect;

    use super::*;

    fn project(files: &[(&str, &str)]) -> Project {
        let mut project = Project::new(ApiDb::builtin(), Dialect::XMAKE_DEFAULT);
        for (path, text) in files {
            project.set_file(PathBuf::from(path), Arc::from(*text));
        }
        project
    }

    fn offset_of(text: &str, needle: &str) -> TextSize {
        TextSize::new(text.find(needle).expect("needle") as u32)
    }

    #[test]
    fn target_across_files() {
        let lib = "target('core')\n  set_kind('static')\n";
        let app = "includes('lib')\ntarget('app')\n  add_deps('core')\n";
        let p = project(&[("lib/xmake.lua", lib), ("xmake.lua", app)]);
        let path = Path::new("xmake.lua");
        let file = p.file(path).expect("file");

        let defs = goto_definition(&p, file, path, offset_of(app, "core") + TextSize::new(1));
        assert_eq!(defs.len(), 1);
        assert_eq!(defs[0].path, Path::new("lib/xmake.lua"));
        assert_eq!(&lib[defs[0].range], "core");

        let mut refs: Vec<_> = references(&p, file, path, offset_of(app, "core"))
            .into_iter()
            .map(|t| t.path.display().to_string())
            .collect();
        refs.sort();
        assert_eq!(refs, ["lib/xmake.lua", "xmake.lua"]);
    }

    #[test]
    fn paths() {
        let dir = std::env::temp_dir().join(format!("xmake-ide-nav-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("app/src")).expect("dirs");
        std::fs::write(dir.join("common.lua"), "").expect("file");
        std::fs::write(dir.join("app/src/main.c"), "").expect("file");
        let text = "includes('../common.lua')
target('app')
  add_files('src/*.c')
  add_includedirs('src')
  on_load(function (t) import('lib.detect.find_program') end)
";
        let path = dir.join("app/xmake.lua");
        let p = project(&[]);
        let mut p = p;
        let file = p.set_file(path.clone(), Arc::from(text));
        let include = goto_definition(&p, &file, &path, offset_of(text, "common"));
        assert_eq!(include.len(), 1);
        assert_eq!(include[0].path, crate::paths::normalize(&dir.join("common.lua")));
        let sources = goto_definition(&p, &file, &path, offset_of(text, "src/"));
        assert_eq!(sources.len(), 1);
        assert!(sources[0].path.ends_with("main.c"));

        // One file: a link; a glob: none, its files are in the hover.
        // A module of xmake: its stub.
        let links = document_links(&p, &file, &path);
        assert_eq!(links.len(), 2);
        assert_eq!(&text[links[0].0], "../common.lua");
        assert!(links[1].1.ends_with("find_program.lua"), "{:?}", links[1].1);
        let hover = crate::hover(&p, &file, offset_of(text, "src/")).expect("hover").markdown;
        assert!(hover.starts_with("1 file:"), "{hover}");
        assert!(hover.contains("main.c](file:///"), "{hover}");
        let hover = crate::hover(&p, &file, offset_of(text, "common")).expect("hover").markdown;
        assert!(hover.starts_with('`') && hover.contains("common.lua"), "{hover}");
        // A directory: its path, no link.
        let hover = crate::hover(&p, &file, offset_of(text, "src')")).expect("hover").markdown;
        assert!(hover.contains("src"), "{hover}");
        assert!(!hover.contains("Not found"), "{hover}");
        assert!(document_links(&p, &file, &path).iter().all(|(r, _)| &text[*r] != "src"));
        // A module of xmake: not a missing file.
        let hover = crate::hover(&p, &file, offset_of(text, "lib.detect")).map(|h| h.markdown);
        assert!(hover.as_ref().is_none_or(|h| !h.contains("Not found")), "{hover:?}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn local_modules() {
        let dir = std::env::temp_dir().join(format!("xmake-ide-mod-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("dir");
        std::fs::write(
            dir.join("utils.lua"),
            "--- Says hello.
---@param name string
function greet(name) end
function _private() end
",
        )
        .expect("file");
        let text = "target('a')
  on_load(function (t)
    import('utils')
    utils.greet('x')
    utils.
  end)
";
        let path = dir.join("xmake.lua");
        let mut p = project(&[]);
        let file = p.set_file(path.clone(), Arc::from(text));

        let found = goto_definition(&p, &file, &path, offset_of(text, "greet"));
        assert_eq!(found.len(), 1);
        assert!(found[0].path.ends_with("utils.lua"));
        assert_eq!(found[0].range, TextRange::new(47.into(), 52.into()));

        let hover = crate::hover(&p, &file, offset_of(text, "greet")).expect("hover").markdown;
        assert!(hover.contains("function greet(name: string)"), "{hover}");
        assert!(hover.contains("Says hello."), "{hover}");

        let offset = offset_of(
            text, "utils.
",
        ) + TextSize::new(6);
        let items = crate::completions(&p, &file, Some(&path), offset);
        let labels: Vec<_> = items.iter().map(|i| &*i.label).collect();
        assert!(labels.contains(&"greet"), "{labels:?}");
        assert!(!labels.contains(&"_private"), "{labels:?}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn modules_of_included_files() {
        let dir = std::env::temp_dir().join(format!("xmake-ide-modinc-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("scripts")).expect("dirs");
        let utils = "function clang_format(fix) end
function run() clang_format(true) end
";
        std::fs::write(dir.join("scripts/task_utils.lua"), utils).expect("file");
        let tasks = "task('fmt')
  on_run(function ()
    import('task_utils', {rootdir = os.scriptdir()})
    task_utils.clang_format(true)
  end)
task_end()
";
        std::fs::write(dir.join("scripts/tasks.lua"), tasks).expect("file");
        let mut p = project(&[]);
        p.set_file(
            dir.join("xmake.lua"),
            Arc::from(
                "includes('scripts/tasks.lua')
",
            ),
        );
        p.update_includes();
        let tasks_path = crate::paths::normalize(&dir.join("scripts/tasks.lua"));
        let tasks_file = Arc::clone(p.file(&tasks_path).expect("included"));

        let found = goto_definition(&p, &tasks_file, &tasks_path, offset_of(tasks, "clang_format"));
        assert_eq!(found.len(), 1);
        assert!(found[0].path.ends_with("task_utils.lua"));

        // The module opened: its functions count the uses in tasks.lua.
        let utils_path = crate::paths::normalize(&dir.join("scripts/task_utils.lua"));
        let utils_file = p.set_file(utils_path.clone(), Arc::from(utils));
        let lenses = code_lenses(&p, &utils_file, &utils_path);
        let counts: Vec<_> = lenses.iter().map(|(r, uses)| (&utils[*r], uses.len())).collect();
        assert_eq!(counts, [("clang_format", 2), ("run", 0)]);
        // The client is told: an xmake file, not plain Lua.
        assert_eq!(p.imported_modules(), [utils_path.clone()]);
        let refs = references(&p, &tasks_file, &tasks_path, offset_of(tasks, "clang_format"));
        assert_eq!(
            refs.len(),
            2,
            "{:?}",
            refs.iter().map(|r| r.path.display().to_string()).collect::<Vec<_>>()
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn stubs() {
        let text = "target('a')
  set_kind('binary')
  on_load(function (t)
    os.cp('a', 'b')
    t:name()
    import('core.base.json')
  end)
";
        let path = PathBuf::from("/p/xmake.lua");
        let mut p = project(&[]);
        let file = p.set_file(path.clone(), Arc::from(text));
        let at = |needle: &str| {
            let found = goto_definition(&p, &file, &path, offset_of(text, needle));
            let target = found.first().unwrap_or_else(|| panic!("{needle}"));
            target.path.to_string_lossy().replace('\\', "/")
        };
        assert!(at("set_kind").ends_with("description/scopes/target.lua"));
        assert!(at("cp(").ends_with("script/os.lua"));
        assert!(at("name()").ends_with("script/instances/target.lua"));
        assert!(at("core.base").ends_with("modules/core/base/json.lua"));

        // A type of an annotation: the alias or class of the stubs.
        let text = "---@param k xmake.value.kind
---@param t target
local function f(k, t) end
";
        let path = PathBuf::from("/p/xmake.lua");
        let file = p.set_file(path.clone(), Arc::from(text));
        let at = |needle: &str| {
            let found = goto_definition(&p, &file, &path, offset_of(text, needle));
            let target = found.first().unwrap_or_else(|| panic!("{needle}"));
            target.path.to_string_lossy().replace('\\', "/")
        };
        assert!(at("value.kind").ends_with("types/values.lua"));
        assert!(
            at("target
")
            .ends_with("script/instances/target.lua")
        );
    }

    #[test]
    fn package_uses() {
        // A package of a local repository: its own project, used by the root.
        let mut p = project(&[]);
        let root = "add_requires('zlib 1.3', 'fmt')
target('a')
  add_packages('zlib')
";
        let recipe = "package('zlib')
  set_homepage('x')
";
        let root_path = PathBuf::from("/p/xmake.lua");
        let recipe_path = PathBuf::from("/p/repo/packages/z/zlib/xmake.lua");
        p.set_file(root_path.clone(), Arc::from(root));
        let file = p.set_file(recipe_path.clone(), Arc::from(recipe));
        let lenses = code_lenses(&p, &file, &recipe_path);
        let uses: Vec<_> =
            lenses[0].1.iter().map(|t| (t.path.to_path_buf(), &root[t.range])).collect();
        assert_eq!(uses, [(root_path.clone(), "zlib"), (root_path, "zlib")]);
    }

    #[test]
    fn included_files_are_descriptions() {
        let dir = std::env::temp_dir().join(format!("xmake-ide-inc-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("app")).expect("dirs");
        // No scope call at the top level: alone, it would be a script.
        std::fs::write(
            dir.join("flags.lua"),
            "add_defines('X')
",
        )
        .expect("file");
        let mut p = project(&[]);
        p.set_file(
            dir.join("app/xmake.lua"),
            Arc::from(
                "includes('../flags.lua')
",
            ),
        );
        assert!(p.update_includes());
        let flags = crate::paths::normalize(&dir.join("flags.lua"));
        assert_eq!(p.file(&flags).map(|f| f.kind), Some(crate::FileKind::Description));
        assert!(!p.update_includes(), "stable");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn locals() {
        let text = "local x = 1\nprint(x)\nlocal function f() return x end\n";
        let p = project(&[("a.lua", text)]);
        let path = Path::new("a.lua");
        let file = p.file(path).expect("file");
        let use_offset = offset_of(text, "x)");
        let defs = goto_definition(&p, file, path, use_offset);
        assert_eq!(defs[0].range, TextRange::at(TextSize::new(6), TextSize::new(1)));
        assert_eq!(highlights(file, use_offset).len(), 3);
        assert!(prepare_rename(file, use_offset).is_some());
        assert_eq!(rename(&p, file, path, use_offset).len(), 3);
    }
}

#[cfg(test)]
mod annotation_tests {
    use std::sync::Arc;

    use xmake_api::ApiDb;
    use xmake_syntax::Dialect;

    use super::*;

    const TEXT: &str = "---@param para? string\nfunction t(para)\n    return para\nend\n\nt('x')\n";

    fn renamed(at: usize) -> Vec<String> {
        let mut project = Project::new(ApiDb::builtin(), Dialect::XMAKE_DEFAULT);
        let file = project.set_file(PathBuf::from("xmake.lua"), Arc::from(TEXT));
        let path = Path::new("xmake.lua");
        let offset = TextSize::new(at as u32);
        assert!(prepare_rename(&file, offset).is_some());
        let found = rename(&project, &file, path, offset);
        found.iter().map(|t| format!("{}@{}", &TEXT[t.range], u32::from(t.range.start()))).collect()
    }

    #[test]
    fn params_and_their_annotations() {
        let from_param = renamed(TEXT.find("(para)").expect("param") + 1);
        let from_annotation = renamed(TEXT.find("para?").expect("annotation"));
        assert_eq!(from_param, ["para@10", "para@34", "para@51"]);
        assert_eq!(from_annotation, from_param);
    }

    #[test]
    fn global_functions() {
        let from_call = renamed(TEXT.rfind("t(").expect("call"));
        let from_declaration = renamed(TEXT.find("t(para)").expect("declaration"));
        assert_eq!(from_call, ["t@32", "t@61"]);
        assert_eq!(from_declaration, from_call);
    }
}
