//! Refactorings offered at the cursor, edits without a diagnostic.
//!
//! The two ways to write a scope, one into the other:
//!
//! ```lua
//! target("app")                     target("app", function ()
//!     set_kind("binary")      <->       set_kind("binary")
//! target_end()                      end)
//! ```

use xmake_api::{Scope, ScopeEdge};
use xmake_syntax::{
    SyntaxKind, SyntaxNode, TextRange, TextSize,
    ast::{AstNode, Block, CallExpr, FunctionExpr},
};

use crate::AnalyzedFile;

/// An edit of the file offered at the cursor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Assist {
    pub title: String,
    pub range: TextRange,
    pub replacement: String,
}

/// The refactorings for the statement at `offset`.
pub fn assists(file: &AnalyzedFile, offset: TextSize) -> Vec<Assist> {
    let root = file.root();
    let stmt = root
        .descendants()
        .filter(|n| n.kind() == SyntaxKind::CALL_STMT && n.text_range().contains_inclusive(offset))
        .last();
    let Some(stmt) = stmt else {
        return Vec::new();
    };
    let Some((call, scope)) = scope_opener(stmt) else {
        return Vec::new();
    };
    let text = &*file.text;
    let block = stmt.parent().and_then(Block::cast);
    let mut out = Vec::new();
    match call.arg(1) {
        None => out.extend(block.and_then(|b| to_function(text, b, stmt, call, scope))),
        Some(arg) if arg.kind() == SyntaxKind::FUNCTION_EXPR && call.arg(2).is_none() => {
            out.extend(block.and_then(|b| to_block(text, b, stmt, call, scope, arg)));
        }
        Some(_) => {}
    }
    out
}

/// The `target(...)`, `option(...)`, ... call of `stmt` and its scope.
fn scope_opener(stmt: SyntaxNode<'_>) -> Option<(CallExpr<'_>, Scope)> {
    let call = stmt.children().find_map(CallExpr::cast)?;
    match Scope::from_opener(call.callee_name()?)? {
        (scope, ScopeEdge::Begin) if call.arg(0).is_some() => Some((call, scope)),
        _ => None,
    }
}

/// The scope call or `_end` call of `stmt`, if it is one.
fn scope_edge(stmt: SyntaxNode<'_>) -> Option<(Scope, ScopeEdge)> {
    let call = stmt.children().find_map(CallExpr::cast)?;
    Scope::from_opener(call.callee_name()?)
}

/// `target("app")` and the statements up to the next scope or `target_end()`
/// into `target("app", function () ... end)`.
fn to_function(
    text: &str,
    block: Block<'_>,
    stmt: SyntaxNode<'_>,
    call: CallExpr<'_>,
    scope: Scope,
) -> Option<Assist> {
    let mut body: Vec<SyntaxNode<'_>> = Vec::new();
    let mut end_call = None;
    for next in block.statements().skip_while(|s| *s != stmt).skip(1) {
        match scope_edge(next) {
            Some((closed, ScopeEdge::End)) if closed == scope => {
                end_call = Some(next);
                break;
            }
            // Another scope (or the `_end` of one) closes this one.
            Some(_) => break,
            None => body.push(next),
        }
    }
    let start = stmt.text_range().end();
    let (body_end, replace_end) = match (end_call, body.last()) {
        (Some(end), _) => (line_start(text, end.text_range().start()), end.text_range().end()),
        (None, Some(last)) => {
            let end = line_end(text, last.text_range().end());
            (end, end)
        }
        (None, None) => (start, start),
    };
    let indent = indent_of(text, stmt.text_range().start());
    let inner = format!("{indent}{}", indent_unit(text, indent));
    let body_text = text[usize::from(start)..usize::from(body_end)].trim_end();
    let (first, rest) = body_text.split_once('\n').unwrap_or((body_text, ""));

    let mut out = opening(text, call)?;
    out.push_str(", function ()");
    out.push_str(first.trim_end());
    for line in reindent(rest.split('\n'), &inner) {
        out.push('\n');
        out.push_str(&line);
    }
    out.push('\n');
    out.push_str(indent);
    out.push_str("end)");
    Some(Assist {
        title: format!("Rewrite as `{}(..., function () ... end)`", scope.name()),
        range: TextRange::new(stmt.text_range().start(), replace_end),
        replacement: out,
    })
}

