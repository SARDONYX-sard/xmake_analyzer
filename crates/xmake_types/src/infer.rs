//! Type inference, from the stubs and the bindings.
//!
//! Types flow through:
//! - the parameter of a script callback: `function (target)` is a `target`;
//! - `import("core.base.json")`: the module `core.base.json`;
//! - builtin modules: `os`, `path`, ...;
//! - locals: `local dep = target:dep("core")` is a `target`, as the stub of
//!   `target:dep` returns `target?`;
//! - string literals, whose methods are those of `string`.
//!
//! Unknown is the answer whenever a step is not known: the features fall
//! back to what they did without types.

use xmake_api::Stubs;
use xmake_syntax::{
    SyntaxKind, SyntaxNode, TextRange,
    ast::{AstNode, CallExpr, FieldExpr, MethodCallExpr, NameRef},
};

use crate::ty::Type;
use crate::{
    AnalyzedFile, Project, annotations,
    context::CodeContext,
    resolve::{BindingKind, resolve},
};

/// The deepest inference: locals of locals of ... stop here.
const MAX_DEPTH: u8 = 8;

/// The type of the expression `node`.
pub fn type_of<'a>(project: &'a Project, file: &'a AnalyzedFile, node: SyntaxNode<'a>) -> Type<'a> {
    infer(project, file, node, 0)
}

fn infer<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    node: SyntaxNode<'a>,
    depth: u8,
) -> Type<'a> {
    if depth > MAX_DEPTH {
        return Type::Unknown;
    }
    let operand = |nth: usize| {
        node.children().nth(nth).map_or(Type::Unknown, |n| infer(project, file, n, depth + 1))
    };
    let operator = || node.child_tokens().find(|t| !t.kind().is_trivia()).map(|t| t.kind());
    match node.kind() {
        SyntaxKind::LITERAL => match node.first_token().map(|t| t.kind()) {
            // `"c++20"`: its value, unless escapes change it.
            Some(SyntaxKind::STRING) => {
                let text = node.text().trim();
                let value = text.get(1..text.len().saturating_sub(1)).unwrap_or("");
                if text.len() >= 2 && !value.contains('\\') {
                    Type::Literal(value)
                } else {
                    Type::String
                }
            }
            Some(SyntaxKind::LONG_STRING) => Type::String,
            Some(SyntaxKind::INT) => Type::Integer,
            Some(SyntaxKind::FLOAT) => Type::Number,
            Some(SyntaxKind::TRUE_KW) => Type::True,
            Some(SyntaxKind::FALSE_KW) => Type::False,
            Some(SyntaxKind::NIL_KW) => Type::Nil,
            _ => Type::Unknown,
        },
        SyntaxKind::PAREN_EXPR => operand(0),
        SyntaxKind::NAME_REF => name_type(project, file, node, depth),
        SyntaxKind::TABLE_EXPR => table_type(project, file, node, depth),
        SyntaxKind::FUNCTION_EXPR => {
            let params = node.child_node(SyntaxKind::PARAM_LIST).map_or("()", |p| p.text().trim());
            Type::Function(Some(params))
        }
        SyntaxKind::UNARY_EXPR => match operator() {
            Some(SyntaxKind::NOT_KW) => Type::Boolean,
            Some(SyntaxKind::HASH | SyntaxKind::TILDE) => Type::Integer,
            // `-x`: integers stay integers, strings convert to numbers.
            Some(SyntaxKind::MINUS) if operand(0) == Type::Integer => Type::Integer,
            Some(SyntaxKind::MINUS) => Type::Number,
            _ => Type::Unknown,
        },
        SyntaxKind::BIN_EXPR => match operator() {
            Some(SyntaxKind::DOT2) => Type::String,
            Some(
                SyntaxKind::EQ2
                | SyntaxKind::NEQ
                | SyntaxKind::LT
                | SyntaxKind::LTEQ
                | SyntaxKind::GT
                | SyntaxKind::GTEQ,
            ) => Type::Boolean,
            // Only `nil` and `false` are falsy.
            Some(SyntaxKind::OR_KW) => operand(0).or(operand(1)),
            Some(SyntaxKind::AND_KW) => operand(0).and(operand(1)),
            // Integers when both sides are, as Lua 5.3+.
            Some(
                SyntaxKind::PLUS
                | SyntaxKind::MINUS
                | SyntaxKind::STAR
                | SyntaxKind::PERCENT
                | SyntaxKind::SLASH2,
            ) => match (operand(0), operand(1)) {
                (Type::Integer, Type::Integer) => Type::Integer,
                _ => Type::Number,
            },
            Some(SyntaxKind::SLASH | SyntaxKind::CARET) => Type::Number,
            Some(
                SyntaxKind::AMP
                | SyntaxKind::PIPE
                | SyntaxKind::TILDE
                | SyntaxKind::SHL
                | SyntaxKind::SHR,
            ) => Type::Integer,
            _ => Type::Unknown,
        },
        SyntaxKind::CALL_EXPR | SyntaxKind::METHOD_CALL_EXPR => {
            nth_return(project, file, node, 0, depth)
        }
        _ => Type::Unknown,
    }
}

