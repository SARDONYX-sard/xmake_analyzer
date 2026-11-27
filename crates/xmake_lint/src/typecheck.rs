//! `mismatched-type`: arguments of the wrong type.
//!
//! The expected types come from the prototypes of the manual for the
//! description APIs (`set_default(default: boolean)`) and from the stubs for
//! the script functions (`os.cp(src: string, ...)`). An argument is checked
//! when its type is known; as in TypeScript, a union argument must fit as a
//! whole (`string|boolean` does not fit `string`). Whether a value may be
//! `nil` is left out of the check.

use std::borrow::Cow;

use xmake_syntax::{
    SyntaxKind, SyntaxNode, TextRange,
    ast::{AstNode, CallExpr, MethodCallExpr, NameRef},
};

use crate::{
    AnalyzedFile, Diagnostic, Fix, Project, Severity, Type,
    resolve::{BindingKind, resolve},
};

/// Checks the arguments of every call of `file`.
pub(crate) fn check<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    out: &mut Vec<Diagnostic<'a>>,
) {
    for node in file.root().descendants() {
        let args = match node.kind() {
            SyntaxKind::CALL_EXPR => CallExpr::cast(node).and_then(|c| c.arg_list()),
            SyntaxKind::METHOD_CALL_EXPR => MethodCallExpr::cast(node).and_then(|c| c.arg_list()),
            _ => continue,
        };
        let (Some(args), Some(expected)) =
            (args, xmake_types::expected_params(project, file, node))
        else {
            continue;
        };
        let mut params = expected.iter();
        let mut current: Option<&xmake_types::Expected<'_>> = None;
        for arg in args.args() {
            // Past a `...`, its type applies to the rest.
            if !current.is_some_and(|c| c.variadic) {
                current = params.next();
            }
            let Some(param) = current else {
                break;
            };
            let actual = argument_type(project, file, arg);
            if actual.is_assignable_to(&param.ty) {
                continue;
            }
            let message = format!("`{}` expects `{}`, found `{actual}`", param.name, param.ty);
            let range = arg.trimmed_range();
            // `"true"` for a boolean: the quotes are a mistake.
            let text = arg.text().trim();
            let unquoted = text.trim_matches(['"', '\'']);
            let fix = (Type::Boolean.is_assignable_to(&param.ty)
                && matches!(unquoted, "true" | "false")
                && unquoted != text)
                .then(|| Fix {
                    title: Cow::Owned(format!("Replace with `{unquoted}`")),
                    range,
                    replacement: Cow::Owned(unquoted.to_owned()),
                    safe: true,
                });
            out.push(Diagnostic {
                range,
                severity: Severity::Warning,
                message: Cow::Owned(message),
                code: "mismatched-type",
                fixes: fix.into_iter().collect(),
            });
        }
    }
    comparisons(project, file, out);
}

/// `target:kind() == "shard"`: a literal the other side never holds, as
/// TypeScript says of comparisons that cannot be true.
fn comparisons<'a>(project: &'a Project, file: &'a AnalyzedFile, out: &mut Vec<Diagnostic<'a>>) {
    let literals = file.root().descendants().filter(|n| {
        n.kind() == SyntaxKind::LITERAL && n.first_token().is_some_and(|t| t.kind().is_string())
    });
    for literal in literals {
        let Some(other) = xmake_types::compared_with(literal) else {
            continue;
        };
        let (Type::Literal(value), expected) = (
            xmake_types::type_of(project, file, literal),
            xmake_types::type_of(project, file, other),
        ) else {
            continue;
        };
        let expected = expected.non_nil();
        let values: Vec<&str> = expected
            .members()
            .iter()
            .filter_map(|m| if let Type::Literal(v) = m { Some(*v) } else { None })
            .collect();
        // Only a union of literals tells every value.
        if values.is_empty() || values.len() != expected.members().len() || values.contains(&value)
        {
            continue;
        }
        let range = literal.trimmed_range();
        let content = TextRange::new(
            range.start() + xmake_syntax::TextSize::new(1),
            range.end() - xmake_syntax::TextSize::new(1),
        );
        let fix =
            crate::fuzzy::closest(value, values.iter().copied()).map(|c| Fix::replace(content, c));
        out.push(Diagnostic {
            range,
            severity: Severity::Warning,
            message: Cow::Owned(format!(
                "`{}` is never `\"{value}\"`: it is `{expected}`",
                other.text().trim()
            )),
            code: "mismatched-type",
            fixes: fix.into_iter().collect(),
        });
    }
}

