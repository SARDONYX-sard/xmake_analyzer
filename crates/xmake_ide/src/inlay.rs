//! Inlay hints.
//!
//! - the instance a script callback receives: `function (target: target)`;
//! - parameter names in calls to functions of the file: `f(a: 1, b: 2)`;
//! - what an `end` closes: `target_end() -- app`, `end) -- on_build`.
//!
//! Labels borrow the names; the server adds the punctuation.

use std::borrow::Cow;

use xmake_api::Scope;
use xmake_syntax::{
    SyntaxKind, SyntaxNode, TextRange, TextSize,
    ast::{AstNode, CallExpr, NameRef},
};

use crate::{
    AnalyzedFile,
    resolve::{BindingKind, resolve},
};

/// What a hint shows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InlayKind {
    /// `: label` after a parameter.
    Type,
    /// `label:` before an argument.
    Parameter,
    /// `-- label` after the end of a scope or callback.
    End,
    /// `-> label` after the parameters of a function: what it returns.
    Return,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InlayHint<'a> {
    pub offset: TextSize,
    pub label: Cow<'a, str>,
    pub kind: InlayKind,
}

/// The positions of an option of a task: `{"n", "name", "kv", "xmake", "The name."}`.
const MENU_OPTION_FIELDS: [&str; 5] = ["short", "long", "kind", "default", "description"];

/// `short:`, `long:`, ... before the values of an option of
/// `set_menu {options = {{"n", "name", "kv", nil, "..."}}}`.
fn menu_option_hints(table: SyntaxNode<'_>, out: &mut Vec<InlayHint<'_>>) {
    use xmake_syntax::ast::{AstNode, TableExpr, TableField};
    // option -> its field -> the list -> `options = ...` -> the menu -> the
    // arguments of `set_menu`.
    let in_options = table
        .parent()
        .and_then(|element| element.parent())
        .and_then(|list| list.parent())
        .and_then(TableField::cast)
        .filter(|f| f.key_name().is_some_and(|k| k.text() == "options"))
        .and_then(|f| f.syntax().parent())
        .and_then(|menu| menu.parent())
        .filter(|args| args.kind() == SyntaxKind::ARG_LIST)
        .and_then(|args| args.parent())
        .and_then(CallExpr::cast)
        .is_some_and(|call| call.callee_name() == Some("set_menu"));
    let Some(option) = TableExpr::cast(table).filter(|_| in_options) else {
        return;
    };
    let positional = option.fields().filter(|f| f.key_name().is_none());
    for (field, label) in positional.zip(MENU_OPTION_FIELDS) {
        if let Some(value) = field.value() {
            out.push(InlayHint {
                offset: value.trimmed_range().start(),
                label: Cow::Borrowed(label),
                kind: InlayKind::Parameter,
            });
        }
    }
}

/// The hints of `file` in `range` (the whole file for `None`).
pub fn inlay_hints<'a>(
    project: &'a crate::Project,
    file: &'a AnalyzedFile,
    range: Option<TextRange>,
) -> Vec<InlayHint<'a>> {
    let root = file.root();
    let range = range.unwrap_or_else(|| root.text_range());
    let mut out = Vec::new();
    callback_hints(file, root, &mut out);
    scope_end_hints(file, &mut out);
    for node in root.descendants() {
        if node.text_range().intersect(range).is_none() {
            continue;
        }
        match node.kind() {
            SyntaxKind::CALL_EXPR => {
                let call = CallExpr::cast(node).expect("call");
                if !parameter_hints(file, call, &mut out)
                    && !stub_parameter_hints(project, file, node, call.arg_list(), &mut out)
                {
                    api_parameter_hints(file, call, &mut out);
                }
            }
            SyntaxKind::METHOD_CALL_EXPR => {
                let args = xmake_syntax::ast::MethodCallExpr::cast(node).and_then(|c| c.arg_list());
                stub_parameter_hints(project, file, node, args, &mut out);
            }
            SyntaxKind::TABLE_EXPR => menu_option_hints(node, &mut out),
            SyntaxKind::LOCAL_STMT => local_type_hints(project, file, node, &mut out),
            SyntaxKind::FUNCTION_STMT
            | SyntaxKind::LOCAL_FUNCTION_STMT
            | SyntaxKind::FUNCTION_EXPR => return_type_hints(project, file, node, &mut out),
            SyntaxKind::GENERIC_FOR_STMT => loop_type_hints(project, file, node, &mut out),
            _ => {}
        }
    }
    out.retain(|h| range.contains_inclusive(h.offset));
    out.sort_by_key(|h| h.offset);
    out
}