/// `{"a", "b"}` is a `string[]`; tables with keys are `table`s.
fn table_type<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    table: SyntaxNode<'a>,
    depth: u8,
) -> Type<'a> {
    let fields = crate::tables::fields(table);
    let positional = fields.iter().all(|(k, _)| matches!(k, crate::tables::Key::Index(_)));
    if fields.is_empty() || !positional {
        return Type::Table;
    }
    let elements = fields.iter().map(|(_, field)| {
        field.value().map_or(Type::Unknown, |v| infer(project, file, v, depth + 1).widen())
    });
    Type::Array(Box::new(Type::union(elements)))
}

fn name_type<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    node: SyntaxNode<'a>,
    depth: u8,
) -> Type<'a> {
    let Some(ident) = NameRef::cast(node).and_then(|n| n.ident()) else {
        return Type::Unknown;
    };
    let Some(binding) = resolve(file, ident) else {
        // A builtin module.
        let name = ident.text();
        let script = CodeContext::at(file, ident.text_range().start()).is_script();
        let known = project.db.module(name, script).is_some() || Stubs::get().class(name).is_some();
        return if known { Type::Module(name) } else { Type::Unknown };
    };
    variable_type_at(project, file, binding.kind, binding.range, depth + 1)
}

/// The value given to the local declared at `range`: `b` in `local a, b = x, y`.
pub fn initializer(file: &AnalyzedFile, range: TextRange) -> Option<SyntaxNode<'_>> {
    let token = file.root().token_at_offset(range.start()).right_biased()?;
    let statement = token.ancestors().find(|n| n.kind() == SyntaxKind::LOCAL_STMT)?;
    let names = statement.child_node(SyntaxKind::NAME_LIST)?;
    let index = names
        .children()
        .filter(|n| n.kind() == SyntaxKind::NAME)
        .position(|n| n.text_range().contains_range(range))?;
    statement.child_node(SyntaxKind::EXPR_LIST)?.children().nth(index)
}

/// The stub of the function a call calls: `os.iorun`, `target:dep`,
/// `print`, ...
pub fn called_stub<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    call: SyntaxNode<'a>,
) -> Option<&'static xmake_api::Function> {
    called_stub_at(project, file, call, 0)
}

fn called_stub_at<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    call: SyntaxNode<'a>,
    depth: u8,
) -> Option<&'static xmake_api::Function> {
    let stubs = Stubs::get();
    if let Some(method) = MethodCallExpr::cast(call) {
        let owner = infer(project, file, method.receiver()?, depth + 1).owner()?;
        return stubs.member(owner, method.method()?.text());
    }
    let callee = CallExpr::cast(call)?.callee()?;
    if let Some(field) = FieldExpr::cast(callee) {
        let base = infer(project, file, field.base()?, depth + 1);
        return stubs.member(base.module()?, field.field()?.text());
    }
    let name = NameRef::cast(callee)?.ident()?;
    if resolve(file, name).is_some() {
        return None;
    }
    stubs.member(xmake_api::GLOBALS, name.text())
}

