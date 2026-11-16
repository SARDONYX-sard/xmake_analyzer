//! Completion inside the annotations of LuaCATS, as LuaLS reads them:
//!
//! ```lua
//! ---@|                      -- the tags: param, return, type, class, ...
//! ---@param |                -- the parameters of the function below
//! ---@param name str|        -- the types: builtins, classes of the stubs,
//! ---@return target|         --   `@class` and `@alias` of the file
//! local function f(name) end
//! ```

use std::borrow::Cow;

use xmake_syntax::{SyntaxKind, SyntaxNode, SyntaxToken, TextRange, TextSize};

use super::{CompletionItem, CompletionKind, Items, group};
use crate::AnalyzedFile;

/// The tags: label, snippet, documentation.
const TAGS: &[(&str, &str, &str)] = &[
    (
        "alias",
        "alias ${1:Name} ${2:type}",
        "A name for a type: `---@alias Mode \"debug\"|\"release\"`.",
    ),
    ("as", "as ${1:type}", "Casts an expression inline: `--[[@as string]]`."),
    ("async", "async", "The function yields."),
    ("cast", "cast ${1:name} ${2:type}", "Changes the type of a variable from here on."),
    ("class", "class ${1:Name}", "Declares a class: `---@class Point: Shape`."),
    ("deprecated", "deprecated", "The function or field should not be used anymore."),
    ("diagnostic", "diagnostic ${1:disable-next-line}", "Controls the diagnostics of LuaLS."),
    ("enum", "enum ${1:Name}", "The table below lists the values of a type."),
    ("field", "field ${1:name} ${2:type}", "A field of the class above: `---@field name string`."),
    ("generic", "generic ${1:T}", "A type parameter of the function below."),
    ("meta", "meta", "The file only declares types: no code runs."),
    ("module", "module ${1:name}", "The file is the module `name`."),
    ("nodiscard", "nodiscard", "The returned values must be used."),
    (
        "operator",
        "operator ${1:add}(${2:type}): ${3:type}",
        "The type of an operator of the class.",
    ),
    ("overload", "overload fun(${1}): ${2}", "Another signature of the function below."),
    ("package", "package", "The field is visible in its file only."),
    (
        "param",
        "param ${1:name} ${2:type}",
        "A parameter of the function below: `---@param path string`.",
    ),
    ("private", "private", "The field is visible in its class only."),
    ("protected", "protected", "The field is visible in its class and subclasses only."),
    ("return", "return ${1:type}", "A value the function below returns: `---@return string?`."),
    ("see", "see ${1:name}", "A reference to another symbol."),
    ("source", "source ${1:path}", "Where the definition comes from."),
    ("type", "type ${1:type}", "The type of the variable below: `---@type string[]`."),
    ("vararg", "vararg ${1:type}", "The type of `...`; prefer `---@param ... type`."),
    ("version", "version ${1:5.4}", "The Lua versions the definition exists in."),
];

/// The types of Lua.
const BUILTIN_TYPES: &[(&str, &str)] = &[
    ("any", "Any value; not checked."),
    ("boolean", "`true` or `false`."),
    ("false", "The value `false`."),
    ("fun", "A function type: `fun(a: string, b?: integer): boolean`."),
    ("function", "Any function."),
    ("integer", "A number with an integral value (a subtype since Lua 5.3)."),
    ("lightuserdata", "A C pointer."),
    ("nil", "No value."),
    ("number", "A number, integer or float."),
    ("string", "A string; its methods are those of the `string` module."),
    ("table", "Any table; `table<K, V>` for a map, `T[]` for a list."),
    ("thread", "A coroutine."),
    ("true", "The value `true`."),
    ("unknown", "A value not known yet."),
    ("userdata", "A value of C."),
];

/// The documentation of the tag `@tag`.
pub(crate) fn tag_doc(tag: &str) -> Option<&'static str> {
    TAGS.iter().find(|t| t.0 == tag).map(|t| t.2)
}

/// The documentation of the builtin type `name`.
pub(crate) fn builtin_type_doc(name: &str) -> Option<&'static str> {
    BUILTIN_TYPES.iter().find(|t| t.0 == name).map(|t| t.1)
}

/// The tags whose first argument is a name, not a type.
const NAMED_TAGS: &[&str] = &["alias", "cast", "class", "enum", "field", "generic", "param"];

/// The tags taking types.
const TYPED_TAGS: &[&str] =
    &["alias", "cast", "class", "field", "overload", "param", "return", "type", "vararg"];

