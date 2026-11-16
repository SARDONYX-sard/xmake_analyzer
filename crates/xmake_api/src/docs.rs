//! Documentation of the APIs, in the style of rustdoc: a summary, then
//! `# Examples` and `# Notes`.
//!
//! The hand written pages live in `docs/` (see `build.rs`); every other API
//! gets a page generated from its kind, so that each hover has examples.

use std::fmt::Write as _;

use crate::{ApiKind, Instance, Scope};

/// The page of a description API or a script function, in `scope` when
/// given: `xmake.option` overrides `xmake.api` in an option.
pub fn api_doc(scope: Option<Scope>, name: &str) -> Option<&'static str> {
    let stubs = crate::Stubs::get();
    let scoped = scope.and_then(|scope| {
        let owner = match scope {
            Scope::Root => "xmake.root",
            Scope::Target => "xmake.target",
            Scope::Option => "xmake.option",
            Scope::Rule => "xmake.rule",
            Scope::Package => "xmake.package",
            Scope::Toolchain => "xmake.toolchain",
            Scope::Task => "xmake.task",
        };
        stubs.member(owner, name)
    });
    let function = scoped
        .or_else(|| stubs.member("xmake.api", name))
        .or_else(|| stubs.member(crate::stubs::GLOBALS, name))?;
    (!function.doc.is_empty()).then_some(function.doc.as_str())
}

/// The page of `module.function`, from the stubs.
pub fn module_doc(module: &str, function: &str) -> Option<&'static str> {
    let doc = &crate::Stubs::get().member(module, function)?.doc;
    (!doc.is_empty()).then_some(doc.as_str())
}

/// The page of `instance:method`, from the stubs.
pub fn instance_doc(instance: Instance, method: &str) -> Option<&'static str> {
    module_doc(instance.name(), method)
}

/// The page of the manual documenting the APIs of `scope`.
fn scope_page(scope: Scope) -> &'static str {
    match scope {
        Scope::Root => "api/description/global-interfaces",
        Scope::Target => "api/description/project-target",
        Scope::Option => "api/description/configuration-option",
        Scope::Rule => "api/description/custom-rule",
        Scope::Package => "api/description/package-dependencies",
        Scope::Toolchain => "api/description/custom-toolchain",
        Scope::Task => "api/description/plugin-and-task",
    }
}

fn url(page: &str, anchor: &str) -> String {
    if anchor.is_empty() {
        format!("https://xmake.io/{page}.html")
    } else {
        format!("https://xmake.io/{page}.html#{anchor}")
    }
}

/// The section of the manual documenting `name`: the page of `scope` when it
/// has one (`set_kind` of a target and of a package differ), else the page of
/// the conditions, the global interfaces, ... (`set_license` is documented
/// with the xpack interfaces).
pub fn doc_link(scope: Option<Scope>, name: &str) -> Option<String> {
    use crate::doc_links::DOC_LINKS;
    let entry = pick(DOC_LINKS, scope, name, |e| e.0, |e| e.1)?;
    Some(url(entry.1, entry.2))
}

/// The entry of `name` in a table sorted by name, on the page of `scope`
/// when there is one, else on the global pages, else the first.
fn pick<T>(
    table: &'static [T],
    scope: Option<Scope>,
    name: &str,
    name_of: impl Fn(&T) -> &str,
    page_of: impl Fn(&T) -> &str,
) -> Option<&'static T> {
    let start = table.partition_point(|e| name_of(e) < name);
    let entries =
        &table[start..table[start..].iter().take_while(|e| name_of(e) == name).count() + start];
    let preferred = [
        scope.map(scope_page),
        Some("api/description/global-interfaces"),
        Some("api/description/conditions"),
        Some("api/description/helper-interfaces"),
        Some("api/description/project-target"),
    ];
    preferred
        .iter()
        .flatten()
        .find_map(|page| entries.iter().find(|e| page_of(e) == *page))
        .or_else(|| entries.first())
}

/// The prototype of `name` in the manual, `set_default(default: boolean)`,
/// with its documented parameters `(name, description)`.
pub fn doc_prototype(
    scope: Option<Scope>,
    name: &str,
) -> Option<(&'static str, Vec<(&'static str, &'static str)>)> {
    use crate::doc_links::{DOC_PARAMS, DOC_PROTOTYPES};
    let entry = pick(DOC_PROTOTYPES, scope, name, |e| e.0, |e| e.1)?;
    let page = entry.1;
    let start = DOC_PARAMS.partition_point(|e| (e.0, e.1) < (name, page));
    let params = DOC_PARAMS[start..]
        .iter()
        .take_while(|e| e.0 == name && e.1 == page)
        .map(|e| (e.2, e.3))
        .collect();
    Some((entry.2, params))
}