/// `target("app", function () ... end)` into `target("app")`, the body,
/// and `target_end()` when statements follow.
fn to_block(
    text: &str,
    block: Block<'_>,
    stmt: SyntaxNode<'_>,
    call: CallExpr<'_>,
    scope: Scope,
    function: SyntaxNode<'_>,
) -> Option<Assist> {
    let function = FunctionExpr::cast(function)?;
    let params_end = function.param_list()?.syntax().text_range().end();
    let end_kw = function.syntax().child_token(SyntaxKind::END_KW)?;
    let body_text = &text[usize::from(params_end)..usize::from(end_kw.text_range().start())];
    let (first, rest) = body_text.split_once('\n').unwrap_or((body_text, ""));

    let indent = indent_of(text, stmt.text_range().start());
    let inner = format!("{indent}{}", indent_unit(text, indent));
    let mut lines: Vec<String> = Vec::new();
    if !first.trim().is_empty() {
        lines.push(format!("{inner}{}", first.trim()));
    }
    if !rest.is_empty() {
        lines.extend(reindent(rest.split('\n'), &inner));
    }
    while lines.last().is_some_and(|l| l.is_empty()) {
        lines.pop();
    }

    // `target("app"` of `target("app", function ...`.
    let name = call.arg(0)?;
    let mut out = format!(
        "{})",
        &text[usize::from(stmt.text_range().start())..usize::from(name.text_range().end())]
    );
    for line in &lines {
        out.push('\n');
        out.push_str(line);
    }
    // Without `target_end()`, the statements after would join the scope.
    let followed = block
        .statements()
        .skip_while(|s| *s != stmt)
        .nth(1)
        .is_some_and(|next| !matches!(scope_edge(next), Some((_, ScopeEdge::Begin))));
    if followed {
        out.push('\n');
        out.push_str(indent);
        out.push_str(scope.name());
        out.push_str("_end()");
    }
    Some(Assist {
        title: format!("Rewrite as `{}(...)` ... `{}_end()`", scope.name(), scope.name()),
        range: stmt.text_range(),
        replacement: out,
    })
}

/// `target("app"`: the call up to its closing parenthesis.
fn opening(text: &str, call: CallExpr<'_>) -> Option<String> {
    let start = usize::from(call.syntax().text_range().start());
    let args = call.arg_list()?;
    match args.syntax().child_token(SyntaxKind::R_PAREN) {
        Some(close) => Some(text[start..usize::from(close.text_range().start())].to_owned()),
        // `target "app"`
        None => {
            let callee = call.callee()?.text_range();
            let arg = call.arg(0)?.text_range();
            Some(format!("{}({}", &text[callee], &text[arg]))
        }
    }
}

/// `lines` with their common indentation replaced by `indent`; blank lines
/// empty.
fn reindent<'t>(lines: impl Iterator<Item = &'t str> + Clone, indent: &str) -> Vec<String> {
    let common = lines
        .clone()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.len() - l.trim_start().len())
        .min()
        .unwrap_or(0);
    let mut out: Vec<String> = lines
        .map(|l| {
            let l = l.trim_end();
            if l.trim().is_empty() {
                String::new()
            } else {
                format!("{indent}{}", &l[common.min(l.len())..])
            }
        })
        .skip_while(String::is_empty)
        .collect();
    while out.last().is_some_and(String::is_empty) {
        out.pop();
    }
    out
}

