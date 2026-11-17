//! Lexical name resolution: which declaration a name refers to.

use xmake_api::Instance;
use xmake_syntax::{
    SyntaxKind, SyntaxNode, SyntaxToken, TextRange, TextSize,
    ast::{AstNode, CallExpr, Literal, LocalStmt, Name},
};

use crate::{AnalyzedFile, outline::content_range};

/// What declared a name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BindingKind<'a> {
    /// `local x`
    Local,
    /// A function parameter; the first parameter of a script callback is
    /// the instance of its scope.
    Param(Option<Instance>),
    /// `local function f` / `function f` at the top level.
    Function,
    /// `for i = ...`, `for k, v in ...`
    LoopVar,
    /// `import("core.base.json")` binds `json` to the module path.
    Import { module: &'a str },
}

/// A declaration visible at some offset.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Binding<'a> {
    pub name: &'a str,
    /// The declaring identifier (for an import, the module string).
    pub range: TextRange,
    pub kind: BindingKind<'a>,
}

/// The declarations visible at `offset`, innermost first. A name may appear
/// several times: the first one shadows the others.
pub fn visible_bindings(file: &AnalyzedFile, offset: TextSize) -> Vec<Binding<'_>> {
    let root = file.root();
    let mut out = Vec::new();
    let start = root.covering_node(TextRange::empty(offset));
    let ancestors = std::iter::once(start).chain(start.ancestors());
    for node in ancestors {
        match node.kind() {
            SyntaxKind::BLOCK => block_bindings(node, offset, &mut out),
            SyntaxKind::FUNCTION_EXPR
            | SyntaxKind::FUNCTION_STMT
            | SyntaxKind::LOCAL_FUNCTION_STMT => {
                function_bindings(file, node, offset, &mut out);
            }
            SyntaxKind::NUMERIC_FOR_STMT | SyntaxKind::GENERIC_FOR_STMT => {
                loop_bindings(node, offset, &mut out);
            }
            _ => {}
        }
    }
    global_functions(root, &mut out);
    out
}

/// The declaration `name_ref` (an `IDENT` of a `NAME_REF`) refers to.
pub fn resolve<'a>(file: &'a AnalyzedFile, name_ref: SyntaxToken<'_>) -> Option<Binding<'a>> {
    let name = name_ref.text();
    let bindings = visible_bindings(file, name_ref.text_range().start());
    bindings.into_iter().find(|b| b.name == name)
}

/// Locals declared by the statements of `block` that end before `offset`.
fn block_bindings<'a>(block: SyntaxNode<'a>, offset: TextSize, out: &mut Vec<Binding<'a>>) {
    let statements: Vec<_> = block.children().collect();
    for stmt in statements.into_iter().rev() {
        let range = stmt.text_range();
        let before = range.end() <= offset;
        match stmt.kind() {
            SyntaxKind::LOCAL_STMT if before && !lacks_value(stmt) => {
                let local = LocalStmt::cast(stmt).expect("local statement");
                let names: Vec<_> = local.names().collect();
                for name in names.into_iter().rev() {
                    push_name(out, name, BindingKind::Local);
                }
            }
            // The function sees its own name.
            SyntaxKind::LOCAL_FUNCTION_STMT if range.start() < offset => {
                if let Some(name) = stmt.children().find_map(Name::cast) {
                    push_name(out, name, BindingKind::Function);
                }
            }
            SyntaxKind::CALL_STMT if before => {
                out.extend(import_binding(stmt));
            }
            _ => {}
        }
    }
}

/// `local x =` being typed: `x` is not declared yet.
fn lacks_value(stmt: SyntaxNode<'_>) -> bool {
    let has_eq = stmt.child_token(SyntaxKind::EQ).is_some();
    let values = stmt.child_node(SyntaxKind::EXPR_LIST);
    has_eq && values.is_none_or(|v| v.children().next().is_none())
}

/// The parameters of `function`, when `offset` is in its body.
fn function_bindings<'a>(
    file: &AnalyzedFile,
    function: SyntaxNode<'a>,
    offset: TextSize,
    out: &mut Vec<Binding<'a>>,
) {
    let Some(params) = function.child_node(SyntaxKind::PARAM_LIST) else {
        return;
    };
    if offset < params.text_range().end() {
        return;
    }
    let instance =
        file.outline.callback(function.text_range()).and_then(|c| c.scope.script_instance());
    let names: Vec<_> = params.children().filter_map(Name::cast).collect();
    for (i, name) in names.into_iter().enumerate() {
        let instance = if i == 0 { instance } else { None };
        push_name(out, name, BindingKind::Param(instance));
    }
}

