//! Table constructors: their keys, the types and values of their fields.
//!
//! ```lua
//! local config = {name = "app", 42, ["k"] = true}
//! -- hover `config`:      local config: { name: string = "app", [1]: integer = 42, ["k"]: boolean = true }
//! -- hover `config.name`: (field) config.name: string = "app"
//! ```

use std::fmt::Write as _;

pub(crate) use xmake_project::tables::field;
use xmake_project::tables::fields;
use xmake_syntax::{SyntaxKind, SyntaxNode, ast::TableField};
use xmake_types::type_of;

use crate::{AnalyzedFile, Project};

/// The type of a value: `string`, `integer`, `fun(a, b)`, `target`, ...
pub(crate) fn value_type<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    value: SyntaxNode<'a>,
) -> xmake_types::Type<'a> {
    // A field holding `true` is a `boolean`.
    type_of(project, file, value).widen()
}

/// A short rendering of a value: literals as written, others elided.
fn preview(value: SyntaxNode<'_>) -> Option<&str> {
    let text = value.text().trim();
    (value.kind() == SyntaxKind::LITERAL && text.len() <= 40 && !text.contains('\n'))
        .then_some(text)
}

/// `key: type = value`.
pub(crate) fn describe_field(
    project: &Project,
    file: &AnalyzedFile,
    key: &str,
    field: TableField<'_>,
) -> String {
    let Some(value) = field.value() else {
        return format!("{key}: any");
    };
    let ty = value_type(project, file, value);
    match preview(value) {
        Some(text) => format!("{key}: {ty} = {text}"),
        None => format!("{key}: {ty}"),
    }
}

/// `{ name: string = "app", [1]: integer = 42 }`, one field per line.
pub(crate) fn shape(project: &Project, file: &AnalyzedFile, table: SyntaxNode<'_>) -> String {
    const MAX_FIELDS: usize = 32;
    let fields = fields(table);
    if fields.is_empty() {
        return "{}".to_owned();
    }
    let mut out = String::from("{\n");
    for (key, field) in fields.iter().take(MAX_FIELDS) {
        let _ = writeln!(out, "    {},", describe_field(project, file, &key.display(), *field));
    }
    if fields.len() > MAX_FIELDS {
        let _ = writeln!(out, "    -- {} more", fields.len() - MAX_FIELDS);
    }
    out.push('}');
    out
}