/// The parameters the arguments of `call` fill: `s:split(",")` calls
/// `string.split(s, separator)`, so its arguments start at `separator`.
pub fn call_params<'f>(
    function: &'f xmake_api::Function,
    call: SyntaxNode<'_>,
) -> &'f [xmake_api::Param] {
    let implicit_self = call.kind() == SyntaxKind::METHOD_CALL_EXPR && !function.method;
    if implicit_self { function.params.get(1..).unwrap_or(&[]) } else { &function.params }
}

/// The type of the `index`-th value an expression list gives: `b` in
/// `local a, b = os.iorun(cmd)` is the second value of `os.iorun`.
pub fn nth_value_type<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    values: &[SyntaxNode<'a>],
    index: usize,
) -> Type<'a> {
    nth_value_type_at(project, file, values, index, 0)
}

fn nth_value_type_at<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    values: &[SyntaxNode<'a>],
    index: usize,
    depth: u8,
) -> Type<'a> {
    let is_call = |n: &SyntaxNode<'_>| {
        matches!(n.kind(), SyntaxKind::CALL_EXPR | SyntaxKind::METHOD_CALL_EXPR)
    };
    let (value, nth_return) = match values.get(index) {
        // A call in the middle gives its first value only.
        Some(value) => (*value, 0),
        None => match values.last() {
            Some(last) if is_call(last) => (*last, index + 1 - values.len()),
            _ => return Type::Nil,
        },
    };
    if nth_return == 0 {
        // A variable holding `true` is a `boolean`.
        return infer(project, file, value, depth).widen();
    }
    self::nth_return(project, file, value, nth_return, depth)
}

/// The `index`-th value `call` returns, per the stubs or the declaration of
/// a function of the project.
fn nth_return<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    call: SyntaxNode<'a>,
    index: usize,
    depth: u8,
) -> Type<'a> {
    if let Some(function) = called_stub_at(project, file, call, depth) {
        return function.returns.get(index).map_or(Type::Nil, |r| Type::parse(r.ty));
    }
    match called_declaration(project, file, call) {
        Some((other, statement)) => {
            let returns = function_returns_at(project, other, statement, depth + 1);
            returns.into_iter().nth(index).unwrap_or(Type::Nil)
        }
        None => Type::Unknown,
    }
}

/// The values the function declared by `statement` returns: its
/// `---@return` annotations, else the `return` statements of its body, each
/// position the union of what the statements give there.
pub fn function_returns<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    statement: SyntaxNode<'a>,
) -> Vec<Type<'a>> {
    function_returns_at(project, file, statement, 0)
}

fn function_returns_at<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    statement: SyntaxNode<'a>,
    depth: u8,
) -> Vec<Type<'a>> {
    let annotated = annotations::return_types(statement);
    if !annotated.is_empty() {
        return annotated.into_iter().map(|ty| self::annotated(project, file, ty)).collect();
    }
    if depth > MAX_DEPTH {
        return vec![Type::Unknown];
    }
    let Some(function) = annotations::function_params(statement).and_then(|p| p.parent()) else {
        return vec![Type::Unknown];
    };
    let is_function = |n: &SyntaxNode<'_>| {
        matches!(
            n.kind(),
            SyntaxKind::FUNCTION_STMT | SyntaxKind::LOCAL_FUNCTION_STMT | SyntaxKind::FUNCTION_EXPR
        )
    };
    // The `return`s of this function, not of the functions inside it.
    let returns = function.descendants().filter(|n| {
        n.kind() == SyntaxKind::RETURN_STMT && n.ancestors().find(is_function) == Some(function)
    });
    let lists: Vec<Vec<SyntaxNode<'a>>> = returns
        .map(|r| {
            r.child_node(SyntaxKind::EXPR_LIST).map(|l| l.children().collect()).unwrap_or_default()
        })
        .collect();
    let count = lists.iter().map(Vec::len).max().unwrap_or(0);
    (0..count)
        .map(|index| {
            Type::union(
                lists
                    .iter()
                    .map(|values| nth_value_type_at(project, file, values, index, depth + 1)),
            )
        })
        .collect()
}

/// The type an annotation of `file` writes, its `---@alias`es of the project
/// expanded: `Std` of `---@alias Std "c++20"|"c++23"`.
pub fn annotated<'a>(project: &'a Project, file: &'a AnalyzedFile, text: &'a str) -> Type<'a> {
    expand(project, file, Type::parse(text), 0)
}

