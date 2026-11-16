//! Context-aware completion.
//!
//! What is offered depends on where the cursor is:
//!
//! - in a string argument: the values the API accepts (`set_kind("|")`,
//!   `add_deps("|")`, `import("|")`, file paths for path APIs);
//! - after `x.` or `x:`: the functions of a module, an imported module or the
//!   methods of an instance (`target:|` in `on_build(function (target) ...)`);
//! - elsewhere: the APIs of the current scope in description code, the
//!   builtins of the script context in script code, locals and keywords.
//!
//! Items borrow their labels from the API database or the source whenever
//! possible.

use std::{borrow::Cow, path::Path};

use rustc_hash::FxHashSet;
use xmake_api::{ApiKind, ArgValues, Scope, ScopeApi, api_doc, instance_doc, module_doc};
use xmake_syntax::{
    Feature, SyntaxKind, SyntaxToken, TextRange, TextSize,
    ast::{AstNode, CallExpr},
};

use xmake_project::{call_arg_values, enclosing_call};

use crate::{
    AnalyzedFile, Project,
    context::CodeContext,
    outline::content_range,
    resolve::{BindingKind, visible_bindings},
};

/// The kind of a completion item, mapped to the protocol by the server.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompletionKind {
    Function,
    Method,
    Module,
    Variable,
    Keyword,
    Snippet,
    /// A value of an enumeration: `binary`, `mode.debug`, ...
    Value,
    /// A scope function: `target`, `option`, ...
    Scope,
    File,
    Folder,
}

/// A completion item.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompletionItem<'a> {
    pub label: Cow<'a, str>,
    pub kind: CompletionKind,
    pub detail: Option<Cow<'a, str>>,
    pub documentation: Option<Cow<'a, str>>,
    /// Snippet text (`$1`, `$0` placeholders) inserted instead of the label.
    pub snippet: Option<Cow<'a, str>>,
    /// The text the item replaces.
    pub replace: TextRange,
    /// Items of lower groups are listed first.
    pub group: u8,
}

pub(crate) mod annotations;

/// Sorting groups.
mod group {
    pub(super) const LOCAL: u8 = 0;
    pub(super) const SCOPE_API: u8 = 1;
    pub(super) const VALUE: u8 = 1;
    pub(super) const FUNCTION: u8 = 2;
    pub(super) const MODULE: u8 = 3;
    pub(super) const KEYWORD: u8 = 4;
}

/// Collects items, skipping duplicate labels and labels not matching the
/// typed prefix.
struct Items<'a> {
    prefix: &'a str,
    replace: TextRange,
    seen: FxHashSet<&'a str>,
    items: Vec<CompletionItem<'a>>,
}

impl<'a> Items<'a> {
    fn new(prefix: &'a str, replace: TextRange) -> Items<'a> {
        Items { prefix, replace, seen: FxHashSet::default(), items: Vec::new() }
    }

    /// Adds an item unless it is filtered out; returns it for more details.
    fn add(
        &mut self,
        label: &'a str,
        kind: CompletionKind,
        group: u8,
    ) -> Option<&mut CompletionItem<'a>> {
        if !fuzzy_match(label, self.prefix) || !self.seen.insert(label) {
            return None;
        }
        self.items.push(CompletionItem {
            label: Cow::Borrowed(label),
            kind,
            detail: None,
            documentation: None,
            snippet: None,
            replace: self.replace,
            group,
        });
        self.items.last_mut()
    }
}

/// Whether `label` matches what was typed: same first character, the rest a
/// subsequence, ignoring case. Editors filter again as the user types.
fn fuzzy_match(label: &str, typed: &str) -> bool {
    let mut label_chars = label.chars().map(|c| c.to_ascii_lowercase());
    let mut typed_chars = typed.chars().map(|c| c.to_ascii_lowercase());
    let Some(first) = typed_chars.next() else {
        return true;
    };
    if label_chars.next() != Some(first) {
        return false;
    }
    typed_chars.all(|t| label_chars.any(|l| l == t))
}

fn is_ident_byte(b: u8) -> bool {
    b == b'_' || b.is_ascii_alphanumeric()
}

