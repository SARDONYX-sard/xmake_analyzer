//! Edits shared by the quick fixes: removing an argument, inserting a
//! statement, allowing a lint.

use std::borrow::Cow;

use xmake_syntax::{
    SyntaxKind, SyntaxNode, TextRange, TextSize,
    ast::{AstNode, CallExpr},
};

use crate::{AnalyzedFile, Diagnostic, Fix, LineCol, lints};

/// The indentation of the line holding `offset`.
fn indentation_at(file: &AnalyzedFile, offset: TextSize) -> &str {
    let line = file.line_index.line_col(&file.text, offset).line;
    let start: usize = file.line_index.offset(&file.text, LineCol { line, col: 0 }).into();
    let rest = &file.text[start..];
    &rest[..rest.len() - rest.trim_start_matches([' ', '\t']).len()]
}

/// The start of the line holding `offset`.
fn line_start(file: &AnalyzedFile, offset: TextSize) -> TextSize {
    let line = file.line_index.line_col(&file.text, offset).line;
    file.line_index.offset(&file.text, LineCol { line, col: 0 })
}

/// Removes the argument of a call whose string content is at `content`,
/// with its comma; the whole statement when it is the only argument.
pub(crate) fn remove_argument<'a>(
    file: &'a AnalyzedFile,
    content: TextRange,
    title: String,
) -> Option<Fix<'a>> {
    let root = file.root();
    let literal = root.token_at_offset(content.start()).right_biased()?.parent()?;
    let args = literal.parent().filter(|p| p.kind() == SyntaxKind::ARG_LIST)?;
    let values: Vec<SyntaxNode<'_>> = args.children().collect();
    let index = values.iter().position(|n| *n == literal)?;
    let range = if values.len() == 1 {
        // `add_deps("self")` alone: the whole line.
        let statement = args.ancestors().find(|n| n.kind() == SyntaxKind::CALL_STMT)?;
        let start = line_start(file, statement.text_range().start());
        let end = statement.text_range().end();
        let end = if file.text[usize::from(end)..].starts_with("\r\n") {
            end + TextSize::new(2)
        } else if file.text[usize::from(end)..].starts_with('\n') {
            end + TextSize::new(1)
        } else {
            end
        };
        TextRange::new(start, end)
    } else if let Some(next) = values.get(index + 1) {
        // `"a", "b"` -> `"b"`: up to the next argument.
        TextRange::new(literal.text_range().start(), next.text_range().start())
    } else {
        // `"a", "b"` -> `"a"`: from the end of the previous one.
        TextRange::new(values[index - 1].text_range().end(), literal.text_range().end())
    };
    Some(Fix { title: Cow::Owned(title), range, replacement: Cow::Borrowed(""), safe: true })
}

/// Inserts `add_requires("name")` after the last `add_requires` of the
/// file, or before its first statement.
pub(crate) fn insert_require<'a>(file: &'a AnalyzedFile, name: &str) -> Option<Fix<'a>> {
    let root = file.root();
    let block = root.child_node(SyntaxKind::BLOCK)?;
    let statements: Vec<SyntaxNode<'_>> = block.children().collect();
    let last_require = statements.iter().rev().find(|s| {
        s.children().find_map(CallExpr::cast).and_then(|c| c.callee_name()) == Some("add_requires")
    });
    let line = format!("add_requires(\"{name}\")");
    let (offset, replacement) = match last_require {
        Some(statement) => (statement.text_range().end(), format!("\n{line}")),
        None => (
            statements.first().map_or(TextSize::new(0), |s| s.text_range().start()),
            format!("{line}\n\n"),
        ),
    };
    Some(Fix {
        title: Cow::Owned(format!("Add `{line}`")),
        range: TextRange::empty(offset),
        replacement: Cow::Owned(replacement),
        safe: true,
    })
}

/// Inserts `import("module")` at the start of the function around
/// `offset`, or of the file.
pub fn insert_import<'a>(file: &'a AnalyzedFile, offset: TextSize, module: &str) -> Fix<'a> {
    let root = file.root();
    let node = root.covering_node(TextRange::empty(offset));
    let body = std::iter::once(node)
        .chain(node.ancestors())
        .filter(|n| n.kind() == SyntaxKind::BLOCK)
        .find(|b| {
            b.parent().is_some_and(|p| p.kind() != SyntaxKind::SOURCE_FILE && is_function(p))
        });
    let first =
        body.or_else(|| root.child_node(SyntaxKind::BLOCK)).and_then(|b| b.children().next());
    let at = first.map_or(TextSize::new(0), |s| s.text_range().start());
    let indent = first.map_or("", |s| indentation_at(file, s.text_range().start()));
    let line = format!("import(\"{module}\")");
    Fix {
        title: Cow::Owned(format!("Add `{line}`")),
        range: TextRange::empty(at),
        replacement: Cow::Owned(format!("{line}\n{indent}")),
        safe: false,
    }
}

