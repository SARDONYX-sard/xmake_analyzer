//! Semantic highlighting: what names refer to, beyond what a TextMate
//! grammar can tell (APIs, modules, instances, parameters).
//!
//! As rust-analyzer does, string contents get no token: a string keeps the
//! string color of the theme, names of targets inside included.

use xmake_syntax::{
    SyntaxKind, SyntaxToken, TextRange,
    ast::{AstNode, FieldExpr, NameRef},
};

use crate::{
    AnalyzedFile, Project,
    context::CodeContext,
    resolve::{BindingKind, resolve},
};

/// Token types, in legend order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TokenType {
    /// Modules and imports.
    Namespace,
    /// Scope functions: `target`, `option`, ...
    Keyword,
    /// Project items in strings: targets, options, ...
    Class,
    Function,
    Method,
    Parameter,
    Variable,
    Property,
}

impl TokenType {
    /// Every type with its protocol name, in legend order.
    pub const ALL: [(TokenType, &'static str); 8] = [
        (TokenType::Namespace, "namespace"),
        (TokenType::Keyword, "keyword"),
        (TokenType::Class, "class"),
        (TokenType::Function, "function"),
        (TokenType::Method, "method"),
        (TokenType::Parameter, "parameter"),
        (TokenType::Variable, "variable"),
        (TokenType::Property, "property"),
    ];
}

/// Token modifiers, as bits in legend order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum TokenModifier {
    Declaration = 1,
    /// Provided by xmake.
    DefaultLibrary = 2,
}

impl TokenModifier {
    /// Every modifier with its protocol name, in bit order.
    pub const ALL: [(TokenModifier, &'static str); 2] = [
        (TokenModifier::Declaration, "declaration"),
        (TokenModifier::DefaultLibrary, "defaultLibrary"),
    ];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SemanticToken {
    pub range: TextRange,
    pub ty: TokenType,
    pub modifiers: u32,
}

const DECL: u32 = TokenModifier::Declaration as u32;
const LIB: u32 = TokenModifier::DefaultLibrary as u32;

/// The tokens of `file` in `range` (the whole file for `None`), in order.
pub fn semantic_tokens(
    project: &Project,
    file: &AnalyzedFile,
    range: Option<TextRange>,
) -> Vec<SemanticToken> {
    let root = file.root();
    let range = range.unwrap_or_else(|| root.text_range());
    let mut out = Vec::new();
    let tokens = root.descendant_tokens().filter(|t| range.contains_range(t.text_range()));
    for token in tokens {
        let classified = match token.kind() {
            SyntaxKind::IDENT => {
                classify_ident(project, file, token).map(|(ty, m)| (token.text_range(), ty, m))
            }
            _ => None,
        };
        // Multi-line tokens cannot be expressed in the protocol.
        if let Some((range, ty, modifiers)) = classified
            && !range.is_empty()
            && !file.text[range].contains('\n')
        {
            out.push(SemanticToken { range, ty, modifiers });
        }
    }
    out
}

fn classify_ident(
    project: &Project,
    file: &AnalyzedFile,
    token: SyntaxToken<'_>,
) -> Option<(TokenType, u32)> {
    let parent = token.parent()?;
    let db = &project.db;
    match parent.kind() {
        SyntaxKind::NAME => {
            let owner = parent.parent()?.kind();
            let ty = match owner {
                SyntaxKind::PARAM_LIST => TokenType::Parameter,
                SyntaxKind::LOCAL_FUNCTION_STMT => TokenType::Function,
                _ => TokenType::Variable,
            };
            Some((ty, DECL))
        }
        SyntaxKind::FUNC_NAME => {
            let is_last = token.next_token().is_none_or(|t| t.parent() != Some(parent));
            Some(if is_last { (TokenType::Function, DECL) } else { (TokenType::Variable, 0) })
        }
        SyntaxKind::NAME_REF => {
            if let Some(binding) = resolve(file, token) {
                let ty = match binding.kind {
                    BindingKind::Param(_) => TokenType::Parameter,
                    BindingKind::Local | BindingKind::LoopVar => TokenType::Variable,
                    BindingKind::Function => TokenType::Function,
                    BindingKind::Import { .. } => TokenType::Namespace,
                };
                return Some((ty, 0));
            }
            let name = token.text();
            if xmake_api::Scope::from_opener(name).is_some() {
                // A function, like the APIs: blue in most themes, as with lua-ls.
                return Some((TokenType::Function, 0));
            }
            let script = CodeContext::at(file, token.text_range().start()).is_script();
            if db.module(name, script).is_some() {
                return Some((TokenType::Namespace, LIB));
            }
            let is_api = db.scopes_of(name).next().is_some()
                || db.scope_api(xmake_api::Scope::Root, name).is_some()
                || db.is_description_function(name)
                || db.script_functions().iter().any(|f| f == name);
            // No `defaultLibrary`: themes map `function.defaultLibrary` to
            // `support.function`, colored apart from the other functions.
            is_api.then_some((TokenType::Function, 0))
        }
        SyntaxKind::FIELD_EXPR => {
            let field = FieldExpr::cast(parent)?;
            let base = NameRef::cast(field.base()?)?;
            let base_ident = base.ident()?;
            match resolve(file, base_ident).map(|b| b.kind) {
                Some(BindingKind::Import { .. }) => Some((TokenType::Function, 0)),
                Some(_) => Some((TokenType::Property, 0)),
                None => {
                    let script = CodeContext::at(file, base_ident.text_range().start()).is_script();
                    let is_function = db
                        .module(base.text(), script)
                        .is_some_and(|m| m.functions.iter().any(|f| f == token.text()));
                    Some(if is_function {
                        (TokenType::Function, 0)
                    } else {
                        (TokenType::Property, 0)
                    })
                }
            }
        }
        SyntaxKind::METHOD_CALL_EXPR => Some((TokenType::Method, 0)),
        SyntaxKind::TABLE_FIELD => Some((TokenType::Property, 0)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, sync::Arc};

    use xmake_api::ApiDb;
    use xmake_syntax::Dialect;

    use super::*;

    #[test]
    fn classifies() {
        let text =
            "target('app')\n  set_kind('binary')\n  on_build(function (t) os.cp(t:name()) end)\n";
        let mut project = Project::new(ApiDb::builtin(), Dialect::XMAKE_DEFAULT);
        let file = project.set_file(PathBuf::from("xmake.lua"), Arc::from(text));
        let tokens: Vec<_> = semantic_tokens(&project, &file, None)
            .into_iter()
            .map(|t| format!("{}:{:?}:{}", &text[t.range], t.ty, t.modifiers))
            .collect();
        assert_eq!(
            tokens,
            [
                "target:Function:0",
                "set_kind:Function:0",
                "on_build:Function:0",
                "t:Parameter:1",
                "os:Namespace:2",
                "cp:Function:0",
                "t:Parameter:0",
                "name:Method:0",
            ]
        );
    }
}