/// Completions at `offset` of `file`, stored at `path` (for file paths).
pub fn completions<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    path: Option<&Path>,
    offset: TextSize,
) -> Vec<CompletionItem<'a>> {
    let root = file.root();
    let token = root.token_at_offset(offset).left_biased();
    if let Some(token) = token {
        if token.kind().is_comment() && token.text_range().start() < offset {
            // `---@param name type`: the annotations of LuaCATS.
            return annotations::completions(file, token, offset).unwrap_or_default();
        }
        if token.kind().is_string() && inside_string(token, offset) {
            if let Some(items) = variable_completions(project, file, token, offset) {
                return items;
            }
            if let Some(items) = tool_completions(file, token, offset) {
                return items;
            }
            if let Some(items) = literal_completions(project, file, token, offset) {
                return items;
            }
            return string_completions(project, file, path, token, offset);
        }
    }

    let text = &*file.text;
    let end = offset.into();
    let start = text[..end].bytes().rev().take_while(|&b| is_ident_byte(b)).count();
    let prefix_start = end - start;
    let prefix = &text[prefix_start..end];
    let replace = TextRange::new(TextSize::new(prefix_start as u32), offset);
    let mut items = Items::new(prefix, replace);

    let before = &text.as_bytes()[..prefix_start];
    match before.last() {
        Some(b'.') if before.len() < 2 || before[before.len() - 2] != b'.' => {
            member_completions(
                project,
                file,
                offset,
                &before[..before.len() - 1],
                false,
                &mut items,
            );
        }
        Some(b':') if before.len() < 2 || before[before.len() - 2] != b':' => {
            member_completions(
                project,
                file,
                offset,
                &before[..before.len() - 1],
                true,
                &mut items,
            );
        }
        _ => {
            // A number such as `1.5` is not a name.
            if !prefix.as_bytes().first().is_some_and(u8::is_ascii_digit) {
                name_completions(project, file, offset, &mut items);
            }
        }
    }
    items.items
}

/// `"$(scr|"`: the variables xmake substitutes; `"$|"`: the same, with
/// their `$(...)`.
fn variable_completions<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    token: SyntaxToken<'a>,
    offset: TextSize,
) -> Option<Vec<CompletionItem<'a>>> {
    let content = content_range(token);
    let before = &file.text[TextRange::new(content.start(), offset)];
    let variables = crate::variables::names(project, file);
    if let Some(open) = before.rfind("$(").filter(|&o| !before[o..].contains(')')) {
        let typed = &before[open + 2..];
        let replace = TextRange::new(offset - TextSize::new(typed.len() as u32), offset);
        let mut items = Items::new(typed, replace);
        for (name, doc) in variables {
            if let Some(item) = items.add(name, CompletionKind::Value, group::VALUE) {
                item.detail = crate::variables::value(project, file, name).map(Cow::Owned);
                item.documentation = Some(doc);
            }
        }
        for &(name, snippet, doc) in crate::variables::COMMANDS {
            if let Some(item) = items.add(name, CompletionKind::Snippet, group::FUNCTION) {
                item.snippet = Some(Cow::Borrowed(snippet));
                item.documentation = Some(Cow::Borrowed(doc));
            }
        }
        return Some(items.items);
    }
    if !before.ends_with('$') {
        return None;
    }
    let replace = TextRange::new(offset - TextSize::new(1), offset);
    let items = variables.into_iter().map(|(name, doc)| CompletionItem {
        label: Cow::Owned(format!("$({name})")),
        kind: CompletionKind::Value,
        detail: crate::variables::value(project, file, name).map(Cow::Owned),
        documentation: Some(doc),
        snippet: None,
        replace,
        group: group::VALUE,
    });
    Some(items.collect())
}

/// `use("|")` of `---@param std "c++20"|"c++23"`: the string literals the
/// parameter accepts, from the annotations (aliases followed) or the stubs.
fn literal_completions<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    token: SyntaxToken<'a>,
    offset: TextSize,
) -> Option<Vec<CompletionItem<'a>>> {
    // `target:kind() == "|"`: the values of the other side.
    if let Some(other) = xmake_types::compared_with(token.parent()?) {
        let ty = crate::type_of(project, file, other);
        return literal_items(file, token, offset, &ty, "compared with");
    }
    let (call, index) = enclosing_call(token)?;
    let params = xmake_types::expected_params(project, file, call.syntax())?;
    let param = xmake_types::nth_expected(&params, index)?;
    literal_items(file, token, offset, &param.ty, param.name)
}

