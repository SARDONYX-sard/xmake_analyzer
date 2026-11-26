//! Hover documentation, laid out like rustdoc in rust-analyzer:
//!
//! ````markdown
//! ```lua
//! target                        <- where the item lives
//! ```
//! ```lua
//! set_kind(values: string...)   <- its signature
//! ```
//! ---
//! Summary.
//!
//! # Examples
//! ...
//! # Notes
//! ...
//! ````

use std::{borrow::Cow, fmt::Write as _};

use xmake_api::{
    ApiKind, ArgValues, Instance, Scope, Stubs, api_doc, api_params, doc_link, generated_api_doc,
    generated_member_doc, member_link, render_signature,
};
use xmake_syntax::{
    SyntaxKind, SyntaxNode, SyntaxToken, TextRange, TextSize,
    ast::{AstNode, FieldExpr, MethodCallExpr},
};

use crate::{
    AnalyzedFile, Project,
    context::CodeContext,
    doc_comment::doc_comment,
    resolve::{Binding, BindingKind, resolve},
    types::type_of,
    xmake_project::annotations,
    xmake_project::enclosing_call,
};

/// The result of a hover: Markdown text about `range`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hover {
    pub range: TextRange,
    pub markdown: String,
}

/// Documentation of the item at `offset`.
pub fn hover(project: &Project, file: &AnalyzedFile, offset: TextSize) -> Option<Hover> {
    let token = file.root().token_at_offset(offset).max_by_key(|t| {
        matches!(
            t.kind(),
            SyntaxKind::IDENT | SyntaxKind::STRING | SyntaxKind::LONG_STRING | SyntaxKind::INT
        )
    })?;
    // `"$(scriptdir)/src"`
    if token.kind().is_string()
        && let Some((range, name)) =
            crate::variables::variable_at(token.text(), token.text_range().start(), offset)
    {
        return Some(Hover { range, markdown: crate::variables::hover(project, file, name) });
    }
    // `includes("sub")`, `add_files("src/*.c")`: the files.
    if token.kind().is_string()
        && let Some(markdown) = path_hover(project, file, token)
    {
        return Some(Hover { range: token.text_range(), markdown });
    }
    let markdown = match token.kind() {
        // `---@param path string`
        SyntaxKind::LINE_COMMENT => return annotation_hover(project, file, token, offset),
        SyntaxKind::IDENT => ident_hover(project, file, token)?,
        SyntaxKind::STRING | SyntaxKind::LONG_STRING => {
            string_hover(project, file, token).or_else(|| index_hover(project, file, token))?
        }
        // `t[1]`
        SyntaxKind::INT => index_hover(project, file, token)?,
        _ => return None,
    };
    Some(Hover { range: token.text_range(), markdown })
}

/// At most this many files are listed for a glob.
const MAX_LISTED_FILES: usize = 20;

/// The files a path argument names, as links.
fn path_hover(project: &Project, file: &AnalyzedFile, token: SyntaxToken<'_>) -> Option<String> {
    let dir = crate::modules::path_of(project, file)?.parent()?;
    let found = crate::navigation::string_files(project, file, dir, token, true)?;
    if found.package || found.module {
        return None;
    }
    let mut out = String::new();
    match found.files.as_slice() {
        [] if found.glob => out.push_str("No file matches."),
        [] => out.push_str("Not found."),
        // The string is a link already (`document_links`): the path only.
        [single] if !found.glob => {
            let _ = write!(out, "`{}`", single.display());
        }
        files => {
            let plural = if files.len() == 1 { "" } else { "s" };
            let _ = writeln!(out, "{} file{plural}:\n", files.len());
            for file in files.iter().take(MAX_LISTED_FILES) {
                let shown = file.strip_prefix(dir).unwrap_or(file);
                let _ = writeln!(out, "- [{}]({})", shown.display(), file_uri(file));
            }
            if files.len() > MAX_LISTED_FILES {
                let _ = writeln!(out, "- ...");
            }
        }
    }
    Some(out)
}

/// `file:///D:/a%20b/c.lua`
fn file_uri(path: &std::path::Path) -> String {
    let path = path.to_string_lossy().replace('\\', "/");
    let mut out = String::from(if path.starts_with('/') { "file://" } else { "file:///" });
    for c in path.chars() {
        match c {
            ' ' => out.push_str("%20"),
            '#' => out.push_str("%23"),
            '%' => out.push_str("%25"),
            '(' => out.push_str("%28"),
            ')' => out.push_str("%29"),
            _ => out.push(c),
        }
    }
    out
}

/// A rustdoc-like page: location, signature, rule, documentation, link.
fn page(location: &str, signature: &str, doc: &str, link: Option<String>) -> String {
    let mut out = String::with_capacity(location.len() + signature.len() + doc.len() + 96);
    let _ = write!(out, "```lua\n{location}\n```\n```lua\n{signature}\n```\n\n---\n\n{doc}");
    if let Some(url) = link {
        let _ = write!(out, "\n\n[xmake manual]({url})");
    }
    out
}