/// `function f(a) -> string`: what a function returns, from its `return`s.
/// Nothing when it returns nothing, nothing known, or says it with
/// `---@return`.
fn return_type_hints<'a>(
    project: &'a crate::Project,
    file: &'a AnalyzedFile,
    function: SyntaxNode<'a>,
    out: &mut Vec<InlayHint<'a>>,
) {
    let Some(params) = function.child_node(SyntaxKind::PARAM_LIST) else {
        return;
    };
    // `local f = function () end` is documented by its statement.
    let statement = match function.kind() {
        SyntaxKind::FUNCTION_EXPR => {
            xmake_project::annotations::statement_of_params(params).unwrap_or(function)
        }
        _ => function,
    };
    if !xmake_project::annotations::return_types(statement).is_empty() {
        return;
    }
    let returns = crate::types::function_returns(project, file, statement);
    if returns.is_empty() || returns.iter().all(crate::Type::is_unknown) {
        return;
    }
    let label = match returns.as_slice() {
        [single] => single.text(),
        _ => Cow::Owned(returns.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ")),
    };
    out.push(InlayHint { offset: params.trimmed_range().end(), label, kind: InlayKind::Return });
}

/// The instance type of callback parameters, and the API after their `end`.
fn callback_hints<'a>(file: &'a AnalyzedFile, root: SyntaxNode<'a>, out: &mut Vec<InlayHint<'a>>) {
    for callback in &file.outline.callbacks {
        let function = root.covering_node(callback.function_range);
        if function.kind() != SyntaxKind::FUNCTION_EXPR {
            continue;
        }
        let first_param = function
            .child_node(SyntaxKind::PARAM_LIST)
            .and_then(|p| p.child_node(SyntaxKind::NAME));
        // `function (target)` says it already: only `function (t: target)`.
        let instance = callback.scope.script_instance();
        let named_like_type =
            first_param.zip(instance).is_some_and(|(p, i)| p.text().trim() == i.name());
        if let (Some(param), Some(instance), false) = (first_param, instance, named_like_type) {
            let offset = param.text_range().end();
            out.push(InlayHint {
                offset,
                label: Cow::Borrowed(instance.name()),
                kind: InlayKind::Type,
            });
        }
        let api = function
            .parent()
            .and_then(|args| args.parent())
            .and_then(CallExpr::cast)
            .and_then(|c| c.callee_name());
        let end = function.child_token(SyntaxKind::END_KW);
        if let (Some(api), Some(end)) = (api, end) {
            // After `end)`, or `end` when the call has no parentheses.
            let close = end.next_token().filter(|t| t.kind() == SyntaxKind::R_PAREN);
            let offset = close.unwrap_or(end).text_range().end();
            if spans_lines(file, callback.function_range) {
                out.push(InlayHint { offset, label: Cow::Borrowed(api), kind: InlayKind::End });
            }
        }
    }
}

/// `target_end()  -- app`
fn scope_end_hints<'a>(file: &'a AnalyzedFile, out: &mut Vec<InlayHint<'a>>) {
    let text = &*file.text;
    for def in &file.outline.definitions {
        let end = def.full_range.end();
        let before = &text[..end.into()];
        let closes = before.trim_end().ends_with(')') && {
            let call = before
                .trim_end()
                .trim_end_matches(|c: char| c == ')' || c == '(' || c.is_whitespace());
            let scope_call = call.strip_suffix("_end");
            scope_call.is_some_and(|c| Scope::NAMED.iter().any(|s| c.ends_with(s.name())))
        };
        if closes && spans_lines(file, def.full_range) {
            out.push(InlayHint {
                offset: end,
                label: Cow::Borrowed(&text[def.name_range]),
                kind: InlayKind::End,
            });
        }
    }
}

