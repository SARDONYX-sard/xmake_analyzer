//! What the parameters of a call accept: from the stubs, the annotations of
//! the functions of the project, or the prototypes of the manual.

use xmake_syntax::{
    SyntaxNode,
    ast::{AstNode, CallExpr, NameRef},
};

use crate::{AnalyzedFile, CodeContext, Project, Type, resolve::resolve};

/// What a parameter accepts.
#[derive(Clone, Debug)]
pub struct Expected<'a> {
    pub name: &'a str,
    pub ty: Type<'a>,
    /// `...`: the parameter takes every argument from here on.
    pub variadic: bool,
}

/// The parameter the `index`-th argument of a call fills.
pub fn nth_expected<'e, 'a>(params: &'e [Expected<'a>], index: usize) -> Option<&'e Expected<'a>> {
    params
        .get(index)
        .or_else(|| params.iter().rfind(|p| p.variadic).filter(|_| index >= params.len()))
}

/// The parameters of the function `call` calls, when known.
pub fn expected_params<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    call: SyntaxNode<'a>,
) -> Option<Vec<Expected<'a>>> {
    if let Some(function) = crate::called_stub(project, file, call) {
        let params = crate::call_params(function, call).iter().map(|p| {
            let ty = Type::parse(p.ty);
            Expected {
                name: p.name,
                ty: if p.optional { Type::union([ty, Type::Nil]) } else { ty },
                variadic: p.name == "...",
            }
        });
        return Some(params.collect());
    }
    // A function of the file with `---@param` annotations.
    if let Some((_, statement)) = crate::called_declaration(project, file, call) {
        let params = xmake_project::annotations::function_params(statement)?;
        let mut annotated = false;
        let expected = xmake_project::annotations::param_names(params).map(|name| {
            let name = name.text();
            let declared = xmake_project::annotations::param_type(statement, name);
            annotated |= declared.is_some();
            let ty = match declared {
                Some((ty, true)) => Type::union([crate::annotated(project, file, ty), Type::Nil]),
                Some((ty, false)) => crate::annotated(project, file, ty),
                None => Type::Any,
            };
            Expected { name, ty, variadic: name == "..." }
        });
        let expected: Vec<_> = expected.collect();
        return annotated.then_some(expected);
    }
    // A description API: the prototype of the manual.
    let ident = CallExpr::cast(call)?.callee().and_then(NameRef::cast)?.ident()?;
    // `target("x", function () ... end)` and `target("x", {...})` both work.
    if resolve(file, ident).is_some() || xmake_api::Scope::from_opener(ident.text()).is_some() {
        return None;
    }
    let scope = match CodeContext::at(file, ident.text_range().start()) {
        CodeContext::Description(scope) => Some(scope),
        CodeContext::Script => None,
    };
    let (prototype, _) = xmake_api::doc_prototype(scope, ident.text())?;
    // `on_build("windows", function (target) end, {extra})`
    if let Some(script) = crate::scripts::raw_script_param(prototype) {
        let script = script.split_once(':')?.1;
        let script = Type::union([Type::parse(script), Type::String]);
        let args: Vec<_> = CallExpr::cast(call)?.arg_list()?.args().collect();
        let expected = crate::script_args(&args).into_iter().map(|role| match role {
            crate::ScriptArg::Pattern => {
                Expected { name: "pattern", ty: Type::String, variadic: false }
            }
            crate::ScriptArg::Script => {
                Expected { name: "script", ty: script.clone(), variadic: false }
            }
            crate::ScriptArg::Extra => Expected { name: "extra", ty: Type::Table, variadic: false },
        });
        return Some(expected.collect());
    }
    let mut out: Vec<Expected<'a>> = Vec::new();
    for param in xmake_api::prototype_params(prototype) {
        if param == "..." {
            // The previous parameter repeats.
            if let Some(last) = out.last_mut() {
                last.variadic = true;
            }
            continue;
        }
        if param.starts_with('{') {
            out.push(Expected { name: "options", ty: Type::Table, variadic: false });
            continue;
        }
        let (name, ty) = param.split_once(':')?;
        let optional = name.trim().ends_with('?');
        let ty = Type::parse(ty);
        out.push(Expected {
            name: name.trim().trim_end_matches('?'),
            ty: if optional { Type::union([ty, Type::Nil]) } else { ty },
            variadic: false,
        });
    }
    // xmake takes any number of values for the values and paths APIs, also
    // when the manual writes one before the options: `set_warnings("all",
    // "error")` for `set_warnings(warnings: string, {private})`. The options
    // come after the values: a table is accepted. `set_kind(kind: string)`,
    // without options, keeps its single value.
    let kind = scope.and_then(|s| project.db.visible_api(s, ident.text())).map(|api| api.kind);
    if matches!(kind, Some(xmake_api::ApiKind::Values | xmake_api::ApiKind::Paths))
        && out.last().is_some_and(|p| p.name == "options")
        && !out.iter().any(|p| p.variadic)
        && let Some(last) = out.iter_mut().rfind(|p| p.name != "options")
    {
        last.variadic = true;
        last.ty = Type::union([last.ty.clone(), Type::Table]);
    }
    // `add_files(files, ..., {options})`: after the repeated files comes the
    // table of options; checking it against `files` would be wrong.
    if out.iter().any(|p| p.variadic) && out.last().is_some_and(|p| p.name == "options") {
        out.pop();
    }
    Some(out)
}

/// The other operand of `==` or `~=` when `operand` is one side:
/// `target:kind()` for `"shared"` in `target:kind() == "shared"`.
pub fn compared_with(operand: SyntaxNode<'_>) -> Option<SyntaxNode<'_>> {
    let comparison = operand.parent().filter(|p| p.kind() == xmake_syntax::SyntaxKind::BIN_EXPR)?;
    let operator = comparison.child_tokens().find(|t| !t.kind().is_trivia())?.kind();
    if !matches!(operator, xmake_syntax::SyntaxKind::EQ2 | xmake_syntax::SyntaxKind::NEQ) {
        return None;
    }
    comparison.children().find(|n| *n != operand)
}
