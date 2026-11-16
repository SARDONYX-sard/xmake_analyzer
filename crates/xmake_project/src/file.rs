//! A parsed and outlined file.

use std::sync::Arc;

use xmake_syntax::{Dialect, Parse, SyntaxKind, SyntaxNode, ast::AstNode, ast::CallExpr, parse};

use crate::{line_index::LineIndex, outline::Outline};

/// The context the top level of a file runs in.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FileKind {
    /// `xmake.lua` and the files it includes: `target(...)`, `set_kind(...)`.
    Description,
    /// Modules, plugins, tasks: `import(...)`, `os.cp(...)`, `function main()`.
    Script,
}

impl FileKind {
    /// Guesses the kind of `file_name` with contents `root`: `xmake.lua` is a
    /// description, and so is a file calling a scope function at its top
    /// level (an included file); anything else is a script.
    fn detect(file_name: &str, root: SyntaxNode<'_>) -> FileKind {
        if file_name.eq_ignore_ascii_case("xmake.lua") {
            return FileKind::Description;
        }
        let top_level_calls = root
            .children()
            .filter(|n| n.kind() == SyntaxKind::BLOCK)
            .flat_map(|b| b.children())
            .filter(|s| s.kind() == SyntaxKind::CALL_STMT)
            .filter_map(|s| s.children().find_map(CallExpr::cast));
        let mut names = top_level_calls.filter_map(|c| c.callee_name());
        let describes = names.any(|n| {
            xmake_api::Scope::from_opener(n).is_some()
                || matches!(n, "includes" | "add_requires" | "set_project" | "add_rules")
        });
        if describes { FileKind::Description } else { FileKind::Script }
    }
}

/// A file ready for queries: text, tree, line index and outline.
#[derive(Debug)]
pub struct AnalyzedFile {
    pub text: Arc<str>,
    pub kind: FileKind,
    pub dialect: Dialect,
    pub parse: Parse,
    pub line_index: LineIndex,
    /// Empty for scripts.
    pub outline: Outline,
}

impl AnalyzedFile {
    /// Analyzes `text`, the contents of a file named `file_name`.
    pub fn new(text: Arc<str>, file_name: &str, dialect: Dialect) -> AnalyzedFile {
        AnalyzedFile::build(text, dialect, |root| FileKind::detect(file_name, root))
    }

    /// Analyzes `text` as a file of the given kind, e.g. a description file
    /// loaded by `includes()` whatever its name.
    pub fn with_kind(text: Arc<str>, kind: FileKind, dialect: Dialect) -> AnalyzedFile {
        AnalyzedFile::build(text, dialect, |_| kind)
    }

    fn build(
        text: Arc<str>,
        dialect: Dialect,
        kind: impl FnOnce(SyntaxNode<'_>) -> FileKind,
    ) -> AnalyzedFile {
        let parse = parse(&text, dialect);
        let root = parse.root(&text);
        let kind = kind(root);
        let outline = match kind {
            FileKind::Description => Outline::build(root),
            FileKind::Script => Outline::default(),
        };
        let line_index = LineIndex::new(&text);
        AnalyzedFile { kind, dialect, outline, line_index, parse, text }
    }

    /// The root node.
    pub fn root(&self) -> SyntaxNode<'_> {
        self.parse.root(&self.text)
    }
}