/// The string literals of `ty` as items replacing the content of `token`.
fn literal_items<'a>(
    file: &'a AnalyzedFile,
    token: SyntaxToken<'a>,
    offset: TextSize,
    ty: &crate::Type<'a>,
    what: &str,
) -> Option<Vec<CompletionItem<'a>>> {
    let ty = ty.non_nil();
    let literals: Vec<&str> = ty
        .members()
        .iter()
        .filter_map(|m| match m {
            crate::Type::Literal(value) => Some(*value),
            _ => None,
        })
        .collect();
    if literals.is_empty() {
        return None;
    }
    let content = content_range(token);
    let typed = &file.text[TextRange::new(content.start(), offset)];
    let replace = TextRange::new(content.start(), content.end().max(offset));
    let mut items = Items::new(typed, replace);
    for value in literals {
        if let Some(item) = items.add(value, CompletionKind::Value, group::VALUE) {
            item.detail = Some(Cow::Owned(format!("{what}: {ty}")));
        }
    }
    Some(items.items)
}

/// `{tools = "|"}` of the flags: the tools of xmake.
fn tool_completions<'a>(
    file: &'a AnalyzedFile,
    token: SyntaxToken<'a>,
    offset: TextSize,
) -> Option<Vec<CompletionItem<'a>>> {
    let content = content_range(token);
    let typed = &file.text[TextRange::new(content.start(), offset)];
    let replace = TextRange::new(content.start(), content.end().max(offset));
    let Some((call, key)) = xmake_project::option_of_value(token.parent()?) else {
        return tool_prefix_completions(token, typed, replace);
    };
    if key != "tools" || !xmake_project::is_flags_api(call.callee_name()?) {
        return None;
    }
    let mut items = Items::new(typed, replace);
    for &tool in xmake_api::TOOLS {
        if let Some(item) = items.add(tool, CompletionKind::Value, group::VALUE) {
            item.detail = Some(Cow::Borrowed("tool"));
        }
    }
    Some(items.items)
}

/// `add_cxxflags("c|")`: a flag for one tool only, `"cl::/W4"`. Nothing
/// once the flag itself is being typed (`-`, `/`, `::`).
fn tool_prefix_completions<'a>(
    token: SyntaxToken<'a>,
    typed: &'a str,
    replace: TextRange,
) -> Option<Vec<CompletionItem<'a>>> {
    let (call, _) = enclosing_call(token)?;
    if !xmake_project::is_flags_api(call.callee_name()?)
        || typed.contains("::")
        || typed.starts_with(['-', '/'])
    {
        return None;
    }
    let items = xmake_api::TOOLS.iter().filter(|tool| fuzzy_match(tool, typed)).map(|tool| {
        CompletionItem {
            label: Cow::Owned(format!("{tool}::")),
            kind: CompletionKind::Value,
            detail: Some(Cow::Owned(format!("the flag for `{tool}` only"))),
            documentation: None,
            snippet: None,
            replace,
            group: group::VALUE,
        }
    });
    Some(items.collect())
}

/// Whether `offset` is between the quotes of a string token.
fn inside_string(token: SyntaxToken<'_>, offset: TextSize) -> bool {
    let content = content_range(token);
    let range = token.text_range();
    let closed = content.end() < range.end();
    offset >= content.start() && (offset <= content.end() || !closed && offset <= range.end())
}

// === Names ===