fn spans_lines(file: &AnalyzedFile, range: TextRange) -> bool {
    file.text[range].contains('\n')
}

/// `: string` after `local name = os.curdir()`, as rust-analyzer does,
/// unless the value says it already (a literal, a table, a function) or the
/// type is unknown.
fn local_type_hints<'a>(
    project: &'a crate::Project,
    file: &'a AnalyzedFile,
    statement: SyntaxNode<'a>,
    out: &mut Vec<InlayHint<'a>>,
) {
    let values: Vec<SyntaxNode<'a>> = statement
        .child_node(SyntaxKind::EXPR_LIST)
        .map(|l| l.children().collect())
        .unwrap_or_default();
    let Some(names) = statement.child_node(SyntaxKind::NAME_LIST) else {
        return;
    };
    let names = names.children().filter(|n| n.kind() == SyntaxKind::NAME);
    for (index, name) in names.enumerate() {
        let obvious = values.get(index).is_some_and(|v| {
            matches!(
                v.kind(),
                SyntaxKind::LITERAL | SyntaxKind::TABLE_EXPR | SyntaxKind::FUNCTION_EXPR
            )
        });
        if obvious || values.is_empty() {
            continue;
        }
        let Some(ident) = name.first_token() else {
            continue;
        };
        let ty = crate::types::variable_type(project, file, BindingKind::Local, ident.text_range());
        push_type(out, name.text_range().end(), ty);
    }
}

/// `i: integer, file: string` in `for i, file in ipairs(os.files(...))`.
fn loop_type_hints<'a>(
    project: &'a crate::Project,
    file: &'a AnalyzedFile,
    statement: SyntaxNode<'a>,
    out: &mut Vec<InlayHint<'a>>,
) {
    let Some(names) = statement.child_node(SyntaxKind::NAME_LIST) else {
        return;
    };
    for name in names.children().filter(|n| n.kind() == SyntaxKind::NAME) {
        let Some(ident) = name.first_token() else {
            continue;
        };
        let ty =
            crate::types::variable_type(project, file, BindingKind::LoopVar, ident.text_range());
        push_type(out, name.text_range().end(), ty);
    }
}

fn push_type<'a>(
    out: &mut Vec<InlayHint<'a>>,
    offset: xmake_syntax::TextSize,
    ty: crate::Type<'a>,
) {
    if ty.is_unknown() || matches!(ty, crate::Type::Nil | crate::Type::Never) {
        return;
    }
    out.push(InlayHint { offset, label: ty.text(), kind: InlayKind::Type });
}

/// Parameter names of the functions typed by the stubs: `os.cp(src: a,
/// dst: b)`, `target:add(name: "defines", ...)`. Variadic parameters get
/// none.
fn stub_parameter_hints<'a>(
    project: &'a crate::Project,
    file: &'a AnalyzedFile,
    call: SyntaxNode<'a>,
    args: Option<xmake_syntax::ast::ArgList<'a>>,
    out: &mut Vec<InlayHint<'a>>,
) -> bool {
    let (Some(function), Some(args)) = (crate::types::called_stub(project, file, call), args)
    else {
        return false;
    };
    for (param, arg) in crate::types::call_params(function, call).iter().zip(args.args()) {
        if param.name == "..." {
            break;
        }
        if arg.text().trim() == param.name {
            continue;
        }
        let offset = arg.trimmed_range().start();
        out.push(InlayHint {
            offset,
            label: Cow::Borrowed(param.name),
            kind: InlayKind::Parameter,
        });
    }
    true
}

