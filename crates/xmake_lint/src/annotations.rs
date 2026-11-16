//! Mistakes in the annotations of LuaCATS:
//!
//! ```lua
//! ---@parm path string     -- invalid-annotation: unknown tag, `param`?
//! ---@param path strng     -- unknown-type: `string`?
//! ---@param pth string     -- unknown-param: `path`?
//! ---@return               -- invalid-annotation: missing type
//! local function f(path) end
//! ```

use std::borrow::Cow;

use rustc_hash::FxHashSet;
use xmake_project::annotations::{
    self, Annotation, TAGS, documented_statement, function_params, param_names, type_part,
};
use xmake_syntax::TextRange;

use crate::{AnalyzedFile, Diagnostic, Fix, Project, Severity};

/// The names of types every annotation may use.
const BUILTIN_TYPES: &[&str] = &[
    "any",
    "array",
    "bool",
    "boolean",
    "false",
    "fun",
    "function",
    "integer",
    "lightuserdata",
    "nil",
    "number",
    "self",
    "string",
    "table",
    "thread",
    "true",
    "unknown",
    "userdata",
    "void",
];

/// The tags followed by a type, after a name for some.
enum Shape {
    /// `---@type T`, `---@return T name`.
    Type,
    /// `---@param name T`, `---@field name T`, `---@alias Name T`.
    NameThenType,
    /// `---@class Name: Parent`.
    Class,
    /// Free text or nothing to check.
    Other,
}

fn shape(tag: &str) -> Shape {
    match tag {
        "type" | "return" | "vararg" | "overload" | "as" => Shape::Type,
        "param" | "field" | "alias" | "cast" => Shape::NameThenType,
        "class" => Shape::Class,
        _ => Shape::Other,
    }
}

/// Checks the annotations of `file`, and the calls of deprecated functions.
pub(crate) fn check<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    out: &mut Vec<Diagnostic<'a>>,
) {
    check_annotations(project, file, out);
    deprecated_calls(project, file, out);
}

fn check_annotations<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    out: &mut Vec<Diagnostic<'a>>,
) {
    let annotations: Vec<_> = annotations::all(file.root()).collect();
    if annotations.is_empty() {
        return;
    }
    let known = known_types(project, file);
    for annotation in &annotations {
        if !TAGS.contains(&annotation.tag) {
            let fix = crate::fuzzy::closest(annotation.tag, TAGS.iter().copied())
                .map(|t| Fix::replace(annotation.tag_range, t));
            push(
                out,
                "invalid-annotation",
                annotation.tag_range,
                format!("unknown annotation `@{}`", annotation.tag),
                fix,
            );
            continue;
        }
        let ty = match shape(annotation.tag) {
            Shape::Type => {
                if annotation.args.is_empty() {
                    missing_type(out, annotation);
                    continue;
                }
                annotation.args
            }
            Shape::NameThenType => {
                let Some((name, range, rest)) = annotation.name_and_rest() else {
                    push(
                        out,
                        "invalid-annotation",
                        annotation.tag_range,
                        format!("`@{}` needs a name and a type", annotation.tag),
                        None,
                    );
                    continue;
                };
                if annotation.tag == "param" {
                    check_param(annotation, name, range, out);
                }
                if rest.is_empty() {
                    missing_type(out, annotation);
                    continue;
                }
                rest
            }
            Shape::Class => match annotation.args.split_once(':') {
                Some((_, parent)) if !parent.trim().is_empty() => parent.trim(),
                _ => continue,
            },
            Shape::Other => continue,
        };
        check_type(annotation, type_part(ty), &known, out);
    }
}

/// `f()` of a function annotated `---@deprecated`.
fn deprecated_calls<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    out: &mut Vec<Diagnostic<'a>>,
) {
    for call in
        file.root().descendants().filter(|n| n.kind() == xmake_syntax::SyntaxKind::CALL_EXPR)
    {
        let Some((_, statement)) = xmake_types::called_declaration(project, file, call) else {
            continue;
        };
        let Some(deprecated) =
            annotations::above(statement).into_iter().find(|a| a.tag == "deprecated")
        else {
            continue;
        };
        let Some(callee) = call.children().next() else {
            continue;
        };
        let name = callee.text().trim();
        let message = match deprecated.args {
            "" => format!("`{name}` is deprecated"),
            reason => format!("`{name}` is deprecated: {reason}"),
        };
        push(out, "deprecated-function", callee.trimmed_range(), message, None);
    }
}

fn push<'a>(
    out: &mut Vec<Diagnostic<'a>>,
    code: &'static str,
    range: TextRange,
    message: String,
    fix: Option<Fix<'a>>,
) {
    out.push(Diagnostic {
        range,
        severity: Severity::Warning,
        message: Cow::Owned(message),
        code,
        fixes: fix.into_iter().collect(),
    });
}

fn missing_type<'a>(out: &mut Vec<Diagnostic<'a>>, annotation: &Annotation<'a>) {
    let message = format!("`@{}` without a type", annotation.tag);
    push(out, "invalid-annotation", annotation.tag_range, message, None);
}

/// `---@param name` of a parameter the function below has.
fn check_param<'a>(
    annotation: &Annotation<'a>,
    name: &'a str,
    range: TextRange,
    out: &mut Vec<Diagnostic<'a>>,
) {
    let Some(params) = documented_statement(annotation.comment).and_then(function_params) else {
        return;
    };
    let name = name.trim_end_matches('?');
    let names: Vec<&str> = param_names(params).map(|t| t.text()).collect();
    if names.contains(&name) {
        return;
    }
    let range = TextRange::at(range.start(), xmake_syntax::TextSize::new(name.len() as u32));
    let fix = crate::fuzzy::closest(name, names.iter().copied()).map(|n| Fix::replace(range, n));
    let message = format!("the function has no parameter `{name}`");
    push(out, "unknown-param", range, message, fix);
}

