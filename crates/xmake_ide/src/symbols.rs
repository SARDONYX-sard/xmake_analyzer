//! Document symbols, workspace symbols and folding ranges.

use std::path::Path;

use xmake_api::Scope;
use xmake_syntax::{
    SyntaxKind, TextRange, ast::AstNode, ast::FunctionStmt, ast::LocalFunctionStmt,
};

use crate::{AnalyzedFile, Project};

/// What a symbol is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SymbolKind {
    /// A scope definition: target, option, ...
    Scope(Scope),
    Function,
}

/// A symbol of a file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Symbol<'a> {
    pub name: &'a str,
    pub kind: SymbolKind,
    /// The whole item.
    pub range: TextRange,
    /// The name.
    pub selection: TextRange,
}

/// The definitions and functions of `file`, in source order.
pub fn document_symbols(file: &AnalyzedFile) -> Vec<Symbol<'_>> {
    let text = &*file.text;
    let mut out: Vec<Symbol<'_>> = file
        .outline
        .definitions
        .iter()
        .map(|d| Symbol {
            name: &text[d.name_range],
            kind: SymbolKind::Scope(d.scope),
            range: d.full_range,
            selection: d.name_range,
        })
        .collect();

    for node in file.root().descendants() {
        let name_token = if let Some(f) = FunctionStmt::cast(node) {
            // `a.b:c`: show the whole name, select the last part.
            let mut tokens = f.name_tokens();
            let first = tokens.next();
            let last = tokens.last().or(first);
            match (first, last) {
                (Some(first), Some(last)) => {
                    let full = first.text_range().cover(last.text_range());
                    Some((&text[full], last.text_range()))
                }
                _ => None,
            }
        } else if let Some(f) = LocalFunctionStmt::cast(node) {
            f.name().and_then(|n| n.ident()).map(|t| (t.text(), t.text_range()))
        } else {
            None
        };
        if let Some((name, selection)) = name_token {
            let range = node.text_range();
            out.push(Symbol { name, kind: SymbolKind::Function, range, selection });
        }
    }
    out.sort_by_key(|s| s.range.start());
    out
}

/// Project symbols whose name contains `query` (case-insensitive).
pub fn workspace_symbols<'a>(project: &'a Project, query: &str) -> Vec<(&'a Path, Symbol<'a>)> {
    let query = query.to_ascii_lowercase();
    let mut out = Vec::new();
    for (path, file) in project.files() {
        for symbol in document_symbols(file) {
            if symbol.name.to_ascii_lowercase().contains(&query) {
                out.push((path, symbol));
            }
        }
    }
    out
}

/// The tasks of `file` runnable from the command line, `xmake <name>`: those
/// with a `set_menu`, without which xmake does not know the command.
pub fn runnable_tasks(file: &AnalyzedFile) -> Vec<(TextRange, &str)> {
    let root = file.root();
    let menus: Vec<TextRange> = root
        .descendants()
        .filter_map(xmake_syntax::ast::CallExpr::cast)
        .filter(|call| call.callee_name() == Some("set_menu"))
        .map(|call| call.syntax().text_range())
        .collect();
    file.outline
        .definitions
        .iter()
        .filter(|d| d.scope == Scope::Task)
        .filter(|d| menus.iter().any(|m| d.full_range.contains_range(*m)))
        .map(|d| (d.name_range, &file.text[d.name_range]))
        .collect()
}

/// A foldable region.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fold {
    pub range: TextRange,
    pub is_comment: bool,
}

/// Blocks, tables, long comments and scope definitions.
pub fn folding_ranges(file: &AnalyzedFile) -> Vec<Fold> {
    let mut out = Vec::new();
    for element in file.root().descendants_with_tokens() {
        let kind = element.kind();
        let foldable = matches!(
            kind,
            SyntaxKind::FUNCTION_EXPR
                | SyntaxKind::FUNCTION_STMT
                | SyntaxKind::LOCAL_FUNCTION_STMT
                | SyntaxKind::IF_STMT
                | SyntaxKind::DO_STMT
                | SyntaxKind::WHILE_STMT
                | SyntaxKind::REPEAT_STMT
                | SyntaxKind::NUMERIC_FOR_STMT
                | SyntaxKind::GENERIC_FOR_STMT
                | SyntaxKind::TABLE_EXPR
                | SyntaxKind::BLOCK_COMMENT
                | SyntaxKind::LONG_STRING
        );
        if foldable {
            let range = match element {
                xmake_syntax::SyntaxElement::Node(n) => n.trimmed_range(),
                xmake_syntax::SyntaxElement::Token(t) => t.text_range(),
            };
            out.push(Fold { range, is_comment: kind == SyntaxKind::BLOCK_COMMENT });
        }
    }
    for def in &file.outline.definitions {
        // Up to the last significant token of the scope.
        let range = file.root().covering_node(def.full_range);
        let end = range
            .descendant_tokens()
            .filter(|t| !t.kind().is_trivia() && t.text_range().end() <= def.full_range.end())
            .last()
            .map_or(def.full_range.end(), |t| t.text_range().end());
        out.push(Fold { range: TextRange::new(def.call_range.start(), end), is_comment: false });
    }
    out
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use xmake_syntax::Dialect;

    use super::*;

    #[test]
    fn symbols() {
        let text = "target('app')\n  set_kind('binary')\nfunction helper() end\nlocal function util() end\noption('o')\n";
        let file = AnalyzedFile::new(Arc::from(text), "xmake.lua", Dialect::XMAKE_DEFAULT);
        let names: Vec<_> = document_symbols(&file).iter().map(|s| s.name).collect();
        assert_eq!(names, ["app", "helper", "util", "o"]);
        assert!(!folding_ranges(&file).is_empty());

        let tasks = "task('hello')
  set_menu {usage = 'xmake hello'}
  on_run(function () end)
task('quiet')
  on_run(function () end)
";
        let file = AnalyzedFile::new(Arc::from(tasks), "xmake.lua", Dialect::XMAKE_DEFAULT);
        let names: Vec<_> = runnable_tasks(&file).into_iter().map(|(_, n)| n).collect();
        assert_eq!(names, ["hello"]);
    }
}