fn name_completions<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    offset: TextSize,
    items: &mut Items<'a>,
) {
    // `add_requires("zlib", {|})`, `add_files("a.c", {|})`: only the keys
    // of the options. Elsewhere `{x}` may be a value: names stay.
    if option_table_keys(project, file, offset, items) {
        return;
    }
    for binding in visible_bindings(file, offset) {
        let (kind, detail) = match binding.kind {
            BindingKind::Local => (CompletionKind::Variable, Cow::Borrowed("local")),
            BindingKind::LoopVar => (CompletionKind::Variable, Cow::Borrowed("loop variable")),
            BindingKind::Function => (CompletionKind::Function, Cow::Borrowed("function")),
            BindingKind::Param(Some(instance)) => {
                (CompletionKind::Variable, Cow::Borrowed(instance.name()))
            }
            BindingKind::Param(None) => (CompletionKind::Variable, Cow::Borrowed("parameter")),
            BindingKind::Import { module } => (CompletionKind::Module, Cow::Borrowed(module)),
        };
        if let Some(item) = items.add(binding.name, kind, group::LOCAL) {
            item.detail = Some(detail);
        }
    }

    let db = &project.db;
    match CodeContext::at(file, offset) {
        CodeContext::Description(scope) => {
            for (i, &visible) in scope.visible_scopes().iter().enumerate() {
                let group = if i == 0 { group::SCOPE_API } else { group::FUNCTION };
                for api in db.scope_apis(visible) {
                    add_scope_api(items, api, group);
                }
            }
            add_scope_functions(items, scope);
            for name in db.description_functions() {
                add_function(items, name);
            }
            for module in db.description_modules() {
                if let Some(item) = items.add(&module.name, CompletionKind::Module, group::MODULE) {
                    item.detail = Some(Cow::Borrowed("module"));
                }
            }
        }
        CodeContext::Script => {
            for name in db.script_functions() {
                add_function(items, name);
            }
            for module in db.script_modules() {
                if let Some(item) = items.add(&module.name, CompletionKind::Module, group::MODULE) {
                    item.detail = Some(Cow::Borrowed("module"));
                }
            }
        }
    }
    add_keywords(items, file);
}

fn add_scope_api<'a>(items: &mut Items<'a>, api: &'a ScopeApi, group: u8) {
    let Some(item) = items.add(&api.name, CompletionKind::Function, group) else {
        return;
    };
    item.detail = Some(Cow::Owned(format!("{} · {}", api.scope.name(), api.kind.describe())));
    item.documentation = api_doc(Some(api.scope), &api.name).map(Cow::Borrowed);
    item.snippet = Some(Cow::Owned(api_snippet(&api.name, api.kind, api.scope)));
}

/// The text inserted for an API: `set_kind("$1")`, `on_build(function
/// (target) ... end)`, ...
fn api_snippet(name: &str, kind: ApiKind, scope: Scope) -> String {
    match kind {
        ApiKind::Script => {
            let param = scope.script_instance().map_or("...", |i| i.name());
            format!("{name}(function ({param})\n\t$0\nend)")
        }
        ApiKind::Values | ApiKind::Paths | ApiKind::KeyValues | ApiKind::Dictionary => {
            format!("{name}(\"$1\")")
        }
        ApiKind::Groups | ApiKind::Custom => format!("{name}($1)"),
    }
}

/// `target`, `option`, ... and the `_end` of the current scope.
fn add_scope_functions(items: &mut Items<'_>, current: Scope) {
    for scope in Scope::NAMED {
        let name = scope.name();
        if let Some(item) = items.add(name, CompletionKind::Scope, group::FUNCTION) {
            item.detail = Some(Cow::Borrowed("scope"));
            item.documentation = api_doc(None, name).map(Cow::Borrowed);
            item.snippet = Some(Cow::Owned(format!("{name}(\"$1\")\n\t$0")));
        }
    }
    let end = crate::outline::end_function(current);
    if let Some(item) = end.and_then(|end| items.add(end, CompletionKind::Scope, group::SCOPE_API))
    {
        item.detail = Some(Cow::Borrowed("end of scope"));
        item.snippet = Some(Cow::Owned(format!("{}()", item.label)));
    }
}

fn add_function<'a>(items: &mut Items<'a>, name: &'a str) {
    if let Some(item) = items.add(name, CompletionKind::Function, group::FUNCTION) {
        item.documentation = api_doc(None, name).map(Cow::Borrowed);
        item.snippet = Some(Cow::Owned(format!("{name}($1)")));
    }
}