fn ident_hover(project: &Project, file: &AnalyzedFile, token: SyntaxToken<'_>) -> Option<String> {
    let parent = token.parent()?;
    match parent.kind() {
        SyntaxKind::NAME_REF => match resolve(file, token) {
            Some(binding) => Some(binding_hover(project, file, &binding)),
            None => global_hover(project, file, token),
        },
        SyntaxKind::NAME => {
            let binding = declared_binding(file, token)?;
            Some(binding_hover(project, file, &binding))
        }
        SyntaxKind::FIELD_EXPR => {
            let field = FieldExpr::cast(parent)?;
            field_hover(project, file, field).or_else(|| table_field_hover(project, file, field))
        }
        // `name` in `{name = "app"}`.
        SyntaxKind::TABLE_FIELD => {
            let field = xmake_syntax::ast::TableField::cast(parent)?;
            let key = field.key_name()?.text();
            let described = crate::tables::describe_field(project, file, key, field);
            let signature = format!("(field) {described}");
            // A key of an API options table has a meaning.
            Some(match option_doc(project, field, key) {
                Some(doc) => format!("```lua\n{signature}\n```\n\n---\n\n{doc}"),
                None => format!("```lua\n{signature}\n```"),
            })
        }
        SyntaxKind::METHOD_CALL_EXPR => method_hover(project, file, MethodCallExpr::cast(parent)?),
        // `t` in `function t() end`.
        SyntaxKind::FUNC_NAME => {
            let statement = parent.parent()?;
            let name = xmake_types::global_function_name(statement)?;
            Some(function_page(project, file, statement, name))
        }
        _ => None,
    }
}

fn binding_hover(project: &Project, file: &AnalyzedFile, binding: &Binding<'_>) -> String {
    let name = binding.name;
    let statement = declaration(file, binding.range);
    // As rust-analyzer: every variable with its type.
    let ty = || crate::types::variable_type(project, file, binding.kind, binding.range);
    let signature = match binding.kind {
        BindingKind::Local => match statement.and_then(|s| const_value(s, binding.range)) {
            Some(value) => format!("local {name} <const>: {} = {value}", ty()),
            None => match crate::types::initializer(file, binding.range) {
                Some(table) if table.kind() == SyntaxKind::TABLE_EXPR => {
                    format!("local {name}: {}", crate::tables::shape(project, file, table))
                }
                _ => format!("local {name}: {}", ty()),
            },
        },
        BindingKind::LoopVar => format!("for {name}: {}", ty()),
        BindingKind::Function => match statement {
            Some(statement) => return function_page(project, file, statement, name),
            None => format!("function {name}()"),
        },
        BindingKind::Param(Some(instance)) => format!("(parameter) {name}: {}", instance.name()),
        BindingKind::Param(None) => format!("(parameter) {name}: {}", ty()),
        BindingKind::Import { module } => format!("import(\"{module}\")"),
    };
    let doc: Option<Cow<'_, str>> =
        statement.and_then(doc_comment).or_else(|| match binding.kind {
            BindingKind::Param(Some(instance)) => {
                Some(Cow::Owned(format!("The {} instance this script runs for.", instance.name())))
            }
            BindingKind::Import { module } => {
                Some(Cow::Owned(format!("The imported module `{module}`.")))
            }
            BindingKind::LoopVar => Some(Cow::Borrowed("Loop variable.")),
            _ => None,
        });
    match doc {
        Some(doc) => format!("```lua\n{signature}\n```\n\n---\n\n{doc}"),
        None => format!("```lua\n{signature}\n```"),
    }
}

/// The binding a declared name introduces: `x` in `local x`, `i` in
/// `for i = 1, n`, a parameter, ...
fn declared_binding<'a>(file: &'a AnalyzedFile, token: SyntaxToken<'a>) -> Option<Binding<'a>> {
    let name = token.parent()?;
    let owner = name.parent()?;
    let owner = if owner.kind() == SyntaxKind::NAME_LIST { owner.parent()? } else { owner };
    let kind = match owner.kind() {
        SyntaxKind::LOCAL_STMT => BindingKind::Local,
        SyntaxKind::NUMERIC_FOR_STMT | SyntaxKind::GENERIC_FOR_STMT => BindingKind::LoopVar,
        SyntaxKind::LOCAL_FUNCTION_STMT => BindingKind::Function,
        SyntaxKind::PARAM_LIST => {
            // The first parameter of a script callback is its instance.
            let function = owner.parent()?;
            let first = owner.children().find(|n| n.kind() == SyntaxKind::NAME) == Some(name);
            let instance = file
                .outline
                .callback(function.text_range())
                .and_then(|c| c.scope.script_instance());
            BindingKind::Param(instance.filter(|_| first))
        }
        _ => return None,
    };
    Some(Binding { name: token.text(), range: token.text_range(), kind })
}

/// The statement declaring the name at `range`.
fn declaration(file: &AnalyzedFile, range: TextRange) -> Option<SyntaxNode<'_>> {
    let token = file.root().token_at_offset(range.start()).right_biased()?;
    token.ancestors().find(|n| {
        matches!(
            n.kind(),
            SyntaxKind::LOCAL_STMT
                | SyntaxKind::LOCAL_FUNCTION_STMT
                | SyntaxKind::FUNCTION_STMT
                | SyntaxKind::CALL_STMT
                | SyntaxKind::GLOBAL_STMT
        )
    })
}

/// The initializer of the `<const>` local declared at `range`.
fn const_value(statement: SyntaxNode<'_>, range: TextRange) -> Option<&str> {
    let names = statement.child_node(SyntaxKind::NAME_LIST)?;
    let mut index = 0;
    let mut is_const = false;
    for child in names.children() {
        match child.kind() {
            SyntaxKind::NAME if child.text_range().contains_range(range) => {
                // Its attribute follows it.
                is_const =
                    child.next_sibling().filter(|n| n.kind() == SyntaxKind::ATTRIB).is_some_and(
                        |a| a.child_token(SyntaxKind::IDENT).is_some_and(|t| t.text() == "const"),
                    );
                break;
            }
            SyntaxKind::NAME => index += 1,
            _ => {}
        }
    }
    if !is_const {
        return None;
    }
    let value = statement.child_node(SyntaxKind::EXPR_LIST)?.children().nth(index)?;
    let text = value.text().trim();
    // A long table or function: its first line only.
    Some(text.lines().next().unwrap_or(text))
}

