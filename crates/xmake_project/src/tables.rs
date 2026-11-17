//! Table constructors: their fields and keys.

use std::borrow::Cow;

use xmake_syntax::{
    SyntaxKind, SyntaxNode,
    ast::{AstNode, TableExpr, TableField},
};

/// The key of a field: `name`, `[1]` (positional) or `["k"]`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Key<'a> {
    Name(&'a str),
    Index(usize),
    Expr(&'a str),
}

impl Key<'_> {
    pub fn display(&self) -> Cow<'_, str> {
        match self {
            Key::Name(name) => Cow::Borrowed(name),
            Key::Index(i) => Cow::Owned(format!("[{i}]")),
            Key::Expr(text) => Cow::Owned(format!("[{text}]")),
        }
    }
}

/// The fields of a table constructor with their keys; positional fields
/// count from 1.
pub fn fields(table: SyntaxNode<'_>) -> Vec<(Key<'_>, TableField<'_>)> {
    let Some(table) = TableExpr::cast(table) else {
        return Vec::new();
    };
    let mut index = 0;
    table
        .fields()
        .map(|field| {
            let key = if let Some(name) = field.key_name() {
                Key::Name(name.text())
            } else if field.syntax().child_token(SyntaxKind::L_BRACK).is_some() {
                let key = field.syntax().children().next().map_or("", |k| k.text().trim());
                Key::Expr(key)
            } else {
                index += 1;
                Key::Index(index)
            };
            (key, field)
        })
        .collect()
}

/// The field of `table` named `key` (`name` or `1`).
pub fn field<'a>(table: SyntaxNode<'a>, key: &str) -> Option<TableField<'a>> {
    fields(table).into_iter().find_map(|(k, field)| {
        let matches = match k {
            Key::Name(name) => name == key,
            Key::Index(i) => key.parse::<usize>().ok() == Some(i),
            Key::Expr(text) => text.trim_matches(['"', '\'']) == key,
        };
        matches.then_some(field)
    })
}
