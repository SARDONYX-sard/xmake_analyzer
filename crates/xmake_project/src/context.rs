//! Where an offset is: description or script code, and which scope.

use xmake_api::Scope;
use xmake_syntax::{SyntaxKind, TextRange, TextSize};

use crate::{AnalyzedFile, FileKind};

/// The kind of code at an offset.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodeContext {
    /// Description code in a scope.
    Description(Scope),
    /// Script code: a callback, a function, a module.
    Script,
}

impl CodeContext {
    /// Computes the context at `offset`.
    ///
    /// Function bodies are scripts, except those passed to scope functions
    /// and `namespace`, which are descriptions.
    pub fn at(file: &AnalyzedFile, offset: TextSize) -> CodeContext {
        let root = file.root();
        let node = root.covering_node(TextRange::empty(offset));
        let functions = std::iter::once(node).chain(node.ancestors()).filter(|n| {
            matches!(
                n.kind(),
                SyntaxKind::FUNCTION_EXPR
                    | SyntaxKind::FUNCTION_STMT
                    | SyntaxKind::LOCAL_FUNCTION_STMT
            )
        });
        for function in functions {
            // In the parameter list, not in the body yet.
            let in_body = function
                .child_node(SyntaxKind::BLOCK)
                .is_some_and(|b| b.text_range().contains_inclusive(offset))
                || function
                    .child_node(SyntaxKind::PARAM_LIST)
                    .is_some_and(|p| p.text_range().end() <= offset);
            if !in_body {
                continue;
            }
            if !file.outline.description_bodies.contains(&function.text_range()) {
                return CodeContext::Script;
            }
        }
        match file.kind {
            FileKind::Description => CodeContext::Description(file.outline.scope_at(offset)),
            FileKind::Script => CodeContext::Script,
        }
    }

    /// Whether this is script code.
    pub fn is_script(self) -> bool {
        self == CodeContext::Script
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use xmake_syntax::Dialect;

    use super::*;

    fn context(text: &str, file_name: &str) -> CodeContext {
        let offset = text.find('$').expect("marker");
        let file = AnalyzedFile::new(Arc::from(text), file_name, Dialect::XMAKE_DEFAULT);
        CodeContext::at(&file, TextSize::new(offset as u32))
    }

    #[test]
    fn contexts() {
        use CodeContext::*;
        assert_eq!(context("target('a')\n$", "xmake.lua"), Description(Scope::Target));
        assert_eq!(context("target('a')\non_load(function (t) $ end)", "xmake.lua"), Script);
        assert_eq!(
            context("target('a', function () $ end)", "xmake.lua"),
            Description(Scope::Target)
        );
        assert_eq!(context("function main() $ end", "build.lua"), Script);
        assert_eq!(context("$", "build.lua"), Script);
    }
}