/// The page of a function of the project: its signature typed by its
/// annotations and its `return`s, and its documentation.
fn function_page(
    project: &Project,
    file: &AnalyzedFile,
    statement: SyntaxNode<'_>,
    name: &str,
) -> String {
    let local = if statement.kind() == SyntaxKind::LOCAL_FUNCTION_STMT { "local " } else { "" };
    let mut signature = format!("{local}function {name}(");
    if let Some(params) = annotations::function_params(statement) {
        for (i, param) in annotations::param_names(params).enumerate() {
            let separator = if i == 0 { "" } else { ", " };
            let param = param.text();
            let _ = match annotations::param_type(statement, param) {
                Some((ty, optional)) => {
                    let optional = if optional { "?" } else { "" };
                    write!(
                        signature,
                        "{separator}{param}{optional}: {}",
                        xmake_types::Type::parse(ty)
                    )
                }
                None => write!(signature, "{separator}{param}"),
            };
        }
    }
    signature.push(')');
    let returns = xmake_types::function_returns(project, file, statement);
    for (i, ty) in returns.iter().enumerate() {
        let _ = write!(signature, "{}{ty}", if i == 0 { " -> " } else { ", " });
    }

    let mut doc = doc_comment(statement).map(Cow::into_owned).unwrap_or_default();
    let deprecated = annotations::above(statement).into_iter().find(|a| a.tag == "deprecated");
    if let Some(deprecated) = deprecated {
        let note = match deprecated.args {
            "" => "**Deprecated.**".to_owned(),
            reason => format!("**Deprecated:** {reason}"),
        };
        doc = if doc.is_empty() {
            note
        } else {
            format!(
                "{note}

{doc}"
            )
        };
    }
    if doc.is_empty() {
        format!(
            "```lua
{signature}
```"
        )
    } else {
        format!(
            "```lua
{signature}
```

---

{doc}"
        )
    }
}

/// The tag or the type under the cursor in an annotation.
fn annotation_hover(
    project: &Project,
    file: &AnalyzedFile,
    token: SyntaxToken<'_>,
    offset: TextSize,
) -> Option<Hover> {
    let annotation = annotations::Annotation::parse(token)?;
    if annotation.tag_range.contains_inclusive(offset) {
        let tag = annotation.tag;
        let doc = crate::completion::annotations::tag_doc(tag)?;
        let markdown = format!(
            "```lua
---@{tag}
```

---

{doc}"
        );
        return Some(Hover { range: annotation.tag_range, markdown });
    }
    // The word under the cursor: a type name.
    let range = token.text_range();
    let text = &file.text[range];
    let at = usize::from(offset - range.start());
    let is_word = |b: u8| b == b'_' || b == b'.' || b.is_ascii_alphanumeric();
    let start = text[..at].bytes().rev().take_while(|&b| is_word(b)).count();
    let end = text[at..].bytes().take_while(|&b| is_word(b)).count();
    let word = &text[at - start..at + end];
    let word_range =
        TextRange::new(offset - TextSize::new(start as u32), offset + TextSize::new(end as u32));
    let markdown = type_doc(project, file, word)?;
    Some(Hover { range: word_range, markdown })
}

/// The documentation of the type `name`: builtin, a class of the stubs, or
/// declared by the project with `---@class`, `---@alias` or `---@enum`.
fn type_doc(project: &Project, file: &AnalyzedFile, name: &str) -> Option<String> {
    if let Some(doc) = crate::completion::annotations::builtin_type_doc(name) {
        return Some(format!(
            "```lua
(type) {name}
```

---

{doc}"
        ));
    }
    if let Some(class) = Stubs::get().class(name).filter(|c| !c.name.starts_with("xmake.")) {
        let signature = format!(
            "```lua
(class) {name}
```"
        );
        return Some(match class.doc.as_str() {
            "" => signature,
            doc => format!(
                "{signature}

---

{doc}"
            ),
        });
    }
    let scope = project.scope_of(file);
    let files = std::iter::once(file).chain(scope.files().map(|(_, f)| &**f));
    files.into_iter().flat_map(|f| annotations::all(f.root())).find_map(|a| {
        let ("class" | "alias" | "enum") = a.tag else {
            return None;
        };
        let (declared, _, _) = a.name_and_rest()?;
        (declared.trim_end_matches(':') == name).then(|| {
            format!(
                "```lua
---@{} {}
```",
                a.tag, a.args
            )
        })
    })
}

/// A global name: an API, a scope function, a builtin function or module.
fn global_hover(project: &Project, file: &AnalyzedFile, token: SyntaxToken<'_>) -> Option<String> {
    let db = &project.db;
    let name = token.text();
    let context = CodeContext::at(file, token.text_range().start());
    if let CodeContext::Description(scope) = context {
        let api = db
            .visible_api(scope, name)
            .or_else(|| db.scopes_of(name).next().and_then(|s| db.scope_api(s, name)));
        if let Some(api) = api {
            let fallback =
                || render_signature("", name, api_params(name, Some(api.kind), api.scope));
            let doc: Cow<'_, str> = match api_doc(Some(api.scope), name) {
                Some(doc) => Cow::Borrowed(doc),
                None => Cow::Owned(generated_api_doc(api.scope, name, api.kind)),
            };
            let (signature, doc) = with_prototype(Some(api.scope), name, fallback, doc);
            let location = if api.scope == Scope::Root { "xmake" } else { api.scope.name() };
            return Some(page(location, &signature, &doc, doc_link(Some(api.scope), name)));
        }
        if let Some((scope, _)) = Scope::from_opener(name) {
            let fallback = || render_signature("", name, api_params(name, None, scope));
            let doc = Cow::Borrowed(api_doc(None, name).unwrap_or("Closes the current scope."));
            let (signature, doc) = with_prototype(Some(scope), name, fallback, doc);
            return Some(page("xmake", &signature, &doc, doc_link(Some(scope), name)));
        }
    }
    // Builtins typed by the stubs: `ipairs`, `tostring`, `import`, ...
    if let Some(function) = Stubs::get().member(xmake_api::GLOBALS, name) {
        return Some(stub_page("builtin", function));
    }
    let script = context.is_script();
    let is_function = if script {
        db.script_functions().iter().any(|f| f == name)
    } else {
        db.is_description_function(name)
    };
    if is_function {
        let fallback =
            || render_signature("", name, api_params(name, Some(ApiKind::Custom), Scope::Root));
        let doc = Cow::Borrowed(api_doc(None, name).unwrap_or("Builtin function."));
        let (signature, doc) = with_prototype(None, name, fallback, doc);
        return Some(page("xmake", &signature, &doc, doc_link(None, name)));
    }
    let Some(module) = db.module(name, script) else {
        // `function name() end` in the project.
        if let Some((other, statement)) = xmake_types::global_function(project, file, name) {
            return Some(function_page(project, other, statement, name));
        }
        return global_variable_hover(project, file, name);
    };
    let doc = format!(
        "Builtin module with {} functions: type `{name}.` to list them.",
        module.functions.len()
    );
    Some(page("xmake", &format!("module {name}"), &doc, None))
}

/// The signature of the manual (`set_default(default: boolean)`) instead of
/// the one guessed from the kind of the API, and its parameters documented.
fn with_prototype<'d>(
    scope: Option<Scope>,
    name: &str,
    fallback: impl FnOnce() -> String,
    doc: Cow<'d, str>,
) -> (String, Cow<'d, str>) {
    let Some((prototype, params)) = xmake_api::doc_prototype(scope, name) else {
        return (fallback(), doc);
    };
    // `on_build(script)` and `on_build(pattern, ..., script)`.
    let prototype = match xmake_types::script_overloads(prototype) {
        Some([alone, patterns]) => Cow::Owned(format!("{alone}\n{patterns}")),
        None => xmake_types::luacats_prototype(prototype),
    };
    if params.is_empty() || doc.contains("# Parameters") {
        return (prototype.into_owned(), doc);
    }
    // Before the examples, as rustdoc puts the arguments first.
    let mut parameters = String::from(
        "# Parameters
",
    );
    for (param, description) in &params {
        let _ = write!(
            parameters,
            "
- `{param}`: {description}"
        );
    }
    let doc = match doc.find(
        "
# Examples",
    ) {
        Some(at) => format!(
            "{}

{parameters}
{}",
            doc[..at].trim_end(),
            &doc[at..]
        ),
        None => format!(
            "{doc}

{parameters}"
        ),
    };
    (prototype.into_owned(), Cow::Owned(doc))
}

/// `(global) name: type` from the assignment of a global of the project.
fn global_variable_hover(project: &Project, file: &AnalyzedFile, name: &str) -> Option<String> {
    for (_, other) in project.scope_of(file).files() {
        for statement in other.root().descendants().filter(|n| n.kind() == SyntaxKind::ASSIGN_STMT)
        {
            let targets: Vec<_> =
                statement.children().take_while(|n| n.kind() != SyntaxKind::EXPR_LIST).collect();
            let Some(index) = targets
                .iter()
                .position(|t| t.kind() == SyntaxKind::NAME_REF && t.text().trim() == name)
            else {
                continue;
            };
            let values: Vec<_> = statement
                .child_node(SyntaxKind::EXPR_LIST)
                .map(|l| l.children().collect())
                .unwrap_or_default();
            let ty = crate::types::nth_value_type(project, other, &values, index);
            let signature = format!("(global) {name}: {ty}");
            return Some(match doc_comment(statement) {
                Some(doc) => format!(
                    "```lua
{signature}
```

---

{doc}"
                ),
                None => format!(
                    "```lua
{signature}
```"
                ),
            });
        }
    }
    None
}

/// `os.cp`, `json.encode` with `json` imported, ...: typed by the stubs.
fn field_hover(project: &Project, file: &AnalyzedFile, field: FieldExpr<'_>) -> Option<String> {
    let base = field.base()?;
    let func = field.field()?.text();
    let module = type_of(project, file, base).module()?;
    if let Some(function) = Stubs::get().member(module, func) {
        return Some(stub_page(module, function));
    }
    // `import("utils")`: a module of the project.
    if let Some(local) = crate::modules::local_module(project, file, module) {
        let functions = crate::modules::exported_functions(&local.file);
        if let Some((statement, name, _)) = functions.into_iter().find(|(_, n, _)| *n == func) {
            let location = local.path.display().to_string();
            let page = function_page(project, &local.file, statement, name);
            return Some(format!(
                "{location}

{page}"
            ));
        }
    }
    let script = CodeContext::at(file, base.text_range().start()).is_script();
    let known =
        project.db.module(module, script).is_some_and(|m| m.functions.iter().any(|f| f == func))
            || project
                .db
                .extension_apis()
                .iter()
                .any(|a| a.strip_prefix(module).and_then(|r| r.strip_prefix('.')) == Some(func));
    if !known {
        return None;
    }
    let doc = generated_member_doc(module, '.', func);
    Some(page(
        module,
        &format!("{}.{func}(...)", module.rsplit('.').next().unwrap_or(module)),
        &doc,
        None,
    ))
}

/// The documentation of a key of an API options table: `public` in
/// `add_includedirs("inc", {public = true})`, `shared` in the `configs` of
/// `add_requires`.
fn option_doc(
    project: &Project,
    field: xmake_syntax::ast::TableField<'_>,
    key: &str,
) -> Option<Cow<'static, str>> {
    use xmake_syntax::ast::{CallExpr, Literal, TableField};
    let table = field.syntax().parent().filter(|t| t.kind() == SyntaxKind::TABLE_EXPR)?;
    let parent = table.parent()?;
    let find = |keys: &'static [(&'static str, &'static str)]| {
        keys.iter().find(|(k, _)| *k == key).map(|(_, d)| Cow::Borrowed(*d))
    };
    let is_requires = |name: &str| matches!(name, "add_requires" | "add_requireconfs");
    if parent.kind() == SyntaxKind::ARG_LIST {
        let name = CallExpr::cast(parent.parent()?)?.callee_name()?;
        if is_requires(name) {
            return find(crate::packages::REQUIRE_OPTIONS);
        }
        let db = &project.db;
        let api = db.scopes_of(name).next().and_then(|s| db.scope_api(s, name));
        return find(xmake_api::option_keys(name, api.map(|a| a.kind)));
    }
    // `add_requires("zlib", {configs = {shared = true}})`.
    let configs =
        TableField::cast(parent).filter(|f| f.key_name().is_some_and(|k| k.text() == "configs"))?;
    let args = configs.syntax().parent()?.parent().filter(|a| a.kind() == SyntaxKind::ARG_LIST)?;
    let call =
        CallExpr::cast(args.parent()?).filter(|c| c.callee_name().is_some_and(is_requires))?;
    let package = call.arg(0).and_then(Literal::cast).and_then(|l| l.string_value());
    let details = package
        .as_deref()
        .and_then(|p| project.packages.get(crate::packages::split_requirement(p).0));
    let config = details
        .and_then(|e| e.details())
        .and_then(|d| d.configs.into_iter().find(|(c, _)| c == key));
    match config {
        Some((_, description)) => Some(Cow::Owned(description)),
        None => find(crate::packages::COMMON_CONFIGS),
    }
}

/// The table constructor an expression refers to: a local initialized with
/// a table, or a field of one.
fn table_of<'a>(file: &'a AnalyzedFile, expr: SyntaxNode<'a>) -> Option<SyntaxNode<'a>> {
    let value = match expr.kind() {
        SyntaxKind::NAME_REF => {
            let binding = resolve(file, expr.first_token()?)?;
            if binding.kind != BindingKind::Local {
                return None;
            }
            crate::types::initializer(file, binding.range)?
        }
        SyntaxKind::FIELD_EXPR => {
            let field = FieldExpr::cast(expr)?;
            let table = table_of(file, field.base()?)?;
            crate::tables::field(table, field.field()?.text())?.value()?
        }
        _ => return None,
    };
    (value.kind() == SyntaxKind::TABLE_EXPR).then_some(value)
}

/// `config.name` with `local config = {name = "app"}`.
fn table_field_hover(
    project: &Project,
    file: &AnalyzedFile,
    field: FieldExpr<'_>,
) -> Option<String> {
    let base = field.base()?;
    let key = field.field()?.text();
    let table = table_of(file, base)?;
    let entry = crate::tables::field(table, key)?;
    let path = format!("{}.{key}", base.text().trim());
    Some(format!(
        "```lua\n(field) {}\n```",
        crate::tables::describe_field(project, file, &path, entry)
    ))
}

/// `config[1]` and `config["k"]`.
fn index_hover(project: &Project, file: &AnalyzedFile, token: SyntaxToken<'_>) -> Option<String> {
    let index = token.parent()?.parent().filter(|n| n.kind() == SyntaxKind::INDEX_EXPR)?;
    let base = index.children().next()?;
    let table = table_of(file, base)?;
    let key = token.text().trim_matches(['"', '\'']);
    let entry = crate::tables::field(table, key)?;
    let path = format!("{}[{}]", base.text().trim(), token.text());
    Some(format!(
        "```lua\n(field) {}\n```",
        crate::tables::describe_field(project, file, &path, entry)
    ))
}

/// `target:name()`, `dep:targetfile()`, `s:split(",")`: typed by the stubs.
fn method_hover(
    project: &Project,
    file: &AnalyzedFile,
    call: MethodCallExpr<'_>,
) -> Option<String> {
    let owner = type_of(project, file, call.receiver()?).owner()?;
    let method = call.method()?.text();
    if let Some(function) = Stubs::get().member(owner, method) {
        return Some(stub_page(owner, function));
    }
    let instance = Instance::parse(owner)?;
    project.db.instance_methods(instance).iter().any(|m| m == method).then(|| {
        let doc = generated_member_doc(owner, ':', method);
        page(owner, &format!("{owner}:{method}(...)"), &doc, member_link(owner, ':', method))
    })
}

/// The page of a function of the stubs, with its parameters documented.
fn stub_page(owner: &str, function: &xmake_api::Function) -> String {
    let mut doc = function.doc.clone();
    let documented: Vec<_> = function.params.iter().filter(|p| !p.doc.is_empty()).collect();
    if !documented.is_empty() {
        doc.push_str("\n\n# Parameters\n");
        for param in documented {
            let _ = write!(doc, "\n- `{}`: {}", param.name, param.doc);
        }
    }
    if doc.is_empty() {
        doc = generated_member_doc(owner, if function.method { ':' } else { '.' }, function.name);
    }
    let separator = if function.method { ':' } else { '.' };
    let link = if owner == "builtin" || owner == xmake_api::GLOBALS {
        doc_link(None, function.name)
    } else {
        member_link(owner, separator, function.name)
    };
    page(owner, &format!("function {}", function.signature()), &doc, link)
}

/// A package of the repositories: description, versions, configurations.
fn package_hover(project: &Project, name: &str) -> Option<String> {
    let entry = project.packages.get(name)?;
    let details = entry.details().unwrap_or_default();
    let mut doc = details.description.clone().unwrap_or_default();
    let mut versions = details.versions.clone();
    versions.sort_by(|a, b| xmake_project::compare_versions(b, a));
    let _ = write!(
        doc,
        "

- Repository: `{}`",
        entry.repository
    );
    if let Some(homepage) = &details.homepage {
        let _ = write!(
            doc,
            "
- Homepage: <{homepage}>"
        );
    }
    if let Some(license) = &details.license {
        let _ = write!(
            doc,
            "
- License: {license}"
        );
    }
    if !versions.is_empty() {
        let shown: Vec<String> = versions.iter().take(8).map(|v| format!("`{v}`")).collect();
        let more = if versions.len() > 8 {
            format!(" (and {} more)", versions.len() - 8)
        } else {
            String::new()
        };
        let _ = write!(
            doc,
            "
- Versions: {}{more}",
            shown.join(", ")
        );
    }
    if !details.configs.is_empty() {
        let configs: Vec<String> = details.configs.iter().map(|(c, _)| format!("`{c}`")).collect();
        let _ = write!(
            doc,
            "
- Configs: {}",
            configs.join(", ")
        );
    }
    Some(page("package", &format!("add_requires(\"{name}\")"), doc.trim_start(), None))
}

/// A target, option, ... with the comment above its definition.
fn definition_hover(def: crate::Located<'_, crate::Definition>, value: &str) -> String {
    let file = def.file;
    let line = file.line_index.line_col(&file.text, def.item.name_range.start()).line + 1;
    let call = file.root().covering_node(def.item.call_range);
    let statement =
        std::iter::once(call).chain(call.ancestors()).find(|n| n.kind() == SyntaxKind::CALL_STMT);
    let mut doc = statement.and_then(doc_comment).map(Cow::into_owned).unwrap_or_default();
    if !doc.is_empty() {
        doc.push_str("\n\n");
    }
    let _ = write!(doc, "Defined in `{}:{line}`.", def.path.display());
    let scope = def.item.scope.name();
    page(scope, &format!("{scope}(\"{value}\")"), &doc, None)
}

/// The meaning of a value: `"binary"` in `set_kind("binary")`, a policy, ...
fn string_hover(project: &Project, file: &AnalyzedFile, token: SyntaxToken<'_>) -> Option<String> {
    let (call, index) = enclosing_call(token)?;
    let name = call.callee_name()?;
    let literal = xmake_syntax::ast::Literal::cast(token.parent()?)?;
    let value = literal.string_value()?;
    if name == "add_requires" {
        return package_hover(project, crate::packages::split_requirement(&value).0);
    }
    let values = xmake_project::call_arg_values(file, call, index);
    let defined_scope = match (Scope::from_opener(name), values) {
        (Some((scope, xmake_api::ScopeEdge::Begin)), _) if index == 0 => Some(scope),
        (_, Some(ArgValues::Targets)) => Some(Scope::Target),
        (_, Some(ArgValues::Options | ArgValues::Configs)) => Some(Scope::Option),
        (_, Some(ArgValues::Rules)) => Some(Scope::Rule),
        (_, Some(ArgValues::Toolchains)) => Some(Scope::Toolchain),
        _ => None,
    };
    let project_scope = project.scope_of(file);
    if let Some(scope) = defined_scope
        && let Some(def) = project_scope.definitions(scope).find(|d| d.name() == value)
    {
        return Some(definition_hover(def, &value));
    }
    let known = &project.db.values;
    let doc: &str = match values? {
        ArgValues::Fixed(list) => list.iter().find(|(v, _)| *v == value).map(|(_, d)| *d)?,
        ArgValues::Policies => &known.policies.iter().find(|p| p.name == value)?.description,
        ArgValues::Toolchains => &known.toolchains.iter().find(|t| t.name == value)?.description,
        ArgValues::Configs => xmake_api::builtin_configs().iter().find(|(n, _)| *n == value)?.1,
        _ => return None,
    };
    Some(format!("```lua\n\"{value}\"\n```\n\n---\n\n{doc}"))
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, sync::Arc};

    use xmake_api::ApiDb;
    use xmake_syntax::Dialect;

    use super::*;

    fn hover_at(text: &str) -> Option<String> {
        let offset = text.find('$').expect("marker");
        let text = text.replacen('$', "", 1);
        let mut project = Project::new(ApiDb::builtin(), Dialect::XMAKE_DEFAULT);
        let file = project.set_file(PathBuf::from("xmake.lua"), Arc::from(text.as_str()));
        hover(&project, &file, TextSize::new(offset as u32)).map(|h| h.markdown)
    }

    #[test]
    fn rustdoc_layout() {
        let api = hover_at("target('a')\n  set_k$ind('binary')").expect("api hover");
        // The prototype of the manual, not one guessed from the kind of the API.
        assert!(
            api.starts_with("```lua\ntarget\n```\n```lua\nset_kind(kind: string)\n```\n\n---\n\n"),
            "{api}"
        );
        let default = hover_at("target('a')\n  set_def$ault(false)").expect("set_default");
        assert!(
            default.contains("set_default(default: boolean)")
                && default.contains("# Parameters\n\n- `default`:"),
            "{default}"
        );
        assert!(api.contains("\n# Examples\n") && api.contains("\n# Notes\n"), "{api}");
        assert!(
            api.ends_with("(https://xmake.io/api/description/project-target.html#set-kind)"),
            "{api}"
        );
        let license = hover_at("target('a')\n  set_lic$ense('MIT')").expect("license");
        assert!(
            license
                .ends_with("(https://xmake.io/api/description/xpack-interfaces.html#set-license)"),
            "{license}"
        );
        let cp = hover_at("target('a')\non_load(function (t) os.c$p() end)").expect("cp");
        assert!(
            cp.ends_with("(https://xmake.io/api/scripts/builtin-modules/os.html#os-cp)"),
            "{cp}"
        );
        let method =
            hover_at("target('a')\non_load(function (t) t:target$file() end)").expect("method");
        assert!(
            method
                .ends_with("(https://xmake.io/api/scripts/target-instance.html#target-targetfile)"),
            "{method}"
        );

        // Without a hand written page, one is generated from the kind.
        let generated = hover_at("target('a')\n  set_prefix$name('lib')").expect("generated hover");
        assert!(generated.contains("# Examples"), "{generated}");
    }

    #[test]
    fn comment_docs_and_constants() {
        let local = hover_at("--- The protocol version.\n--- # Notes\n--- Bump it.\nlocal VERSION <const> = 3\nprint(VERS$ION)").expect("local");
        assert_eq!(
            local,
            "```lua\nlocal VERSION <const>: integer = 3\n```\n\n---\n\nThe protocol version.\n# Notes\nBump it."
        );

        let function =
            hover_at("-- Adds one.\nlocal function inc(x) return x + 1 end\nprint(in$c(1))")
                .expect("function");
        assert!(
            function.starts_with("```lua\nlocal function inc(x) -> number\n```")
                && function.ends_with("Adds one."),
            "{function}"
        );

        let target =
            hover_at("--- The core library.\ntarget('core')\ntarget('app')\n  add_deps('co$re')")
                .expect("target");
        assert!(target.contains("The core library.") && target.contains("Defined in"), "{target}");
    }

    #[test]
    fn variable_types() {
        let first_line =
            |text: &str| hover_at(text).map(|h| h.lines().nth(1).unwrap_or("").to_owned());
        assert_eq!(
            first_line(
                "local n = 1
print(n$)"
            )
            .as_deref(),
            Some("local n: integer")
        );
        assert_eq!(
            first_line(
                "local x
print(x$)"
            )
            .as_deref(),
            Some("local x: nil")
        );
        assert_eq!(
            first_line(
                "local f = function (a, b) end
print(f$)"
            )
            .as_deref(),
            Some("local f: fun(a, b)")
        );
        assert_eq!(
            first_line(
                "local V <const> = '3.1'
print(V$)"
            )
            .as_deref(),
            Some("local V <const>: string = '3.1'")
        );
        let script = "target('a')
on_load(function (t)
";
        let iorun = format!(
            "{script}  local out, err = os.iorun('git')
  print(er$r)
end)"
        );
        assert_eq!(first_line(&iorun).as_deref(), Some("local err: string"));
        let dep = format!(
            "{script}  local d = t:dep('x')
  print(d$)
end)"
        );
        assert_eq!(first_line(&dep).as_deref(), Some("local d: target?"));
        let files = format!(
            "{script}  for i, f in ipairs(os.files('*.c')) do print(i, f$) end
end)"
        );
        assert_eq!(first_line(&files).as_deref(), Some("for f: string"));
        let index = format!(
            "{script}  for i$ = 1, 3 do end
end)"
        );
        assert_eq!(first_line(&index).as_deref(), Some("for i: integer"));
        let build_file = "rule('r')
