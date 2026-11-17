//! The calls around arguments.

use xmake_api::ArgValues;
use xmake_syntax::{
    SyntaxKind, SyntaxNode, SyntaxToken,
    ast::{AstNode, CallExpr, TableField},
};

use crate::{AnalyzedFile, CodeContext};

/// The values the `index`-th argument of `call` accepts, in the scope of
/// the call.
pub fn call_arg_values(file: &AnalyzedFile, call: CallExpr<'_>, index: usize) -> Option<ArgValues> {
    let name = call.callee_name()?;
    let scope = match CodeContext::at(file, call.syntax().text_range().start()) {
        CodeContext::Description(scope) => Some(scope),
        CodeContext::Script => None,
    };
    xmake_api::scoped_arg_values(scope, name, index)
}

/// The call a string argument belongs to, with the argument index.
pub fn enclosing_call<'a>(token: SyntaxToken<'a>) -> Option<(CallExpr<'a>, usize)> {
    let literal = token.parent()?;
    let args = literal.parent().filter(|p| p.kind() == SyntaxKind::ARG_LIST)?;
    let call = CallExpr::cast(args.parent()?)?;
    let index = args.children().take_while(|n| *n != literal).count();
    Some((call, index))
}

/// The call and the key of the option table a value is in:
/// `(add_cxxflags(...), "tools")` for `"cl"` in
/// `add_cxxflags("/W4", {tools = "cl"})` or `{tools = {"cl", "gcc"}}`.
pub fn option_of_value(value: SyntaxNode<'_>) -> Option<(CallExpr<'_>, &str)> {
    let mut node = value.parent()?;
    // `{"cl", "gcc"}`: the list is the value.
    if node.kind() == SyntaxKind::TABLE_FIELD && TableField::cast(node)?.key_name().is_none() {
        node = node.parent()?.parent()?;
    }
    let field = TableField::cast(node)?;
    let key = field.key_name()?.text();
    let args = node.parent()?.parent().filter(|n| n.kind() == SyntaxKind::ARG_LIST)?;
    Some((CallExpr::cast(args.parent()?)?, key))
}

/// Whether the API `name` takes flags, with the `tools` option.
pub fn is_flags_api(name: &str) -> bool {
    name.starts_with("add_") && name.ends_with("flags")
}