/// Parameter names of the description APIs, from the prototypes of the
/// manual: `set_default(default: false)`, `add_files(files: "src/*.c", ...)`.
/// Variadic parameters and tables of options get none.
fn api_parameter_hints<'a>(
    file: &'a AnalyzedFile,
    call: CallExpr<'a>,
    out: &mut Vec<InlayHint<'a>>,
) {
    let (Some(name_ref), Some(args)) = (call.callee().and_then(NameRef::cast), call.arg_list())
    else {
        return;
    };
    let Some(ident) = name_ref.ident() else {
        return;
    };
    // `target("app")` and `on_build(function ...)` say it already.
    if resolve(file, ident).is_some() || xmake_api::Scope::from_opener(ident.text()).is_some() {
        return;
    }
    let scope = match crate::CodeContext::at(file, ident.text_range().start()) {
        crate::CodeContext::Description(scope) => Some(scope),
        crate::CodeContext::Script => None,
    };
    let Some((prototype, _)) = xmake_api::doc_prototype(scope, ident.text()) else {
        return;
    };
    if xmake_types::script_param(prototype).is_some() {
        let args: Vec<_> = args.args().collect();
        for (role, arg) in xmake_types::script_args(&args).into_iter().zip(&args) {
            if role == xmake_types::ScriptArg::Pattern {
                let offset = arg.trimmed_range().start();
                out.push(InlayHint {
                    offset,
                    label: Cow::Borrowed("pattern"),
                    kind: InlayKind::Parameter,
                });
            }
        }
        return;
    }
    for (param, arg) in xmake_api::prototype_params(prototype).into_iter().zip(args.args()) {
        // `files: string|array` -> `files`; stop at `...` and `{options}`.
        let Some((name, _)) = param.split_once(':') else {
            break;
        };
        let name = name.trim().trim_end_matches('?');
        if name.is_empty() || name.contains(['.', '{']) {
            break;
        }
        if arg.text().trim() == name || arg.kind() == SyntaxKind::FUNCTION_EXPR {
            continue;
        }
        out.push(InlayHint {
            offset: arg.trimmed_range().start(),
            label: Cow::Borrowed(name),
            kind: InlayKind::Parameter,
        });
    }
}