/// Keywords, plus snippets for block statements.
fn add_keywords(items: &mut Items<'_>, file: &AnalyzedFile) {
    const SNIPPETS: &[(&str, &str)] = &[
        ("function", "function ${1:name}($2)\n\t$0\nend"),
        ("local function", "local function ${1:name}($2)\n\t$0\nend"),
        ("if", "if ${1:condition} then\n\t$0\nend"),
        ("for", "for ${1:i} = ${2:1}, ${3:n} do\n\t$0\nend"),
        ("for in", "for ${1:_}, ${2:v} in ipairs(${3:t}) do\n\t$0\nend"),
        ("while", "while ${1:condition} do\n\t$0\nend"),
        ("repeat", "repeat\n\t$0\nuntil ${1:condition}"),
        ("do", "do\n\t$0\nend"),
    ];
    const KEYWORDS: &[&str] = &[
        "and", "break", "else", "elseif", "end", "false", "in", "local", "nil", "not", "or",
        "return", "then", "true", "until",
    ];
    for (label, snippet) in SNIPPETS {
        if let Some(item) = items.add(label, CompletionKind::Snippet, group::KEYWORD) {
            item.snippet = Some(Cow::Borrowed(snippet));
        }
    }
    for keyword in KEYWORDS {
        items.add(keyword, CompletionKind::Keyword, group::KEYWORD);
    }
    if file.dialect.has(Feature::Goto) {
        items.add("goto", CompletionKind::Keyword, group::KEYWORD);
    }
}

// === Members ===

/// After `receiver.` (`method == false`) or `receiver:`. `before` is the text
/// up to the dot.
fn member_completions<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    offset: TextSize,
    before: &[u8],
    method: bool,
    items: &mut Items<'a>,
) {
    let chain_len = before.iter().rev().take_while(|&&b| is_ident_byte(b) || b == b'.').count();
    let chain_start = before.len() - chain_len;
    let chain = &file.text[chain_start..before.len()];
    let mut segments = chain.split('.');
    let Some(head) = segments.next().filter(|h| !h.is_empty()) else {
        return;
    };
    let rest_start = (head.len() + 1).min(chain.len());
    let rest = &chain[rest_start..];

    // Typed members first: `dep:` after `local dep = target:dep("x")`.
    let receiver = crate::types::expression_before(file, TextSize::new(before.len() as u32));
    let ty = receiver.map_or(crate::Type::Unknown, |r| crate::type_of(project, file, r));
    if let Some(owner) = ty.owner() {
        // `s:upper()` calls `string.upper(s)`: on strings, every function is a method.
        let is_string = owner == "string";
        for function in xmake_api::Stubs::get()
            .members(owner)
            .filter(|f| f.method == method || is_string && method)
        {
            let kind = if method { CompletionKind::Method } else { CompletionKind::Function };
            if let Some(item) = items.add(function.name, kind, group::FUNCTION) {
                item.detail = Some(Cow::Owned(function.signature()));
                item.documentation =
                    (!function.doc.is_empty()).then_some(Cow::Borrowed(function.doc.as_str()));
                item.snippet = Some(Cow::Owned(format!("{}($1)", function.name)));
            }
        }
        if let (Some(class), true) = (ty.class(), method)
            && let Some(instance) = xmake_api::Instance::parse(class)
        {
            add_instance_methods(items, project, instance);
        }
    }

    let db = &project.db;
    let binding = visible_bindings(file, offset).into_iter().find(|b| b.name == head);
    match binding.map(|b| b.kind) {
        Some(BindingKind::Param(Some(instance))) if method && rest.is_empty() => {
            add_instance_methods(items, project, instance);
        }
        Some(BindingKind::Import { module }) if !method => {
            add_extension_members(items, project, module, rest);
        }
        Some(_) => {}
        None if !method && rest.is_empty() => {
            let script = CodeContext::at(file, offset).is_script();
            let Some(module) = db.module(head, script) else {
                return;
            };
            for name in module.functions.iter() {
                if let Some(item) = items.add(name, CompletionKind::Function, group::FUNCTION) {
                    item.detail = Some(Cow::Borrowed(&module.name));
                    item.documentation = module_doc(&module.name, name).map(Cow::Borrowed);
                    item.snippet = Some(Cow::Owned(format!("{name}($1)")));
                }
            }
        }
        None => {}
    }
}