fn expand<'a>(project: &'a Project, file: &'a AnalyzedFile, ty: Type<'a>, depth: u8) -> Type<'a> {
    if depth > MAX_DEPTH {
        return ty;
    }
    match ty {
        Type::Named(name) => match alias(project, file, name) {
            Some(aliased) => expand(project, file, Type::parse(aliased), depth + 1),
            None => Type::Named(name),
        },
        Type::Union(members) => {
            Type::union(members.into_iter().map(|m| expand(project, file, m, depth + 1)))
        }
        Type::Array(element) => Type::Array(Box::new(expand(project, file, *element, depth + 1))),
        Type::Map(key, value) => Type::Map(
            Box::new(expand(project, file, *key, depth + 1)),
            Box::new(expand(project, file, *value, depth + 1)),
        ),
        other => other,
    }
}

/// The type of `---@alias name type` in the project of `file`.
fn alias<'a>(project: &'a Project, file: &'a AnalyzedFile, name: &str) -> Option<&'a str> {
    let scope = project.scope_of(file);
    let files = std::iter::once(file).chain(scope.files().map(|(_, f)| &**f));
    files.into_iter().flat_map(|f| annotations::all(f.root())).find_map(|a| {
        let (declared, _, rest) = a.name_and_rest().filter(|_| a.tag == "alias")?;
        (declared == name && !rest.is_empty()).then(|| annotations::type_part(rest))
    })
}

/// The statement declaring the function `call` calls, with its file:
/// `local function f()`, `local f = function () end`, or `function f()` in
/// a file of the project.
pub fn called_declaration<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    call: SyntaxNode<'a>,
) -> Option<(&'a AnalyzedFile, SyntaxNode<'a>)> {
    let ident = CallExpr::cast(call)?.callee().and_then(NameRef::cast)?.ident()?;
    let Some(binding) = resolve(file, ident) else {
        return global_function(project, file, ident.text());
    };
    let token = file.root().token_at_offset(binding.range.start()).right_biased()?;
    let statement = token.ancestors().find(|n| {
        matches!(
            n.kind(),
            SyntaxKind::LOCAL_FUNCTION_STMT | SyntaxKind::FUNCTION_STMT | SyntaxKind::LOCAL_STMT
        )
    })?;
    annotations::function_params(statement).map(|_| (file, statement))
}

/// The declaration of the global function `name`: `function name() end` in
/// `file` first, then in the files of its project.
pub fn global_function<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    name: &str,
) -> Option<(&'a AnalyzedFile, SyntaxNode<'a>)> {
    let others = project.scope_of(file);
    let files = std::iter::once(file).chain(others.files().map(|(_, f)| &**f));
    files.into_iter().find_map(|f| {
        let statement = f.root().descendants().find(|n| {
            n.kind() == SyntaxKind::FUNCTION_STMT && global_function_name(*n) == Some(name)
        })?;
        Some((f, statement))
    })
}

/// `f` of `function f() end`; `None` for `function a.b()`.
pub fn global_function_name(statement: SyntaxNode<'_>) -> Option<&str> {
    let name = statement.child_node(SyntaxKind::FUNC_NAME)?.text().trim();
    name.bytes().all(|b| b == b'_' || b.is_ascii_alphanumeric()).then_some(name)
}

/// The type of the variable declared at `range`: `integer`, `string[]`,
/// `target?`, ...
pub fn variable_type<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    kind: BindingKind<'a>,
    range: TextRange,
) -> Type<'a> {
    variable_type_at(project, file, kind, range, 0)
}