/// Parameter names before the arguments of calls to functions declared in
/// the file, unless the argument already says it (`f(name)` for `name`).
/// Returns whether the call is to such a function.
fn parameter_hints<'a>(
    file: &'a AnalyzedFile,
    call: CallExpr<'a>,
    out: &mut Vec<InlayHint<'a>>,
) -> bool {
    let Some(name_ref) = call.callee().and_then(NameRef::cast) else {
        return false;
    };
    let Some(binding) = name_ref.ident().and_then(|ident| resolve(file, ident)) else {
        return false;
    };
    if binding.kind != BindingKind::Function {
        return false;
    }
    let root = file.root();
    let declaration = root.covering_node(binding.range);
    let function = std::iter::once(declaration)
        .chain(declaration.ancestors())
        .find(|n| matches!(n.kind(), SyntaxKind::FUNCTION_STMT | SyntaxKind::LOCAL_FUNCTION_STMT));
    let Some(params) = function.and_then(|f| f.child_node(SyntaxKind::PARAM_LIST)) else {
        return false;
    };
    let names = params.children().filter_map(|n| n.first_token()).map(|t| t.text());
    let Some(args) = call.arg_list() else {
        return false;
    };
    for (param, arg) in names.zip(args.args()) {
        if arg.text().trim() == param {
            continue;
        }
        let offset = arg.trimmed_range().start();
        out.push(InlayHint { offset, label: Cow::Borrowed(param), kind: InlayKind::Parameter });
    }
    true
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use xmake_syntax::Dialect;

    use super::*;

    #[test]
    fn return_types() {
        let text = "local function f(c)
  if c then return 1 end
  return 'a'
end
---@return string
local function g() return '' end
local h = function () return {'a'} end
local function none() end
";
        let file = AnalyzedFile::new(Arc::from(text), "xmake.lua", Dialect::XMAKE_DEFAULT);
        let hints: Vec<_> = inlay_hints(&crate::Project::default(), &file, None)
            .into_iter()
            .filter(|h| h.kind == InlayKind::Return)
            .map(|h| (&text[..usize::from(h.offset)], h.label.into_owned()))
            .map(|(before, label)| (before.lines().last().unwrap_or("").to_owned(), label))
            .collect();
        assert_eq!(
            hints,
            [
                ("local function f(c)".to_owned(), "integer|string".to_owned()),
                ("local h = function ()".to_owned(), "string[]".to_owned()),
            ]
        );
    }

    #[test]
    fn hints() {
        let text = "local function f(a, b) end\nf(1, b)\ntarget('app')\n  on_build(function (t)\n  end)\n  on_load(function (target)\n  end)\ntarget_end()\n";
        let file = AnalyzedFile::new(Arc::from(text), "xmake.lua", Dialect::XMAKE_DEFAULT);
        let hints: Vec<_> = inlay_hints(&crate::Project::default(), &file, None)
            .into_iter()
            .map(|h| (h.label.into_owned(), h.kind))
            .collect();
        assert_eq!(
            hints,
            [
                ("a".to_owned(), InlayKind::Parameter),
                ("target".to_owned(), InlayKind::Type),
                ("on_build".to_owned(), InlayKind::End),
                ("on_load".to_owned(), InlayKind::End),
                ("app".to_owned(), InlayKind::End),
            ]
        );
    }

    #[test]
    fn types_and_stub_parameters() {
        let text = "target('a')
on_load(function (t)
    local dir = os.curdir()
    local n = 1
    local d = t:dep('x')
    for i, f in ipairs(os.files('*.c')) do
        os.cp(f, dir)
    end
end)
";
        let project = crate::Project::default();
        let file = AnalyzedFile::new(Arc::from(text), "xmake.lua", Dialect::XMAKE_DEFAULT);
        let hints: Vec<_> = inlay_hints(&project, &file, None)
            .into_iter()
            .filter(|h| h.kind != InlayKind::End)
            .map(|h| format!("{:?} {}", h.kind, h.label))
            .collect();
        assert_eq!(
            hints,
            [
                "Type target",
                "Type string",
                "Type target?",
                "Parameter name",
                "Type integer",
                "Type string",
                "Parameter t",
                "Parameter pattern",
                "Parameter src",
                "Parameter dst",
            ]
        );
    }

    #[test]
    fn api_parameters() {
        let text = "target('a')
    set_default(false)
    add_files('a.c', 'b.c', {defines = 'X'})
    set_kind('binary')
";
        let file = AnalyzedFile::new(Arc::from(text), "xmake.lua", Dialect::XMAKE_DEFAULT);
        let hints: Vec<_> = inlay_hints(&crate::Project::default(), &file, None)
            .into_iter()
            .filter(|h| h.kind == InlayKind::Parameter)
            .map(|h| {
                format!("{}@{}", h.label, &text[usize::from(h.offset)..usize::from(h.offset) + 3])
            })
            .collect();
        assert_eq!(hints, ["default@fal", "files@'a.", "kind@'bi"]);
    }

    #[test]
    fn menu_options() {
        let text = "task('hello')
    set_menu {
        usage = 'xmake hello',
        options = {
            {'n', 'name', 'kv', nil, 'The name.'},
        },
    }
";
        let mut project =
            crate::Project::new(xmake_api::ApiDb::builtin(), xmake_syntax::Dialect::XMAKE_DEFAULT);
        let file =
            project.set_file(std::path::PathBuf::from("xmake.lua"), std::sync::Arc::from(text));
        let labels: Vec<String> = inlay_hints(&project, &file, None)
            .into_iter()
            .filter(|h| h.kind == InlayKind::Parameter)
            .map(|h| h.label.into_owned())
            .collect();
        assert_eq!(labels, ["short", "long", "kind", "default", "description"]);
    }
}