/// The completions at `offset` in the line comment `token`, when it is an
/// annotation (`---@...`); `None` for other comments.
pub(super) fn completions<'a>(
    file: &'a AnalyzedFile,
    token: SyntaxToken<'a>,
    offset: TextSize,
) -> Option<Vec<CompletionItem<'a>>> {
    if token.kind() != SyntaxKind::LINE_COMMENT {
        return None;
    }
    let before = &file.text[TextRange::new(token.text_range().start(), offset)];
    let annotation = before.strip_prefix("---")?.trim_start().strip_prefix('@')?;
    let typed = annotation
        .bytes()
        .rev()
        .take_while(|&b| b == b'_' || b == b'.' || b.is_ascii_alphanumeric())
        .count();
    let replace = TextRange::new(offset - TextSize::new(typed as u32), offset);
    let mut items = Items::new(&file.text[replace], replace);

    // `---@pa|`: the tags.
    let Some((tag, args)) = annotation.split_once(char::is_whitespace) else {
        for &(label, snippet, doc) in TAGS {
            if let Some(item) = items.add(label, CompletionKind::Keyword, group::KEYWORD) {
                item.snippet = Some(Cow::Borrowed(snippet));
                item.documentation = Some(Cow::Borrowed(doc));
            }
        }
        return Some(items.items);
    };
    let args = args.trim_start();
    // The words written before the one being typed.
    let written = args[..args.len() - typed].split_whitespace().count();
    let naming = written == 0 && NAMED_TAGS.contains(&tag);
    if naming && tag == "param" {
        param_names(token, &mut items);
    } else if tag == "class" {
        // `---@class Name: Parent`
        if args.contains(':') {
            type_names(file, &mut items);
        }
    } else if !naming && TYPED_TAGS.contains(&tag) {
        type_names(file, &mut items);
    }
    Some(items.items)
}

/// The parameters of the function the annotation documents.
fn param_names<'a>(comment: SyntaxToken<'a>, items: &mut Items<'a>) {
    let Some(params) = documented_function(comment)
        .and_then(|f| f.descendants().find(|n| n.kind() == SyntaxKind::PARAM_LIST))
    else {
        return;
    };
    for name in params.children().filter_map(|n| n.first_token()) {
        if let Some(item) = items.add(name.text(), CompletionKind::Variable, group::LOCAL) {
            item.detail = Some(Cow::Borrowed("parameter"));
        }
    }
    if params.child_token(SyntaxKind::DOT3).is_some() {
        items.add("...", CompletionKind::Variable, group::LOCAL);
    }
}

/// The statement right after the comment block of `comment`, when it
/// declares a function: `function f()`, `local function f()`,
/// `local f = function () end`.
fn documented_function(comment: SyntaxToken<'_>) -> Option<SyntaxNode<'_>> {
    let mut next = comment.next_token();
    while let Some(token) = next.filter(|t| t.kind().is_trivia()) {
        next = token.next_token();
    }
    let first = next?;
    let statement = first.ancestors().find(|n| {
        matches!(
            n.kind(),
            SyntaxKind::FUNCTION_STMT
                | SyntaxKind::LOCAL_FUNCTION_STMT
                | SyntaxKind::LOCAL_STMT
                | SyntaxKind::ASSIGN_STMT
        )
    })?;
    (statement.trimmed_range().start() == first.text_range().start()).then_some(statement)
}

/// The builtin types, the classes of the stubs, and the classes and aliases
/// the file declares.
fn type_names<'a>(file: &'a AnalyzedFile, items: &mut Items<'a>) {
    for &(name, doc) in BUILTIN_TYPES {
        if let Some(item) = items.add(name, CompletionKind::Keyword, group::KEYWORD) {
            item.documentation = Some(Cow::Borrowed(doc));
            if name == "fun" {
                item.snippet = Some(Cow::Borrowed("fun(${1}): ${2}"));
            }
        }
    }
    for (name, kind) in declared_types(&file.text) {
        if let Some(item) = items.add(name, CompletionKind::Value, group::LOCAL) {
            item.detail = Some(Cow::Borrowed(kind));
        }
    }
    // The descriptions APIs are classes of the stubs, not types of values.
    let stubs = xmake_api::Stubs::get().classes();
    for class in stubs.filter(|c| !c.name.starts_with("xmake.") && c.name != xmake_api::GLOBALS) {
        if let Some(item) = items.add(class.name, CompletionKind::Module, group::VALUE) {
            item.detail = Some(Cow::Borrowed("class"));
            item.documentation = (!class.doc.is_empty()).then_some(Cow::Borrowed(&*class.doc));
        }
    }
}

/// `Name` of `---@class Name: Parent`, `---@alias Name type` and
/// `---@enum Name` in `text`, with the tag.
fn declared_types(text: &str) -> impl Iterator<Item = (&str, &'static str)> {
    text.lines().filter_map(|line| {
        let annotation = line.trim_start().strip_prefix("---")?.trim_start().strip_prefix('@')?;
        let (tag, rest) = annotation.split_once(char::is_whitespace)?;
        let kind = match tag {
            "class" => "class",
            "alias" => "alias",
            "enum" => "enum",
            _ => return None,
        };
        let rest = rest.trim_start();
        let end = rest
            .find(|c: char| !(c == '_' || c == '.' || c.is_ascii_alphanumeric()))
            .unwrap_or(rest.len());
        (end > 0).then(|| (&rest[..end], kind))
    })
}