fn variable_type_at<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    kind: BindingKind<'a>,
    range: TextRange,
    depth: u8,
) -> Type<'a> {
    if depth > MAX_DEPTH {
        return Type::Unknown;
    }
    let Some(token) = file.root().token_at_offset(range.start()).right_biased() else {
        return Type::Unknown;
    };
    let name = token.parent().filter(|n| n.kind() == SyntaxKind::NAME);
    let index_in = |list: SyntaxNode<'_>| {
        list.children().filter(|n| n.kind() == SyntaxKind::NAME).position(|n| Some(n) == name)
    };
    let expressions = |statement: SyntaxNode<'a>| -> Vec<SyntaxNode<'a>> {
        statement
            .child_node(SyntaxKind::EXPR_LIST)
            .map(|l| l.children().collect())
            .unwrap_or_default()
    };
    match kind {
        BindingKind::Local => {
            let Some(statement) = token.ancestors().find(|n| n.kind() == SyntaxKind::LOCAL_STMT)
            else {
                return Type::Unknown;
            };
            let index = statement.child_node(SyntaxKind::NAME_LIST).and_then(index_in).unwrap_or(0);
            // `---@type string[]`: the declared type wins, as in LuaLS.
            if let Some(ty) = annotations::declared_type(statement, index) {
                return annotated(project, file, ty);
            }
            let declared = nth_value_type_at(project, file, &expressions(statement), index, depth);
            // As LuaLS: a variable has every type it is given.
            let assigned = assignments(file, statement, range)
                .into_iter()
                .map(|(values, index)| nth_value_type_at(project, file, &values, index, depth + 1));
            match statement.child_node(SyntaxKind::EXPR_LIST) {
                // `local x` then `x = 1`: the declaration gives no value yet.
                None => Type::union(assigned),
                Some(_) => Type::union(std::iter::once(declared).chain(assigned)),
            }
        }
        BindingKind::LoopVar => {
            let Some(statement) = token.ancestors().find(|n| {
                matches!(n.kind(), SyntaxKind::NUMERIC_FOR_STMT | SyntaxKind::GENERIC_FOR_STMT)
            }) else {
                return Type::Unknown;
            };
            let values = expressions(statement);
            if statement.kind() == SyntaxKind::NUMERIC_FOR_STMT {
                // As Lua 5.3+: an integer loop when the start and the step
                // are integers, a float loop otherwise.
                let integral = |nth: usize| {
                    values.get(nth).is_none_or(|v| infer(project, file, *v, depth) == Type::Integer)
                };
                return if integral(0) && integral(2) { Type::Integer } else { Type::Number };
            }
            let index = statement.child_node(SyntaxKind::NAME_LIST).and_then(index_in).unwrap_or(0);
            loop_variable_type(project, file, values.first().copied(), index, depth)
        }
        BindingKind::Param(Some(instance)) => Type::Class(instance.name()),
        BindingKind::Param(None) => {
            let index = token
                .ancestors()
                .find(|n| n.kind() == SyntaxKind::PARAM_LIST)
                .map(|params| (params, index_in(params).unwrap_or(0)));
            let Some((params, index)) = index else {
                return Type::Unknown;
            };
            // `---@param name type` above the function.
            let annotated = annotations::statement_of_params(params)
                .and_then(|s| annotations::param_type(s, token.text()));
            match annotated {
                Some((ty, true)) => Type::union([crate::annotated(project, file, ty), Type::Nil]),
                Some((ty, false)) => crate::annotated(project, file, ty),
                None => callback_param_type(params, index),
            }
        }
        BindingKind::Function => Type::Function(None),
        BindingKind::Import { module } => Type::Module(module),
    }
}

/// The assignments to the local declared at `range` by `statement`: the
/// values of each assignment with the index of the local among its targets.
fn assignments<'a>(
    file: &'a AnalyzedFile,
    statement: SyntaxNode<'a>,
    range: TextRange,
) -> Vec<(Vec<SyntaxNode<'a>>, usize)> {
    let Some(block) = statement.parent() else {
        return Vec::new();
    };
    let name = &file.text[range];
    let mut out = Vec::new();
    let after = statement.text_range().end();
    for assign in block
        .descendants()
        .filter(|n| n.kind() == SyntaxKind::ASSIGN_STMT && n.text_range().start() >= after)
    {
        let targets = assign.children().take_while(|n| n.kind() != SyntaxKind::EXPR_LIST);
        for (index, target) in targets.enumerate() {
            let Some(ident) = NameRef::cast(target).and_then(|n| n.ident()) else {
                continue;
            };
            let same =
                ident.text() == name && resolve(file, ident).is_some_and(|b| b.range == range);
            if same {
                let values = assign.child_node(SyntaxKind::EXPR_LIST);
                out.push((values.map(|l| l.children().collect()).unwrap_or_default(), index));
            }
        }
    }
    out
}