/// Every method of `instance` xmake has, typed in the stubs or not.
fn add_instance_methods<'a>(
    items: &mut Items<'a>,
    project: &'a Project,
    instance: xmake_api::Instance,
) {
    for name in project.db.instance_methods(instance) {
        if let Some(item) = items.add(name, CompletionKind::Method, group::FUNCTION) {
            item.detail = Some(Cow::Borrowed(instance.name()));
            item.documentation = instance_doc(instance, name).map(Cow::Borrowed);
            item.snippet = Some(Cow::Owned(format!("{name}($1)")));
        }
    }
}

/// Members of the imported `module`: the next segment of the importable
/// functions under `module.rest.`.
fn add_extension_members<'a>(
    items: &mut Items<'a>,
    project: &'a Project,
    module: &str,
    rest: &str,
) {
    let apis = project.db.extension_apis();
    for full in apis {
        let Some(tail) = full.strip_prefix(module).and_then(|t| t.strip_prefix('.')) else {
            continue;
        };
        let tail = if rest.is_empty() {
            tail
        } else {
            match tail.strip_prefix(rest).and_then(|t| t.strip_prefix('.')) {
                Some(t) => t,
                None => continue,
            }
        };
        let (segment, is_module) = match tail.split_once('.') {
            Some((segment, _)) => (segment, true),
            None => (tail, false),
        };
        let kind = if is_module { CompletionKind::Module } else { CompletionKind::Function };
        if let Some(item) = items.add(segment, kind, group::FUNCTION)
            && !is_module
        {
            item.snippet = Some(Cow::Owned(format!("{segment}($1)")));
        }
    }
}

// === Strings ===

/// `add_requires("zlib|")`: the packages of the repositories and of the
/// project; after a blank, the versions of the package.
fn requirement_completions<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    typed: &'a str,
    content_start: TextSize,
    replace: TextRange,
    mut items: Items<'a>,
) -> Vec<CompletionItem<'a>> {
    let index = &project.packages;
    if let Some(space) = typed.rfind(char::is_whitespace) {
        let (name, _) = crate::packages::split_requirement(typed);
        let Some(details) = index.get(name).and_then(|e| e.details()) else {
            return Vec::new();
        };
        let start = content_start + TextSize::new(space as u32 + 1);
        let replace = TextRange::new(start, replace.end());
        let mut versions = details.versions;
        versions.sort_by(|a, b| xmake_project::compare_versions(b, a));
        return versions
            .into_iter()
            .enumerate()
            .filter(|(_, v)| fuzzy_match(v, &typed[space + 1..]))
            .map(|(i, version)| CompletionItem {
                label: Cow::Owned(version),
                kind: CompletionKind::Value,
                detail: Some(Cow::Borrowed(if i == 0 { "latest" } else { "version" })),
                documentation: None,
                snippet: None,
                replace,
                // Newest first.
                group: (i.min(9)) as u8,
            })
            .collect();
    }
    for def in project.scope_of(file).definitions(Scope::Package).collect::<Vec<_>>() {
        if let Some(item) = items.add(def.name(), CompletionKind::Value, group::LOCAL) {
            item.detail = Some(Cow::Borrowed("package of the project"));
        }
    }
    for entry in index.entries() {
        if let Some(item) = items.add(&entry.name, CompletionKind::Value, group::VALUE) {
            item.detail = Some(Cow::Borrowed(&entry.repository));
        }
    }
    items.items
}