/// Balanced brackets, and names of known types.
fn check_type<'a>(
    annotation: &Annotation<'a>,
    ty: &'a str,
    known: &FxHashSet<&'a str>,
    out: &mut Vec<Diagnostic<'a>>,
) {
    if let Some(bracket) = unbalanced(ty) {
        let message = format!("unbalanced `{bracket}` in the type `{ty}`");
        push(out, "invalid-annotation", annotation.range_of(ty), message, None);
        return;
    }
    for name in type_names(ty) {
        if known.contains(name) {
            continue;
        }
        let range = annotation.range_of(name);
        let fix =
            crate::fuzzy::closest(name, known.iter().copied()).map(|c| Fix::replace(range, c));
        push(out, "unknown-type", range, format!("unknown type `{name}`"), fix);
    }
}

/// The first bracket without its pair.
fn unbalanced(ty: &str) -> Option<char> {
    let mut stack = Vec::new();
    for c in ty.chars() {
        match c {
            '(' | '{' | '[' | '<' => stack.push(c),
            ')' | '}' | ']' | '>' => {
                let open = match c {
                    ')' => '(',
                    '}' => '{',
                    ']' => '[',
                    _ => '<',
                };
                if stack.pop() != Some(open) {
                    return Some(c);
                }
            }
            _ => {}
        }
    }
    stack.pop()
}

/// The names of types in a type: `string`, `target` of
/// `fun(name: string): target?`; the names of parameters and fields, the
/// literals and the numbers left out.
fn type_names(ty: &str) -> Vec<&str> {
    let bytes = ty.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if matches!(b, b'"' | b'\'' | b'`') {
            // A literal type: `"debug"`.
            i += 1;
            while i < bytes.len() && bytes[i] != b {
                i += 1;
            }
            i += 1;
            continue;
        }
        if b.is_ascii_digit() {
            while i < bytes.len() && bytes[i].is_ascii_alphanumeric() {
                i += 1;
            }
            continue;
        }
        if !(b == b'_' || b.is_ascii_alphabetic()) {
            i += 1;
            continue;
        }
        let start = i;
        while i < bytes.len()
            && (bytes[i] == b'_' || bytes[i] == b'.' || bytes[i].is_ascii_alphanumeric())
        {
            i += 1;
        }
        let name = ty[start..i].trim_end_matches('.');
        // `name:` and `name?:` are parameters and fields.
        let after = ty[i..].trim_start();
        let is_key = after.starts_with(':') || after.starts_with("?:");
        // `fun(...)` is the syntax of function types.
        let is_fun = name == "fun" && after.starts_with('(');
        if !is_key && !is_fun {
            out.push(name);
        }
    }
    out
}

/// The builtin types, the classes of the stubs, and the classes, aliases,
/// enums and generics the project declares.
fn known_types<'a>(project: &'a Project, file: &'a AnalyzedFile) -> FxHashSet<&'a str> {
    let mut known: FxHashSet<&'a str> = BUILTIN_TYPES.iter().copied().collect();
    known.extend(xmake_api::Stubs::get().classes().map(|c| c.name));
    let files = project.files().map(|(_, f)| &**f).chain(std::iter::once(file));
    for annotation in files.flat_map(|f| annotations::all(f.root())) {
        match annotation.tag {
            "class" | "alias" | "enum" => {
                if let Some((name, _, _)) = annotation.name_and_rest() {
                    known.insert(name.trim_end_matches(':'));
                }
            }
            // `---@generic T, K: string`
            "generic" => {
                for part in annotation.args.split(',') {
                    let name = part.split(':').next().unwrap_or("").trim();
                    if !name.is_empty() {
                        known.insert(name);
                    }
                }
            }
            _ => {}
        }
    }
    known
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, sync::Arc};

    use xmake_api::ApiDb;
    use xmake_syntax::Dialect;

    use super::*;

    fn check_text(text: &str) -> Vec<String> {
        let mut project = Project::new(ApiDb::builtin(), Dialect::XMAKE_DEFAULT);
        let file = project.set_file(PathBuf::from("xmake.lua"), Arc::from(text));
        let mut out = Vec::new();
        check(&project, &file, &mut out);
        out.iter()
            .map(|d| {
                let fix =
                    d.fixes.first().map_or(String::new(), |f| format!(" -> {}", f.replacement));
                format!("{} {} [{}]{fix}", &text[d.range], d.message, d.code)
            })
            .collect()
    }

    #[test]
    fn mistakes() {
        let text = "---@class Point
---@field x number

---@parm path string
---@param pth strng
---@param opt? {force?: boolean, mode: \"a\"|\"b\"}
---@return
---@return fun(p: Point): table<string, target
local function f(path, opt) end

---@generic T
---@param list T[]
---@return T
local function first(list) end

---@type Pointt
local p = {}

---@deprecated use first
local function old() end
old()
";
        assert_eq!(
            check_text(text),
            [
                "parm unknown annotation `@parm` [invalid-annotation] -> param",
                "pth the function has no parameter `pth` [unknown-param] -> path",
                "strng unknown type `strng` [unknown-type] -> string",
                "return `@return` without a type [invalid-annotation]",
                "fun(p: Point): table<string, target unbalanced `<` in the type `fun(p: Point): table<string, target` [invalid-annotation]",
                "Pointt unknown type `Pointt` [unknown-type] -> Point",
                "old `old` is deprecated: use first [deprecated-function]",
            ]
        );
    }
}