/// Whether the local declared at `range` is assigned after its declaration.
/// Without flow analysis, its type then holds at no point in particular.
pub fn is_reassigned(file: &AnalyzedFile, range: TextRange) -> bool {
    let Some(token) = file.root().token_at_offset(range.start()).right_biased() else {
        return false;
    };
    token
        .ancestors()
        .find(|n| n.kind() == SyntaxKind::LOCAL_STMT)
        .is_some_and(|statement| !assignments(file, statement, range).is_empty())
}

/// The parameters of the file callbacks:
/// `on_build_file(function (target, sourcefile, opt))`,
/// `on_buildcmd_file(function (target, batchcmds, sourcefile, opt))`.
fn callback_param_type(params: SyntaxNode<'_>, index: usize) -> Type<'static> {
    let api = params
        .parent()
        .and_then(|f| f.parent())
        .and_then(|a| a.parent())
        .and_then(CallExpr::cast)
        .and_then(|c| c.callee_name())
        .unwrap_or("");
    let cmd = api.contains("cmd_");
    let (file, files) = (api.ends_with("_file"), api.ends_with("_files"));
    match (index, cmd) {
        (1, true) | (3, true) if file || files => Type::Table,
        (2, true) if file => Type::String,
        (2, true) if files => Type::Array(Box::new(Type::String)),
        (1, false) if file => Type::String,
        (1, false) if files => Type::Array(Box::new(Type::String)),
        (2, false) if file || files => Type::Table,
        _ => Type::Unknown,
    }
}

/// `i, v` in `for i, v in ipairs(list)`, `line` in `for line in io.lines(f)`.
fn loop_variable_type<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    iterator: Option<SyntaxNode<'a>>,
    index: usize,
    depth: u8,
) -> Type<'a> {
    let Some(iterator) = iterator else {
        return Type::Unknown;
    };
    if let Some(call) = CallExpr::cast(iterator)
        && let Some(callee @ ("ipairs" | "pairs")) = call.callee_name()
    {
        let list = call.arg(0).map_or(Type::Unknown, |a| infer(project, file, a, depth + 1));
        return match (callee, index, list.non_nil()) {
            ("ipairs", 0, _) | ("pairs", 0, Type::Array(_)) => Type::Integer,
            ("pairs", 0, Type::Map(key, _)) => *key,
            (_, 1, _) => list.element(),
            _ => Type::Unknown,
        };
    }
    // An iterator of the stubs: `fun(): string`.
    let returns = infer(project, file, iterator, depth + 1).returns();
    returns.into_iter().nth(index).unwrap_or(Type::Unknown)
}