/// The keys of the options table of `add_requires`, and of its `configs`
/// table: those of `add_configs` of the package, then the common ones.
/// Returns whether the cursor is at such a key.
fn option_table_keys<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    offset: TextSize,
    items: &mut Items<'a>,
) -> bool {
    let root = file.root();
    // At a key: right after `{`, `,` or `;`.
    let before =
        root.token_at_offset(TextSize::new(u32::from(items.replace.start()))).left_biased();
    let mut previous = before;
    while let Some(t) = previous.filter(|t| t.kind().is_trivia() || t.kind() == SyntaxKind::IDENT) {
        previous = t.prev_token();
    }
    if !previous.is_some_and(|t| {
        matches!(t.kind(), SyntaxKind::L_BRACE | SyntaxKind::COMMA | SyntaxKind::SEMICOLON)
    }) {
        return false;
    }
    let node = root.covering_node(TextRange::empty(offset));
    let Some(table) =
        std::iter::once(node).chain(node.ancestors()).find(|n| n.kind() == SyntaxKind::TABLE_EXPR)
    else {
        return false;
    };
    let require_call = |args: xmake_syntax::SyntaxNode<'a>| {
        let call = CallExpr::cast(args.parent()?)?;
        matches!(call.callee_name()?, "add_requires" | "add_requireconfs").then_some(call)
    };
    let parent = table.parent();
    // The options table of an API: `add_files("a.c", {|})`.
    let api_call = parent
        .filter(|p| p.kind() == SyntaxKind::ARG_LIST)
        .and_then(|a| CallExpr::cast(a.parent()?));
    if let Some(name) = api_call.and_then(|c| c.callee_name()) {
        let keys: &[(&str, &str)] = if matches!(name, "add_requires" | "add_requireconfs") {
            crate::packages::REQUIRE_OPTIONS
        } else {
            let db = &project.db;
            let api = db.scopes_of(name).next().and_then(|s| db.scope_api(s, name));
            xmake_api::option_keys(name, api.map(|a| a.kind))
        };
        if keys.is_empty() {
            return false;
        }
        for (key, doc) in keys {
            if let Some(item) = items.add(key, CompletionKind::Value, group::LOCAL) {
                item.documentation = Some(Cow::Borrowed(doc));
                item.snippet = Some(Cow::Owned(format!("{key} = $1")));
            }
        }
        return true;
    }
    // `configs = {|}` inside the options.
    let field = parent.filter(|p| p.kind() == SyntaxKind::TABLE_FIELD);
    let is_configs = field
        .and_then(xmake_syntax::ast::TableField::cast)
        .and_then(|f| f.key_name())
        .is_some_and(|k| k.text() == "configs");
    let options = field.and_then(|f| f.parent()).filter(|t| t.kind() == SyntaxKind::TABLE_EXPR);
    let call = options
        .and_then(|t| t.parent())
        .filter(|a| a.kind() == SyntaxKind::ARG_LIST)
        .and_then(require_call);
    let (true, Some(call)) = (is_configs, call) else {
        return false;
    };
    let package =
        call.arg(0).and_then(xmake_syntax::ast::Literal::cast).and_then(|l| l.string_value());
    let entry = package
        .as_deref()
        .and_then(|p| project.packages.get(crate::packages::split_requirement(p).0));
    let details = entry.and_then(crate::PackageEntry::details);
    for (config, description) in details.into_iter().flat_map(|d| d.configs) {
        if fuzzy_match(&config, items.prefix) {
            items.items.push(CompletionItem {
                snippet: Some(Cow::Owned(format!("{config} = $1"))),
                label: Cow::Owned(config),
                kind: CompletionKind::Value,
                detail: Some(Cow::Borrowed("package config")),
                documentation: Some(Cow::Owned(description)),
                replace: items.replace,
                group: group::LOCAL,
            });
        }
    }
    for (key, doc) in crate::packages::COMMON_CONFIGS {
        if let Some(item) = items.add(key, CompletionKind::Value, group::VALUE) {
            item.documentation = Some(Cow::Borrowed(doc));
            item.snippet = Some(Cow::Owned(format!("{key} = $1")));
        }
    }
    true
}

fn string_completions<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    path: Option<&Path>,
    token: SyntaxToken<'a>,
    offset: TextSize,
) -> Vec<CompletionItem<'a>> {
    let Some((call, index)) = enclosing_call(token) else {
        return Vec::new();
    };
    let Some(name) = call.callee_name() else {
        return Vec::new();
    };
    let content = content_range(token);
    let typed = &file.text[TextRange::new(content.start(), offset.max(content.start()))];
    let replace = TextRange::new(content.start(), content.end().max(offset));
    let mut items = Items::new(typed, replace);

    if name == "add_requires" {
        return requirement_completions(project, file, typed, content.start(), replace, items);
    }
    if let Some(values) = call_arg_values(file, call, index) {
        add_values(&mut items, project, file, values);
        return items.items;
    }
    let is_path_api = matches!(name, "includes" | "add_subdirs" | "add_subfiles")
        || project
            .db
            .scopes_of(name)
            .any(|s| project.db.scope_api(s, name).is_some_and(|a| a.kind == ApiKind::Paths));
    if let (true, Some(path)) = (is_path_api, path) {
        return path_completions(path, typed, content.start());
    }
    Vec::new()
}

