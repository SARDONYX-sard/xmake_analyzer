//! Signature help: the parameters of the call around the cursor.

use std::{borrow::Cow, fmt::Write as _};

use xmake_api::{ArgValues, Scope, api_doc, api_params, summary};
use xmake_syntax::{
    SyntaxKind, SyntaxNode, TextRange, TextSize,
    ast::{AstNode, CallExpr, FieldExpr, MethodCallExpr, NameRef},
};

use crate::{
    AnalyzedFile, Project,
    context::CodeContext,
    resolve::{BindingKind, resolve},
};

/// A signature with its parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignatureHelp<'a> {
    /// `set_kind(kind: string)`
    pub label: String,
    /// Ranges of the parameters in `label`, in bytes.
    pub params: Vec<(u32, u32)>,
    pub documentation: Option<Cow<'a, str>>,
    /// Documentation of the active parameter, e.g. its accepted values.
    pub active_param_doc: Option<String>,
    pub active_param: u32,
}

/// The signature of the call around `offset`.
pub fn signature_help<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    offset: TextSize,
) -> Option<SignatureHelp<'a>> {
    let root = file.root();
    // Start from the last significant token before the cursor: trailing
    // blanks of an unclosed call belong to an outer node.
    let mut token = root.token_at_offset(offset).left_biased()?;
    while token.kind().is_trivia() && token.text_range().start() > TextSize::new(0) {
        token = token.prev_token()?;
    }
    let (call, args) = token.ancestors().find_map(|n| {
        let args = n.child_node(SyntaxKind::ARG_LIST)?;
        let is_call = matches!(n.kind(), SyntaxKind::CALL_EXPR | SyntaxKind::METHOD_CALL_EXPR);
        (is_call && inside_parens(args, offset)).then_some((n, args))
    })?;
    let index = args
        .child_tokens()
        .filter(|t| t.kind() == SyntaxKind::COMMA && t.text_range().end() <= offset)
        .count();
    let current_arg = args.children().nth(index);

    let (name, mut params, documentation) = callee_signature(project, file, call)?;
    let mut active = index.min(params.len().saturating_sub(1));
    let is_script = params.len() == 1 && params[0].starts_with("script: fun(");
    let is_string = |arg: Option<SyntaxNode<'_>>| {
        arg.and_then(|a| a.first_token()).is_some_and(|t| t.kind().is_string())
    };
    if is_script {
        // `on_build("windows", |`: the overload with patterns.
        if index > 0 && is_string(args.children().next()) {
            let script = params.pop().unwrap_or(Cow::Borrowed("script"));
            params = vec![Cow::Borrowed("pattern: string"), Cow::Borrowed("...: string"), script];
            active = match current_arg {
                Some(arg) if arg.kind() == SyntaxKind::TABLE_EXPR => 2,
                arg if is_string(arg) => 1,
                _ => 2,
            };
        }
    } else if let Some(v) = params.iter().position(|p| p.contains("...")) {
        // `add_files("a", "b", {public = true})`: the table is the options.
        let options = params.len() > v + 1;
        let is_table = current_arg.is_some_and(|a| a.kind() == SyntaxKind::TABLE_EXPR);
        active = if options && is_table { v + 1 } else { index.min(v) };
    }

    let mut label = String::with_capacity(64);
    label.push_str(&name);
    label.push('(');
    let mut ranges = Vec::with_capacity(params.len());
    for (i, param) in params.iter().enumerate() {
        if i > 0 {
            label.push_str(", ");
        }
        let start = label.len() as u32;
        label.push_str(param);
        ranges.push((start, label.len() as u32));
    }
    label.push(')');

    let active_param_doc = CallExpr::cast(call).and_then(|c| values_doc(project, file, c, index));
    Some(SignatureHelp {
        label,
        params: ranges,
        documentation,
        active_param_doc,
        active_param: active as u32,
    })
}

fn inside_parens(args: SyntaxNode<'_>, offset: TextSize) -> bool {
    let Some(open) = args.child_token(SyntaxKind::L_PAREN) else {
        return false;
    };
    let closed_before =
        args.child_token(SyntaxKind::R_PAREN).is_some_and(|r| r.text_range().start() < offset);
    open.text_range().end() <= offset && !closed_before
}