/// The loop variables, when `offset` is in the loop body.
fn loop_bindings<'a>(stmt: SyntaxNode<'a>, offset: TextSize, out: &mut Vec<Binding<'a>>) {
    let in_body = stmt
        .child_node(SyntaxKind::BLOCK)
        .is_some_and(|b| b.text_range().contains_inclusive(offset));
    if !in_body {
        return;
    }
    let direct = stmt.children().filter_map(Name::cast);
    let listed = stmt
        .child_node(SyntaxKind::NAME_LIST)
        .into_iter()
        .flat_map(|l| l.children().filter_map(Name::cast));
    let names: Vec<_> = direct.chain(listed).collect();
    for name in names.into_iter().rev() {
        push_name(out, name, BindingKind::LoopVar);
    }
}

/// `function name() end` statements of the top level block, visible in the
/// whole file (they are globals).
fn global_functions<'a>(root: SyntaxNode<'a>, out: &mut Vec<Binding<'a>>) {
    let Some(block) = root.child_node(SyntaxKind::BLOCK) else {
        return;
    };
    for stmt in block.children().filter(|s| s.kind() == SyntaxKind::FUNCTION_STMT) {
        let Some(func_name) = stmt.child_node(SyntaxKind::FUNC_NAME) else {
            continue;
        };
        let mut idents = func_name.child_tokens().filter(|t| t.kind() == SyntaxKind::IDENT);
        let (Some(ident), None) = (idents.next(), idents.next()) else {
            continue;
        };
        out.push(Binding {
            name: ident.text(),
            range: ident.text_range(),
            kind: BindingKind::Function,
        });
    }
}

/// `import("a.b.c")` binds `c`; `import("a.b.c", {alias = "x"})` binds `x`.
fn import_binding(stmt: SyntaxNode<'_>) -> Option<Binding<'_>> {
    let call = stmt.children().find_map(CallExpr::cast)?;
    if call.callee_name()? != "import" {
        return None;
    }
    let literal = Literal::cast(call.arg(0)?)?;
    let token = literal.token()?;
    if token.kind() != SyntaxKind::STRING {
        return None;
    }
    let range = content_range(token);
    let module = &token.source()[range];
    let alias = call.arg(1).and_then(alias_option);
    let name = alias.unwrap_or_else(|| module.rsplit('.').next().unwrap_or(module));
    Some(Binding { name, range, kind: BindingKind::Import { module } })
}

/// `"x"` in `{alias = "x"}`.
fn alias_option(options: SyntaxNode<'_>) -> Option<&str> {
    let table = xmake_syntax::ast::TableExpr::cast(options)?;
    let field = table.fields().find(|f| f.key_name().is_some_and(|k| k.text() == "alias"))?;
    let token = Literal::cast(field.value()?)?.token()?;
    (token.kind() == SyntaxKind::STRING).then(|| &token.source()[content_range(token)])
}

fn push_name<'a>(out: &mut Vec<Binding<'a>>, name: Name<'a>, kind: BindingKind<'a>) {
    if let Some(ident) = name.ident() {
        out.push(Binding { name: ident.text(), range: ident.text_range(), kind });
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use xmake_syntax::Dialect;

    use super::*;

    fn bindings_at_marker(text: &str) -> Vec<(String, BindingKind<'static>)> {
        let offset = text.find('$').expect("marker");
        let file = AnalyzedFile::new(Arc::from(text), "xmake.lua", Dialect::XMAKE_DEFAULT);
        let found = visible_bindings(&file, TextSize::new(offset as u32));
        found
            .into_iter()
            .map(|b| {
                let kind = match b.kind {
                    BindingKind::Import { .. } => BindingKind::Import { module: "" },
                    BindingKind::Local => BindingKind::Local,
                    BindingKind::Param(i) => BindingKind::Param(i),
                    BindingKind::Function => BindingKind::Function,
                    BindingKind::LoopVar => BindingKind::LoopVar,
                };
                (b.name.to_owned(), kind)
            })
            .collect()
    }

    #[test]
    fn locals_and_params() {
        let text = "local a = 1\ntarget('x')\non_build(function (t, opt)\n  local b = 2\n  for i = 1, 2 do $ end\nend)";
        let names: Vec<_> =
            bindings_at_marker(text).into_iter().map(|(n, k)| format!("{n}:{k:?}")).collect();
        assert_eq!(
            names,
            ["i:LoopVar", "b:Local", "t:Param(Some(Target))", "opt:Param(None)", "a:Local"]
        );
    }

    #[test]
    fn locals_are_visible_after_their_statement() {
        let names = bindings_at_marker("local a = $\nlocal b = 1");
        assert!(names.is_empty(), "{names:?}");
    }

    #[test]
    fn imports() {
        let text = "function main()\n  import('core.base.json')\n  import('lib.detect.find_tool', {alias = 'ft'})\n  $\nend";
        let names: Vec<_> = bindings_at_marker(text).into_iter().map(|(n, _)| n).collect();
        assert_eq!(names, ["ft", "json", "main"]);
    }
}