/// The whitespace before the line of `offset`.
fn indent_of(text: &str, offset: TextSize) -> &str {
    let start = usize::from(line_start(text, offset));
    let line = &text[start..];
    &line[..line.len() - line.trim_start_matches([' ', '\t']).len()]
}

/// A tab when the file indents with tabs, else four spaces.
fn indent_unit(text: &str, indent: &str) -> &'static str {
    let tabs = indent.starts_with('\t') || text.lines().any(|l| l.starts_with('\t'));
    if tabs { "\t" } else { "    " }
}

fn line_start(text: &str, offset: TextSize) -> TextSize {
    let at = usize::from(offset);
    TextSize::new(text[..at].rfind('\n').map_or(0, |i| i + 1) as u32)
}

/// The end of the line of `offset`, a comment after it included.
fn line_end(text: &str, offset: TextSize) -> TextSize {
    let at = usize::from(offset);
    let rest = &text[at..];
    let line = rest.split('\n').next().unwrap_or(rest).trim_end_matches('\r');
    if line.trim_start().starts_with("--") {
        TextSize::new((at + line.trim_end().len()) as u32)
    } else {
        offset
    }
}

#[cfg(test)]
mod tests {
    use xmake_syntax::Dialect;

    use super::*;

    fn rewrite(text: &str) -> Option<String> {
        let marker = text.find('$').expect("marker");
        let text = text.replace('$', "");
        let file = AnalyzedFile::new(text.clone().into(), "xmake.lua", Dialect::XMAKE_DEFAULT);
        let assist = assists(&file, TextSize::new(marker as u32)).into_iter().next()?;
        let mut out = text;
        out.replace_range(std::ops::Range::<usize>::from(assist.range), &assist.replacement);
        Some(out)
    }

    #[test]
    fn block_to_function() {
        assert_eq!(
            rewrite("tar$get(\"app\")\n    set_kind(\"binary\")\n    -- sources\n    add_files(\"*.c\")\ntarget_end()\n\nprint(1)\n").as_deref(),
            Some("target(\"app\", function ()\n    set_kind(\"binary\")\n    -- sources\n    add_files(\"*.c\")\nend)\n\nprint(1)\n"),
        );
        // Without `target_end()`: up to the next scope.
        assert_eq!(
            rewrite("$target(\"a\")\nset_kind(\"binary\") -- kind\n\ntarget(\"b\")\n").as_deref(),
            Some(
                "target(\"a\", function ()\n    set_kind(\"binary\") -- kind\nend)\n\ntarget(\"b\")\n"
            ),
        );
        assert_eq!(
            rewrite("$option(\"x\")\n").as_deref(),
            Some("option(\"x\", function ()\nend)\n"),
        );
    }

    #[test]
    fn function_to_block() {
        assert_eq!(
            rewrite("$target(\"app\", function ()\n    set_kind(\"binary\")\n\n    add_files(\"*.c\")\nend)\nprint(1)\n").as_deref(),
            Some("target(\"app\")\n    set_kind(\"binary\")\n\n    add_files(\"*.c\")\ntarget_end()\nprint(1)\n"),
        );
        // The last statement, or before another scope: no `target_end()`.
        assert_eq!(
            rewrite("$rule(\"r\", function () add_deps(\"c++\") end)\ntarget(\"b\")\n").as_deref(),
            Some("rule(\"r\")\n    add_deps(\"c++\")\ntarget(\"b\")\n"),
        );
    }

    #[test]
    fn round_trip() {
        let block = "target(\"app\")\n    set_kind(\"binary\")\ntarget_end()\nprint(1)\n";
        let function = rewrite(&format!("${block}")).expect("to function");
        assert_eq!(rewrite(&format!("${function}")).as_deref(), Some(block));
    }

    #[test]
    fn nothing_else() {
        assert_eq!(rewrite("$set_kind(\"binary\")\n"), None);
        assert_eq!(rewrite("$target(\"a\", {kind = \"binary\"})\n"), None);
        assert_eq!(rewrite("$target_end()\n"), None);
    }
}