type Signature<'a> = (Cow<'a, str>, Vec<Cow<'a, str>>, Option<Cow<'a, str>>);

/// The name, parameters and documentation of the function `call` calls.
fn callee_signature<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    call: SyntaxNode<'a>,
) -> Option<Signature<'a>> {
    if let Some(method) = MethodCallExpr::cast(call) {
        let owner = crate::type_of(project, file, method.receiver()?).owner()?;
        let name = method.method()?.text();
        return Some(stub_signature(owner, ':', name));
    }
    let call = CallExpr::cast(call)?;
    let callee = call.callee()?;
    if let Some(field) = FieldExpr::cast(callee) {
        if let Some(module) = crate::type_of(project, file, field.base()?).module() {
            return Some(stub_signature(module, '.', field.field()?.text()));
        }
        let base = NameRef::cast(field.base()?)?;
        let function = field.field()?.text();
        let name = format!("{}.{function}", base.text());
        let doc = xmake_api::module_doc(base.text(), function).map(|d| Cow::Borrowed(summary(d)));
        return Some((Cow::Owned(name), vec![Cow::Borrowed("...")], doc));
    }
    let name_ref = NameRef::cast(callee)?;
    let ident = name_ref.ident()?;
    let name = ident.text();
    if let Some(binding) = resolve(file, ident) {
        let BindingKind::Function = binding.kind else {
            return None;
        };
        return Some((Cow::Borrowed(name), function_params(file, binding.range), None));
    }
    let db = &project.db;
    let scope = match CodeContext::at(file, ident.text_range().start()) {
        CodeContext::Description(scope) => scope,
        CodeContext::Script => Scope::Root,
    };
    let api = db
        .visible_api(scope, name)
        .or_else(|| db.scopes_of(name).next().and_then(|s| db.scope_api(s, name)));
    let known = api.is_some()
        || Scope::from_opener(name).is_some()
        || db.is_description_function(name)
        || db.script_functions().iter().any(|f| f == name);
    if !known {
        return None;
    }
    let api_scope = api.map_or(scope, |a| a.scope);
    // The prototype of the manual when there is one: `set_default(default: boolean)`.
    let params = match xmake_api::doc_prototype(Some(api_scope), name) {
        Some((prototype, _)) => {
            let params = xmake_api::prototype_params(prototype).into_iter();
            params.map(xmake_types::luacats_param).collect()
        }
        None => api_params(name, api.map(|a| a.kind), api_scope)
            .iter()
            .map(|p| Cow::Borrowed(*p))
            .collect(),
    };
    let doc = api_doc(Some(api_scope), name).map(|d| Cow::Borrowed(summary(d)));
    Some((Cow::Borrowed(name), params, doc))
}

/// The signature of `owner.name` / `owner:name` from the stubs, or an
/// untyped one.
fn stub_signature<'a>(owner: &'a str, separator: char, name: &'a str) -> Signature<'a> {
    let short = owner.rsplit('.').next().unwrap_or(owner);
    let label = Cow::Owned(format!("{short}{separator}{name}"));
    let Some(function) = xmake_api::Stubs::get().member(owner, name) else {
        return (label, vec![Cow::Borrowed("...")], None);
    };
    // `s:split(",")` calls `string.split(s, ",")`: `s` is given.
    let implicit_self = separator == ':' && !function.method;
    let params = function
        .params
        .iter()
        .skip(usize::from(implicit_self))
        .map(|p| Cow::Owned(format!("{}{}: {}", p.name, if p.optional { "?" } else { "" }, p.ty)))
        .collect();
    (label, params, (!function.doc.is_empty()).then(|| Cow::Borrowed(summary(&function.doc))))
}