/// The expression ending right before `offset`: the receiver of `x.` or
/// `x:` being typed.
pub fn expression_before(
    file: &AnalyzedFile,
    end: xmake_syntax::TextSize,
) -> Option<SyntaxNode<'_>> {
    let token = file.root().token_at_offset(end).left_biased()?;
    let mut found = None;
    for node in token.ancestors() {
        if node.text_range().end() != end && node.trimmed_range().end() != end {
            break;
        }
        if matches!(
            node.kind(),
            SyntaxKind::NAME_REF
                | SyntaxKind::CALL_EXPR
                | SyntaxKind::METHOD_CALL_EXPR
                | SyntaxKind::PAREN_EXPR
                | SyntaxKind::FIELD_EXPR
                | SyntaxKind::LITERAL
                | SyntaxKind::INDEX_EXPR
        ) {
            found = Some(node);
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, sync::Arc};

    use xmake_api::ApiDb;
    use xmake_syntax::{Dialect, TextSize};

    use super::*;

    /// The type of the expression ending at `$`.
    fn type_at(text: &str) -> String {
        let offset = text.find('$').expect("marker");
        let text = text.replacen('$', "", 1);
        let mut project = Project::new(ApiDb::builtin(), Dialect::XMAKE_DEFAULT);
        let file = project.set_file(PathBuf::from("xmake.lua"), Arc::from(text.as_str()));
        let node = expression_before(&file, TextSize::new(offset as u32)).expect("expression");
        type_of(&project, &file, node).to_string()
    }

    fn in_script(body: &str) -> String {
        type_at(&format!("target('a')\non_load(function (t)\n{body}\nend)"))
    }

    #[test]
    fn inference() {
        assert_eq!(in_script("  t$"), "target");
        assert_eq!(in_script("  local d = t:dep('core')\n  d$"), "target?");
        assert_eq!(in_script("  local p = t:pkg('zlib')\n  p$"), "package?");
        assert_eq!(in_script("  local n = t:name()\n  n$"), "string");
        assert_eq!(in_script("  os$"), "os");
        assert_eq!(in_script("  import('core.base.json')\n  json$"), "core.base.json");
        assert_eq!(in_script("  local x = os.curdir()\n  x$"), "string");
        assert_eq!(type_at("local s = 'a'\nprint(s$)"), "string");
        assert_eq!(
            in_script("  local want = (io.readfile('f') or ''):match('%x+')\n  want$"),
            "string?"
        );
        assert_eq!(in_script("  local h = hash.sha256('f')\n  h$"), "string");
        assert_eq!(type_at("local s = 'a' .. 1\nprint(s$)"), "string");
    }

    #[test]
    fn lua_operators() {
        let at = |expr: &str| type_at(&format!("local c = 1 < 2\nlocal x = {expr}\nprint(x$)"));
        assert_eq!(at("c and 1 or 'x'"), "integer|string");
        assert_eq!(at("nil or 'x'"), "string");
        assert_eq!(at("false or 0"), "integer");
        assert_eq!(at("0 or 'never'"), "integer", "0 is truthy");
        assert_eq!(at("1 + 2"), "integer");
        assert_eq!(at("1 / 2"), "number");
        assert_eq!(at("1 + 2.5"), "number");
        assert_eq!(at("#'abc'"), "integer");
        assert_eq!(at("not c"), "boolean");
        assert_eq!(at("{'a', 'b'}"), "string[]");
        assert_eq!(at("{'a', 1}"), "(string|integer)[]");
        assert_eq!(at("{k = 1}"), "table");
        assert_eq!(at("c"), "boolean");
        assert_eq!(
            type_at(
                "local x = 'a'
x = 1
print(x$)"
            ),
            "string|integer"
        );
        assert_eq!(
            type_at(
                "local x
if true then x = {} end
print(x$)"
            ),
            "table"
        );
    }

    #[test]
    fn annotated() {
        assert_eq!(
            type_at(
                "---@type string[]
local names = {}
print(names$)"
            ),
            "string[]"
        );
        assert_eq!(
            type_at(
                "---@param t target
---@param n? integer
local function f(t, n) print(n$) end"
            ),
            "integer?"
        );
        assert_eq!(
            type_at(
                "---@return string, integer
local function f() end
local a, b = f()
print(b$)"
            ),
            "integer"
        );
        assert_eq!(
            type_at(
                "---@return target
function g() end
local t = g()
print(t$)"
            ),
            "target"
        );
        assert_eq!(
            type_at(
                "local function h() end
local x = h()
print(x$)"
            ),
            "nil"
        );
        assert_eq!(
            type_at(
                "local function h(c)\n  if c then return 1 end\n  return 'a'\nend\nlocal x = h()\nprint(x$)"
            ),
            "integer|string"
        );
        assert_eq!(
            type_at("---@deprecated\nfunction t()\n  return ''\nend\nlocal x = t()\nprint(x$)"),
            "string"
        );
    }

    #[test]
    fn multiple_returns() {
        let body = "  local ok, err = os.execv('git', {'status'}, {try = true})
";
        assert_eq!(in_script(&format!("{body}  print(ok$)")), "integer?");
        assert_eq!(in_script(&format!("{body}  print(err$)")), "string?");
    }

    #[test]
    fn loops() {
        assert_eq!(type_at("for i = 1, 10 do print(i$) end"), "integer");
        assert_eq!(type_at("for i = 1, 10, 0.5 do print(i$) end"), "number");
        assert_eq!(type_at("local l = {'a'}\nfor i, v in ipairs(l) do print(v$) end"), "string");
        assert_eq!(type_at("local l = {'a'}\nfor i, v in pairs(l) do print(i$) end"), "integer");
        assert_eq!(in_script("  for f in io.lines('x') do print(f$) end"), "string");
    }
}