on_build_file(function (target, sourcefile, opt) print(source$file) end)";
        assert_eq!(first_line(build_file).as_deref(), Some("(parameter) sourcefile: string"));
        let global = "VERSION = '1.0'
print(VERS$ION)";
        assert_eq!(first_line(global).as_deref(), Some("(global) VERSION: string"));
    }

    #[test]
    fn option_keys() {
        let public = hover_at(
            "target('a')
    add_includedirs('include', {pub$lic = true})",
        )
        .expect("public");
        assert_eq!(
            public,
            "```lua
(field) public: boolean = true
```

---

Also for the targets depending on this one."
        );
        let force = hover_at(
            "target('a')
    add_cxflags('-O3', {fo$rce = true})",
        )
        .expect("force");
        assert!(force.contains("Do not check whether the compiler supports"), "{force}");
        let shared =
            hover_at("add_requires('zlib', {configs = {sha$red = true}})").expect("shared");
        assert!(shared.contains("Build a shared library."), "{shared}");
        let optional = hover_at("add_requires('zlib', {opt$ional = true})").expect("optional");
        assert!(optional.contains("Do not fail"), "{optional}");
    }

    #[test]
    fn tables() {
        let text = "local config = {name = 'app', 42, ['k'] = true, opts = {x = 1.5}}\n";
        let local = hover_at(&format!("{text}print(con$fig)")).expect("local");
        assert_eq!(
            local,
            "```lua\nlocal config: {\n    name: string = 'app',\n    [1]: integer = 42,\n    ['k']: boolean = true,\n    opts: table,\n}\n```"
        );
        let field = hover_at(&format!("{text}print(config.na$me)")).expect("field");
        assert_eq!(field, "```lua\n(field) config.name: string = 'app'\n```");
        let nested = hover_at(&format!("{text}print(config.opts.$x)")).expect("nested");
        assert_eq!(nested, "```lua\n(field) config.opts.x: number = 1.5\n```");
        let index = hover_at(&format!("{text}print(config[$1])")).expect("index");
        assert_eq!(index, "```lua\n(field) config[1]: integer = 42\n```");
        let key = hover_at("local t = {na$me = 'app'}").expect("key");
        assert_eq!(key, "```lua\n(field) name: string = 'app'\n```");
    }

    #[test]
    fn hovers() {
        let value = hover_at("target('a')\n  set_kind('bin$ary')").expect("value hover");
        assert!(value.contains("An executable program."), "{value}");

        let method = hover_at("target('a')\non_load(function (t) t:target$file() end)")
            .expect("method hover");
        assert!(
            method.contains("target:targetfile() -> string") && method.contains("# Examples"),
            "{method}"
        );

        let param =
            hover_at("target('a')\non_load(function (t) $t:name() end)").expect("param hover");
        assert!(param.contains("(parameter) t: target"), "{param}");

        let module =
            hover_at("target('a')\non_load(function (t) os.c$p() end)").expect("module hover");
        assert!(
            module.contains("os.cp(src: string, dst: string") && module.contains("# Parameters"),
            "{module}"
        );

        let typed = hover_at(
            "target('a')
on_load(function (t)
  local d = t:dep('x')
  print(d:target$file())
end)",
        );
        assert!(typed.is_some_and(|h| h.contains("target:targetfile() -> string")));
        let local = hover_at(
            "target('a')
on_load(function (t)
  local d = t:dep('x')
  print(d$)
end)",
        )
        .expect("local");
        assert!(local.contains("local d: target"), "{local}");

        let import = hover_at("target('a')\non_load(function (t) im$port('core.base.json') end)")
            .expect("import");
        assert!(
            import.contains("import(name: xmake.value.module, opt?: {alias?: string"),
            "{import}"
        );

        let ipairs = hover_at("for i, v in ipa$irs({}) do end").expect("ipairs");
        assert!(
            ipairs.contains("ipairs(t: V[]) -> fun(): integer, V") && ipairs.contains("# Examples"),
            "{ipairs}"
        );
    }
}

#[cfg(test)]
mod function_tests {
    use std::{path::PathBuf, sync::Arc};

    use xmake_api::ApiDb;
    use xmake_syntax::Dialect;

    use super::*;

    fn hover_at(text: &str) -> String {
        let offset = text.find('$').expect("marker");
        let text = text.replacen('$', "", 1);
        let mut project = Project::new(ApiDb::builtin(), Dialect::XMAKE_DEFAULT);
        let file = project.set_file(PathBuf::from("xmake.lua"), Arc::from(text.as_str()));
        hover(&project, &file, TextSize::new(offset as u32)).map_or(String::new(), |h| h.markdown)
    }

    #[test]
    fn functions() {
        let declared = hover_at("---@deprecated\nfunction t$()\n    return \"\"\nend\n");
        assert!(declared.starts_with("```lua\nfunction t() -> string\n```"), "{declared}");
        assert!(declared.contains("**Deprecated.**"), "{declared}");
        let called = hover_at("---@deprecated use u\nfunction t() return 1 end\nprint(t$())");
        assert!(called.contains("function t() -> integer"), "{called}");
        assert!(called.contains("**Deprecated:** use u"), "{called}");
        let typed = hover_at(
            "--- Copies.\n---@param src string\n---@param opt? table\n---@return boolean\nlocal function cp(src, opt) end\ncp$('a')",
        );
        assert!(
            typed
                .starts_with("```lua\nlocal function cp(src: string, opt?: table) -> boolean\n```"),
            "{typed}"
        );
        assert!(typed.ends_with("Copies."), "{typed}");
    }

    #[test]
    fn script_overloads() {
        let page = hover_at(
            "target('a')
  on_bu$ild(function (target) end)",
        );
        assert!(
            page.contains(
                "on_build(script: fun(target: target)|string)
on_build(pattern: string, ...: string, script: fun(target: target)|string)"
            ),
            "{page}"
        );
    }

    #[test]
    fn annotations() {
        let tag = hover_at("---@depre$cated\nfunction t() end");
        assert!(tag.starts_with("```lua\n---@deprecated\n```"), "{tag}");
        let builtin = hover_at("---@param s str$ing\nlocal function f(s) end");
        assert!(builtin.starts_with("```lua\n(type) string\n```"), "{builtin}");
        let class = hover_at("---@type tar$get\nlocal t");
        assert!(class.starts_with("```lua\n(class) target\n```"), "{class}");
        let own = hover_at("---@class Point: table\n---@type Poi$nt\nlocal p = {}");
        assert_eq!(own, "```lua\n---@class Point: table\n```");
    }
}