/// The parameters of a prototype: `["files: string|array", "...", "{defines, ...}"]`.
pub fn prototype_params(prototype: &str) -> Vec<&str> {
    let Some(open) = prototype.find('(') else {
        return Vec::new();
    };
    let inner = prototype[open + 1..].strip_suffix(')').unwrap_or(&prototype[open + 1..]);
    let mut out = Vec::new();
    let (mut depth, mut start) = (0i32, 0);
    for (i, c) in inner.char_indices() {
        match c {
            '(' | '{' | '[' => depth += 1,
            ')' | '}' | ']' => depth -= 1,
            ',' if depth == 0 => {
                out.push(inner[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }
    let last = inner[start..].trim();
    if !last.is_empty() {
        out.push(last);
    }
    out
}

/// The section documenting `owner.name` or `owner:name`: `os.cp`,
/// `target:targetfile`, `json.decode` of `core.base.json`, ...
pub fn member_link(owner: &str, separator: char, name: &str) -> Option<String> {
    let short = owner.rsplit('.').next().unwrap_or(owner);
    let key = format!("{short}{separator}{name}");
    doc_link(None, &key)
        // A module documented on one page: `core.base.semver`.
        .or_else(|| doc_link(None, owner))
}

/// The first paragraph of a page, for short displays.
pub fn summary(doc: &str) -> &str {
    doc.split("\n\n").next().unwrap_or(doc).trim()
}

/// A page for an API without a hand written one, from its kind.
pub fn generated_api_doc(scope: Scope, name: &str, kind: ApiKind) -> String {
    let (prefix, what) = name.split_once('_').unwrap_or(("", name));
    let owner = match scope {
        Scope::Root => "the project",
        Scope::Target => "the target",
        Scope::Option => "the option",
        Scope::Rule => "the rule",
        Scope::Package => "the package",
        Scope::Toolchain => "the toolchain",
        Scope::Task => "the task",
    };
    let mut out = String::with_capacity(320);
    let _ = match (kind, prefix) {
        (ApiKind::Script, _) => write!(out, "Runs a script for `{what}` of {owner}."),
        (_, "set") => write!(out, "Sets `{what}` of {owner}, replacing the earlier values."),
        (_, "add") => write!(out, "Adds to `{what}` of {owner}."),
        (_, "del" | "remove") => write!(out, "Removes values from `{what}` of {owner}."),
        _ => write!(out, "A {} API of {owner}.", kind.describe()),
    };

    out.push_str("\n\n# Examples\n\n```lua\n");
    let indent = if scope == Scope::Root { "" } else { "    " };
    if scope != Scope::Root {
        let _ = writeln!(out, "{}(\"name\")", scope.name());
    }
    let _ = match kind {
        ApiKind::Script => {
            let param = scope.script_instance().map_or("...", Instance::name);
            writeln!(out, "{indent}{name}(function ({param})\n{indent}    -- ...\n{indent}end)")
        }
        ApiKind::Values | ApiKind::Groups => writeln!(out, "{indent}{name}(\"value\")"),
        ApiKind::Paths => writeln!(out, "{indent}{name}(\"src/*.c\")"),
        ApiKind::KeyValues => writeln!(out, "{indent}{name}(\"name\", \"value\")"),
        ApiKind::Dictionary => writeln!(out, "{indent}{name}(\"name\", {{}})"),
        ApiKind::Custom => writeln!(out, "{indent}{name}(...)"),
    };
    out.push_str("```\n\n# Notes\n\n");
    out.push_str(match kind {
        ApiKind::Values | ApiKind::Groups | ApiKind::KeyValues => {
            "`set_*` replaces the values set before, `add_*` appends to them. A last table argument holds options, e.g. `{public = true}`."
        }
        ApiKind::Paths => "Paths are relative to the script; `*` and `**` globs are accepted.",
        ApiKind::Script => {
            "The callback runs at build time, in the script context: `import`, `os.*` and the methods of the instance are available."
        }
        ApiKind::Dictionary => "Each call sets one entry of a dictionary.",
        ApiKind::Custom => "A function with its own arguments: see the manual.",
    });
    out
}

/// A page for a method (`separator` `:`) or a module function (`.`) without
/// a hand written one.
pub fn generated_member_doc(owner: &str, separator: char, name: &str) -> String {
    let what = if separator == ':' { "Method of the" } else { "Function of the builtin module" };
    format!(
        "{what} `{owner}`.\n\n# Examples\n\n```lua\nlocal result = {owner}{separator}{name}(...)\n```"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookups() {
        let set_kind = api_doc(Some(Scope::Target), "set_kind").expect("set_kind");
        assert!(set_kind.contains("# Examples"));
        let option_default = api_doc(Some(Scope::Option), "set_default").expect("option page");
        assert!(option_default.contains("default value of the option"));
        let target_default = api_doc(Some(Scope::Target), "set_default").expect("target page");
        assert!(target_default.contains("without a target name"));
        assert!(api_doc(None, "import").is_some());
        // Lua comments in examples keep their dashes.
        assert!(api_doc(None, "target_end").is_some_and(|d| d.contains("-- Root again")));
        assert!(module_doc("os", "cp").is_some());
        assert!(instance_doc(Instance::Target, "targetfile").is_some());
        assert_eq!(summary("First line.\n\n# Examples"), "First line.");
    }

    #[test]
    fn prototypes() {
        let (set_default, params) =
            doc_prototype(Some(Scope::Target), "set_default").expect("set_default");
        assert_eq!(set_default, "set_default(default: boolean)");
        assert_eq!(params.first().map(|p| p.0), Some("default"));
        let (add_files, _) = doc_prototype(Some(Scope::Target), "add_files").expect("add_files");
        assert_eq!(
            prototype_params(add_files),
            [
                "files: string|array",
                "...",
                "{defines, languages, includedirs, rules, force, sourcekind, $flags}"
            ]
        );
        assert_eq!(
            doc_link(Some(Scope::Target), "set_default").as_deref(),
            Some("https://xmake.io/api/description/project-target.html#set-default")
        );
    }

    #[test]
    fn generated_pages_have_examples() {
        let doc = generated_api_doc(Scope::Target, "set_prefixname", ApiKind::Values);
        assert!(doc.starts_with("Sets `prefixname` of the target"), "{doc}");
        assert!(doc.contains("# Examples") && doc.contains("# Notes"));
        let script = generated_api_doc(Scope::Rule, "after_link", ApiKind::Script);
        assert!(script.contains("after_link(function (target)"), "{script}");
    }
}