/// The type of an argument, `nil` aside.
fn argument_type<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
    arg: SyntaxNode<'a>,
) -> Type<'a> {
    let binding = (arg.kind() == SyntaxKind::NAME_REF)
        .then(|| NameRef::cast(arg).and_then(|n| n.ident()))
        .flatten()
        .and_then(|ident| resolve(file, ident));
    let uncertain = binding.is_some_and(|b| match b.kind {
        // Imported modules are the values xmake gives: typed loosely.
        BindingKind::Import { .. } => true,
        // `local p = os.getenv("PATH")` then `p = path.splitenv(p)`: which
        // type holds here needs flow analysis.
        BindingKind::Local => xmake_types::is_reassigned(file, b.range),
        _ => false,
    });
    if uncertain {
        return Type::Unknown;
    }
    // A `nil` argument stands for an omitted one.
    xmake_types::type_of(project, file, arg).non_nil()
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, sync::Arc};

    use xmake_api::ApiDb;
    use xmake_syntax::Dialect;

    use super::*;

    fn check_text(text: &str) -> Vec<String> {
        let mut project = Project::new(ApiDb::builtin(), Dialect::XMAKE_DEFAULT);
        let file = project.set_file(PathBuf::from("xmake.lua"), Arc::from(text));
        let mut out = Vec::new();
        check(&project, &file, &mut out);
        out.sort_by_key(|d| d.range.start());
        out.iter().map(|d| format!("{} {}", &text[d.range], d.message)).collect()
    }

    #[test]
    fn mismatches() {
        let text = "target('a')
    set_default('false')
    set_default(false)
    set_kind(42)
    set_warnings('all', 'error', {private = true})
    set_warnings('all', 1)
    add_files('a.c', 'b.c', {defines = 'X'})
    add_files(1)
    on_load(function (t)
        os.cp('a', 1)
        local n = t:name()
        os.cp(n, 'b')
        print(t:dep(1))
        local c = 1 < 2
        os.cp(c and 'a' or 'b', 'c')
        os.cp(c and 'a' or false, 'c')
        on_build('windows', 'linux', function (target) end, {a = 1})
        on_build(1, function (target) end)
        ---@param name string
        ---@param count? integer
        local function greet(name, count) end
        greet('a', 1)
        greet(1, 'x')
        ---@return target
        local function main() end
        os.cp(main(), 'b')
        ---@alias Std \"c++20\"|\"c++23\"
        ---@param std Std
        local function use(std) end
        use('c++23')
        use('c++17')
        if t:kind() == 'shard' or os.host() ~= 'windows' then end
        ---@param o os
        local function uses(o) end
        uses(os)
        local p = os.getenv('PATH')
        p = {}
        table.join2(p, 'x')
    end)
";
        assert_eq!(
            check_text(text),
            [
                "'false' `default` expects `boolean`, found `\"false\"`",
                "42 `kind` expects `string`, found `integer`",
                "1 `warnings` expects `string|table`, found `integer`",
                "1 `files` expects `string|any[]`, found `integer`",
                "1 `dst` expects `string`, found `integer`",
                "1 `name` expects `string`, found `integer`",
                "c and 'a' or false `src` expects `string`, found `\"a\"|false`",
                "1 `pattern` expects `string`, found `integer`",
                "1 `name` expects `string`, found `integer`",
                "'x' `count` expects `integer?`, found `\"x\"`",
                "main() `src` expects `string`, found `target`",
                "'c++17' `std` expects `\"c++20\"|\"c++23\"`, found `\"c++17\"`",
                "'shard' `t:kind()` is never `\"shard\"`: it is `\"binary\"|\"static\"|\"shared\"|\"object\"|\"headeronly\"|\"moduleonly\"|\"phony\"`",
            ]
        );
    }
}