/// The parameter names of the function declared at `name_range`.
fn function_params(file: &AnalyzedFile, name_range: TextRange) -> Vec<Cow<'_, str>> {
    let root = file.root();
    let function = root.covering_node(name_range);
    let function = std::iter::once(function)
        .chain(function.ancestors())
        .find(|n| matches!(n.kind(), SyntaxKind::FUNCTION_STMT | SyntaxKind::LOCAL_FUNCTION_STMT));
    let Some(params) = function.and_then(|f| f.child_node(SyntaxKind::PARAM_LIST)) else {
        return Vec::new();
    };
    let names = params.children().filter_map(|n| n.first_token()).map(|t| Cow::Borrowed(t.text()));
    let vararg = params.child_token(SyntaxKind::DOT3).map(|_| Cow::Borrowed("..."));
    names.chain(vararg).collect()
}

/// "One of: ..." for arguments with known values.
fn values_doc(
    project: &Project,
    file: &AnalyzedFile,
    call: CallExpr<'_>,
    index: usize,
) -> Option<String> {
    let known = &project.db.values;
    let list: Vec<&str> = match xmake_project::call_arg_values(file, call, index)? {
        ArgValues::Fixed(list) => list.iter().map(|(v, _)| *v).collect(),
        ArgValues::Platforms => known.platforms.iter().map(|v| &**v).collect(),
        ArgValues::Architectures => known.architectures.iter().map(|v| &**v).collect(),
        ArgValues::Modes => known.modes.iter().map(|v| &**v).collect(),
        _ => return None,
    };
    let mut doc = String::from("One of: ");
    for (i, value) in list.iter().enumerate() {
        let _ = write!(doc, "{}`{value}`", if i == 0 { "" } else { ", " });
    }
    Some(doc)
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, sync::Arc};

    use xmake_api::ApiDb;
    use xmake_syntax::Dialect;

    use super::*;

    fn help(text: &str) -> Option<(String, u32)> {
        let offset = text.find('$').expect("marker");
        let text = text.replacen('$', "", 1);
        let mut project = Project::new(ApiDb::builtin(), Dialect::XMAKE_DEFAULT);
        let file = project.set_file(PathBuf::from("xmake.lua"), Arc::from(text.as_str()));
        signature_help(&project, &file, TextSize::new(offset as u32))
            .map(|s| (s.label, s.active_param))
    }

    #[test]
    fn signatures() {
        // The prototypes of the manual: the second file is the `...`.
        let files = "add_files(files: string|any[], ..., {defines, languages, includedirs, rules, force, sourcekind, flags})";
        assert_eq!(
            help(
                "target('a')
  add_files('a.c', $"
            ),
            Some((files.to_owned(), 1))
        );
        assert_eq!(
            help(
                "target('a')
  add_files('a.c', {$})"
            ),
            Some((files.to_owned(), 2))
        );
        assert_eq!(
            help(
                "target('a')
  set_default($"
            ),
            Some(("set_default(default: boolean)".to_owned(), 0))
        );
        assert_eq!(
            help(
                "local function f(a, b) end
f(1, $)"
            ),
            Some(("f(a, b)".to_owned(), 1))
        );
        assert!(
            help("set_policy('x', $)")
                .is_some_and(|(label, active)| label.starts_with("set_policy(") && active == 1)
        );
        assert_eq!(help("print('x')$"), None);
        // The script APIs: the script alone, or after patterns.
        let alone = "on_build(script: fun(target: target)|string)";
        assert_eq!(
            help(
                "target('a')
  on_build($"
            ),
            Some((alone.to_owned(), 0))
        );
        let patterns = "on_build(pattern: string, ...: string, script: fun(target: target)|string)";
        assert_eq!(
            help(
                "target('a')
  on_build('windows', $"
            ),
            Some((patterns.to_owned(), 2))
        );
        assert_eq!(
            help(
                "target('a')
  on_build('windows', 'li$"
            ),
            Some((patterns.to_owned(), 1))
        );
        assert_eq!(
            help(
                "target('a')
on_load(function (t) os.cp('a', $) end)"
            ),
            Some((
                "os.cp(src: string, dst: string, opt?: {rootdir?: string, symlink?: boolean})"
                    .to_owned(),
                1
            ))
        );
    }
}