fn add_values<'a>(
    items: &mut Items<'a>,
    project: &'a Project,
    file: &'a AnalyzedFile,
    values: ArgValues,
) {
    let project_scope = project.scope_of(file);
    let db = &project.db;
    let known = &db.values;
    let plain = |items: &mut Items<'a>, list: &'a [xmake_api::Str]| {
        for value in list {
            items.add(value, CompletionKind::Value, group::VALUE);
        }
    };
    let project_names = |items: &mut Items<'a>, scope: Scope| {
        for def in project_scope.definitions(scope) {
            if let Some(item) = items.add(def.name(), CompletionKind::Value, group::LOCAL) {
                item.detail = Some(Cow::Borrowed(scope.name()));
            }
        }
    };
    match values {
        ArgValues::Fixed(list) => {
            for (value, doc) in list {
                if let Some(item) = items.add(value, CompletionKind::Value, group::VALUE) {
                    item.documentation = Some(Cow::Borrowed(doc));
                }
            }
        }
        ArgValues::Platforms => plain(items, &known.platforms),
        ArgValues::Architectures => plain(items, &known.architectures),
        ArgValues::Modes => plain(items, &known.modes),
        ArgValues::Rules => {
            project_names(items, Scope::Rule);
            plain(items, &known.rules);
        }
        ArgValues::Policies | ArgValues::Toolchains => {
            if values == ArgValues::Toolchains {
                project_names(items, Scope::Toolchain);
            }
            let list =
                if values == ArgValues::Policies { &known.policies } else { &known.toolchains };
            for named in list.iter() {
                if let Some(item) = items.add(&named.name, CompletionKind::Value, group::VALUE) {
                    item.documentation = Some(Cow::Borrowed(&named.description));
                }
            }
        }
        ArgValues::Targets => project_names(items, Scope::Target),
        ArgValues::Options => project_names(items, Scope::Option),
        ArgValues::Configs => {
            project_names(items, Scope::Option);
            for (name, doc) in xmake_api::BUILTIN_CONFIGS {
                if let Some(item) = items.add(name, CompletionKind::Value, group::VALUE) {
                    item.detail = Some(Cow::Borrowed("builtin configuration"));
                    item.documentation = Some(Cow::Borrowed(doc));
                }
            }
        }
        ArgValues::Packages => {
            for require in project_scope.requires() {
                items.add(require.text(), CompletionKind::Value, group::LOCAL);
            }
        }
        ArgValues::ImportModules => {
            for full in db.extension_apis() {
                // `core.base.json.decode` is in module `core.base.json`.
                if let Some((module, _)) = full.rsplit_once('.')
                    && let Some(item) = items.add(module, CompletionKind::Module, group::MODULE)
                {
                    item.detail = Some(Cow::Borrowed("module"));
                }
            }
        }
    }
}

/// Files and directories next to `file_path`, under the directory typed so
/// far in `typed`.
fn path_completions<'a>(
    file_path: &Path,
    typed: &str,
    content_start: TextSize,
) -> Vec<CompletionItem<'a>> {
    let Some(base) = file_path.parent() else {
        return Vec::new();
    };
    let (dir, partial) = match typed.rfind('/') {
        Some(i) => (&typed[..=i], &typed[i + 1..]),
        None => ("", typed),
    };
    let Ok(entries) = std::fs::read_dir(base.join(dir)) else {
        return Vec::new();
    };
    let start = content_start + TextSize::new(dir.len() as u32);
    let replace = TextRange::new(start, start + TextSize::new(partial.len() as u32));
    let mut items = Vec::new();
    for entry in entries.flatten() {
        let Ok(name) = entry.file_name().into_string() else {
            continue;
        };
        if name.starts_with('.') || !fuzzy_match(&name, partial) {
            continue;
        }
        let is_dir = entry.file_type().is_ok_and(|t| t.is_dir());
        let kind = if is_dir { CompletionKind::Folder } else { CompletionKind::File };
        items.push(CompletionItem {
            label: Cow::Owned(name),
            kind,
            detail: None,
            documentation: None,
            snippet: None,
            replace,
            group: if is_dir { 0 } else { 1 },
        });
    }
    items
}

#[cfg(test)]
mod tests;