fn is_function(node: SyntaxNode<'_>) -> bool {
    matches!(
        node.kind(),
        SyntaxKind::FUNCTION_EXPR | SyntaxKind::FUNCTION_STMT | SyntaxKind::LOCAL_FUNCTION_STMT
    )
}

/// `-- xmake-analyzer: allow(<lint>)` above the line of `diagnostic`.
pub fn allow_fix<'a>(file: &'a AnalyzedFile, diagnostic: &Diagnostic<'_>) -> Option<Fix<'a>> {
    lints::lint(diagnostic.code)?;
    let start = line_start(file, diagnostic.range.start());
    let indent = indentation_at(file, start);
    Some(Fix {
        title: Cow::Owned(format!("Allow `{}` here", diagnostic.code)),
        range: TextRange::empty(start),
        replacement: Cow::Owned(format!("{indent}-- xmake-analyzer: allow({})\n", diagnostic.code)),
        safe: false,
    })
}

/// Applies the safe fixes of `diagnostics` to `text`; overlapping fixes after
/// the first are skipped. Returns the new text and the number applied.
pub fn apply_safe_fixes(text: &str, diagnostics: &[Diagnostic<'_>]) -> (String, usize) {
    let mut fixes: Vec<&Fix<'_>> =
        diagnostics.iter().filter_map(|d| d.fixes.iter().find(|f| f.safe)).collect();
    fixes.sort_by_key(|f| (f.range.start(), f.range.end()));
    let mut out = String::with_capacity(text.len());
    let mut last = TextSize::new(0);
    let mut applied = 0;
    for fix in fixes {
        if fix.range.start() < last {
            continue;
        }
        out.push_str(&text[TextRange::new(last, fix.range.start())]);
        out.push_str(&fix.replacement);
        last = fix.range.end();
        applied += 1;
    }
    out.push_str(&text[usize::from(last)..]);
    (out, applied)
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, sync::Arc};

    use xmake_api::ApiDb;
    use xmake_syntax::Dialect;

    use crate::{Project, diagnostics};

    /// The text after applying the fix titled `title`.
    fn apply(text: &str, title: &str) -> String {
        let mut project = Project::new(ApiDb::builtin(), Dialect::XMAKE_DEFAULT);
        let file = project.set_file(PathBuf::from("xmake.lua"), Arc::from(text));
        let diags = diagnostics(&project, &file, &crate::LintLevels::default());
        let fix = diags.iter().flat_map(|d| d.fixes.iter()).find(|f| f.title == title);
        let fix = fix.unwrap_or_else(|| {
            let titles: Vec<_> =
                diags.iter().flat_map(|d| d.fixes.iter().map(|f| f.title.to_string())).collect();
            panic!("no fix {title:?} in {titles:?}")
        });
        let mut out = text.to_owned();
        out.replace_range(std::ops::Range::<usize>::from(fix.range), &fix.replacement);
        out
    }

    #[test]
    fn requires() {
        let text =
            "add_requires('zlib', 'fmt')\ntarget('app')\n    add_packages('zlib', 'openssl')\n";
        assert_eq!(
            apply(text, "Add `add_requires(\"openssl\")`"),
            "add_requires('zlib', 'fmt')\nadd_requires(\"openssl\")\ntarget('app')\n    add_packages('zlib', 'openssl')\n"
        );
        assert_eq!(
            apply(text, "Remove `fmt`"),
            "add_requires('zlib')\ntarget('app')\n    add_packages('zlib', 'openssl')\n"
        );
    }

    #[test]
    fn self_dependency() {
        let text = "target('a')\n    add_deps('a')\n    set_kind('binary')\n";
        assert_eq!(apply(text, "Remove `a`"), "target('a')\n    set_kind('binary')\n");
    }

    #[test]
    fn import() {
        let text = "target('a')\non_load(function (target)\n    print(json.encode({}))\nend)\n";
        assert_eq!(
            apply(text, "Add `import(\"core.base.json\")`"),
            "target('a')\non_load(function (target)\n    import(\"core.base.json\")\n    print(json.encode({}))\nend)\n"
        );
    }

    #[test]
    fn allow_and_safe_fixes() {
        let text = "target('a')\n    local x = 1\n    add_subdirs('lib')\noption_end()\n";
        let mut project = Project::new(ApiDb::builtin(), Dialect::XMAKE_DEFAULT);
        let file = project.set_file(PathBuf::from("xmake.lua"), Arc::from(text));
        let diags = diagnostics(&project, &file, &crate::LintLevels::default());
        let unused = diags.iter().find(|d| d.code == "unused-local").expect("unused-local");
        let allow = super::allow_fix(&file, unused).expect("allow fix");
        assert_eq!(allow.replacement, "    -- xmake-analyzer: allow(unused-local)\n");
        // Safe: the deprecated API and the scope end; not the rename.
        let (fixed, applied) = super::apply_safe_fixes(text, &diags);
        assert_eq!(applied, 2);
        assert_eq!(fixed, "target('a')\n    local x = 1\n    includes('lib')\ntarget_end()\n");
    }
}
