//! Writing the stubs for a version of xmake: the files of an older version,
//! with the functions xmake lists added, tagged (`@xmake.kind`,
//! `@xmake.context`) or removed, and the documentation written by hand kept.
//!
//! The same code updates the stubs of the repository (`xmake_analyzer stubs`)
//! and writes the stubs of the installed xmake at run time.
//!
//! A stub file is a list of items: comment lines, then one line of code,
//! `local os = {}` (with `---@class os`) or `function os.cp(...) end`.

use std::collections::{BTreeMap, BTreeSet};

use serde::Deserialize;

use crate::{ApiDb, ApiKind, Instance, Scope, stubs::Contexts};

/// The tags written here: replaced on every update.
const TAGS: [&str; 2] = ["---@xmake.kind", "---@xmake.context"];
/// The tags of the manual: replaced when a [`Manual`] is given.
const MANUAL_TAGS: [&str; 3] = ["---@xmake.prototype", "---@xmake.param", "---@see"];

/// The file of the values: written whole.
const VALUES_FILE: &str = "values.lua";

/// The sections of the manual (`tools/gen_doc_links.py --json`): links,
/// prototypes and parameters of the APIs, per page.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Manual {
    /// `(name, page, anchor)`, sorted.
    pub links: Vec<(String, String, String)>,
    /// `(name, page, prototype)`, sorted.
    pub prototypes: Vec<(String, String, String)>,
    /// `(name, page, parameter, description)`, sorted.
    pub params: Vec<(String, String, String, String)>,
}

impl Manual {
    /// Parses the JSON of `tools/gen_doc_links.py --json`.
    pub fn from_json(json: &str) -> Result<Manual, serde_json::Error> {
        let mut manual: Manual = serde_json::from_str(json)?;
        manual.links.sort();
        manual.prototypes.sort();
        manual.params.sort();
        Ok(manual)
    }

    /// The URL of the section of `name`, on the page of `scope` when there
    /// is one.
    pub fn link(&self, scope: Option<Scope>, name: &str) -> Option<String> {
        let (_, page, anchor) = pick(&self.links, scope, name, |e| &e.0, |e| &e.1)?;
        Some(if anchor.is_empty() {
            format!("https://xmake.io/{page}.html")
        } else {
            format!("https://xmake.io/{page}.html#{anchor}")
        })
    }

    /// The prototype of `name` and its documented parameters.
    pub fn prototype(&self, scope: Option<Scope>, name: &str) -> Option<(&str, Vec<(&str, &str)>)> {
        let (_, page, prototype) = pick(&self.prototypes, scope, name, |e| &e.0, |e| &e.1)?;
        let params = self
            .params
            .iter()
            .filter(|e| e.0 == name && &e.1 == page)
            .map(|e| (e.2.as_str(), e.3.as_str()))
            .collect();
        Some((prototype, params))
    }
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

/// The entry of `name` in a table sorted by name, on the page of `scope`
/// when there is one, else on the global pages, else the first.
fn pick<'t, T>(
    table: &'t [T],
    scope: Option<Scope>,
    name: &str,
    name_of: impl Fn(&T) -> &str,
    page_of: impl Fn(&T) -> &str,
) -> Option<&'t T> {
    let start = table.partition_point(|e| name_of(e) < name);
    let count = table[start..].iter().take_while(|e| name_of(e) == name).count();
    let entries = &table[start..start + count];
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

/// The class of the APIs of `scope`: `xmake.target`.
pub fn scope_class(scope: Scope) -> String {
    format!("xmake.{}", scope.name())
}

/// What a stub item is.
#[derive(Clone, Debug, PartialEq, Eq)]
enum What {
    Class(String),
    Function { owner: String, name: String },
    Other,
}

#[derive(Clone, Debug)]
struct Item {
    /// The lines before the code: blank lines, comments, annotations.
    lines: Vec<String>,
    /// The line of code; `None` for the comments ending the file.
    code: Option<String>,
    what: What,
}

/// A stub file being updated.
#[derive(Clone, Debug)]
struct File {
    name: String,
    items: Vec<Item>,
    /// The local tables of the classes: `local json = {}` for `core.base.json`.
    vars: BTreeMap<String, String>,
}

impl File {
    fn parse(name: &str, text: &str) -> File {
        let mut file = File { name: name.to_owned(), items: Vec::new(), vars: BTreeMap::new() };
        let mut lines: Vec<String> = Vec::new();
        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("--") {
                lines.push(line.trim_end().to_owned());
                continue;
            }
            let what = file.what(&lines, trimmed);
            file.items.push(Item {
                lines: std::mem::take(&mut lines),
                code: Some(line.trim_end().to_owned()),
                what,
            });
        }
        if !lines.is_empty() {
            file.items.push(Item { lines, code: None, what: What::Other });
        }
        file
    }

    /// What the line `code` after the comments `lines` declares.
    fn what(&mut self, lines: &[String], code: &str) -> What {
        if let Some(rest) = code.strip_prefix("function ") {
            let name = rest.split('(').next().unwrap_or("").trim();
            return match name.rsplit_once(['.', ':']) {
                Some((table, function)) => {
                    let owner = self.vars.get(table).cloned().unwrap_or_else(|| table.to_owned());
                    What::Function { owner, name: function.to_owned() }
                }
                None => What::Function { owner: crate::GLOBALS.to_owned(), name: name.to_owned() },
            };
        }
        let class = lines.iter().rev().find_map(|l| {
            let class = l.trim().strip_prefix("---@class")?.trim();
            Some(class.split([' ', ':']).next()?.to_owned())
        });
        let var =
            code.strip_prefix("local ").unwrap_or(code).split('=').next().unwrap_or("").trim();
        match class {
            Some(class) if code.ends_with("{}") => {
                self.vars.insert(var.to_owned(), class.clone());
                What::Class(class)
            }
            _ => What::Other,
        }
    }

    fn text(&self) -> String {
        let mut out = String::new();
        for item in &self.items {
            for line in &item.lines {
                out.push_str(line);
                out.push('\n');
            }
            if let Some(code) = &item.code {
                out.push_str(code);
                out.push('\n');
            }
        }
        out
    }

    /// The local table of `class` in this file, declared when missing.
    fn var_of(&mut self, class: &str) -> String {
        if class == crate::GLOBALS {
            return String::new();
        }
        if let Some((var, _)) = self.vars.iter().find(|(_, c)| *c == class) {
            return var.clone();
        }
        let base = match class.strip_prefix("xmake.") {
            Some(scope) => format!("{scope}_scope"),
            None => class.rsplit('.').next().unwrap_or(class).replace(['-', '+'], "_"),
        };
        let mut var = base.clone();
        let mut n = 2;
        while self.vars.contains_key(&var) {
            var = format!("{base}{n}");
            n += 1;
        }
        self.vars.insert(var.clone(), class.to_owned());
        self.items.push(Item {
            lines: vec![String::new(), format!("---@class {class}")],
            code: Some(format!("local {var} = {{}}")),
            what: What::Class(class.to_owned()),
        });
        var
    }
}

/// Where a function is listed, from the database.
#[derive(Clone, Copy, Debug, Default)]
struct Wanted {
    kind: Option<ApiKind>,
    contexts: Contexts,
    method: bool,
}

/// The functions `db` lists, by `(owner, name)`.
fn wanted(db: &ApiDb) -> BTreeMap<(String, String), Wanted> {
    let mut out: BTreeMap<(String, String), Wanted> = BTreeMap::new();
    fn add<'m>(
        out: &'m mut BTreeMap<(String, String), Wanted>,
        owner: &str,
        name: &str,
    ) -> &'m mut Wanted {
        out.entry((owner.to_owned(), name.to_owned())).or_default()
    }
    for scope in std::iter::once(Scope::Root).chain(Scope::NAMED) {
        for api in db.scope_apis(scope) {
            add(&mut out, &scope_class(scope), &api.name).kind = Some(api.kind);
        }
    }
    for name in db.description_functions() {
        add(&mut out, crate::GLOBALS, name).contexts.description = true;
    }
    for name in db.script_functions() {
        add(&mut out, crate::GLOBALS, name).contexts.script = true;
    }
    for module in db.description_modules() {
        for name in module.functions.iter() {
            add(&mut out, &module.name, name).contexts.description = true;
        }
    }
    for module in db.script_modules() {
        for name in module.functions.iter() {
            add(&mut out, &module.name, name).contexts.script = true;
        }
    }
    for instance in Instance::ALL {
        for name in db.instance_methods(instance) {
            let wanted = add(&mut out, instance.name(), name);
            wanted.contexts.instance = true;
            wanted.method = true;
        }
    }
    for api in db.extension_apis() {
        if let Some((module, name)) = api.rsplit_once('.') {
            add(&mut out, module, name).contexts.import = true;
        }
    }
    out
}

/// The file a new item of `owner` goes to.
fn file_for(files: &[File], owner: &str, wanted: Wanted) -> String {
    let declared = files.iter().find(|f| {
        f.items.iter().any(|i| i.what == What::Class(owner.to_owned()))
            || f.items
                .iter()
                .any(|i| matches!(&i.what, What::Function { owner: o, .. } if o == owner))
    });
    if let Some(file) = declared {
        return file.name.clone();
    }
    if owner == crate::GLOBALS {
        return "globals.lua".to_owned();
    }
    if owner.starts_with("xmake.") {
        return "description.lua".to_owned();
    }
    let own = format!("{owner}.lua");
    if files.iter().any(|f| f.name == own) {
        return own;
    }
    if wanted.contexts.import { "extensions.lua" } else { "modules.lua" }.to_owned()
}

/// The manual tags of `(owner, name)`.
fn manual_tags(manual: &Manual, owner: &str, name: &str, method: bool) -> Vec<String> {
    let scope = owner
        .strip_prefix("xmake.")
        .and_then(|s| std::iter::once(Scope::Root).chain(Scope::NAMED).find(|x| x.name() == s));
    let mut out = Vec::new();
    let (key, scope) = match () {
        () if owner == crate::GLOBALS => (name.to_owned(), None),
        () if scope.is_some() => (name.to_owned(), scope),
        // `os.cp`, `target:targetfile`, `json.decode` of `core.base.json`.
        () => {
            let short = owner.rsplit('.').next().unwrap_or(owner);
            (format!("{short}{}{name}", if method { ':' } else { '.' }), None)
        }
    };
    if (owner == crate::GLOBALS || scope.is_some())
        && let Some((prototype, params)) = manual.prototype(scope, &key)
    {
        out.push(format!("---@xmake.prototype {prototype}"));
        for (param, description) in params {
            let description = description.split_whitespace().collect::<Vec<_>>().join(" ");
            out.push(format!("---@xmake.param {param} {description}").trim_end().to_owned());
        }
    }
    if let Some(link) = manual.link(scope, &key) {
        out.push(format!("---@see {link}"));
    }
    out
}

fn strip_tags(lines: &mut Vec<String>, tags: &[&str]) {
    lines.retain(|l| !tags.iter().any(|t| l.trim_start().starts_with(t)));
}

/// The stubs `previous` updated for `db`: `(file name, text)`, sorted. With
/// `manual`, its prototypes and links replace those of the stubs.
pub fn update(
    previous: &[(&str, &str)],
    db: &ApiDb,
    manual: Option<&Manual>,
) -> Vec<(String, String)> {
    let mut files: Vec<File> = previous
        .iter()
        .filter(|(name, _)| *name != VALUES_FILE)
        .map(|(name, text)| File::parse(name, text))
        .collect();
    let mut wanted = wanted(db);
    let mut seen: BTreeSet<(String, String)> = BTreeSet::new();

    // The functions already written: tagged again, or gone.
    for file in &mut files {
        file.items.retain_mut(|item| {
            let What::Function { owner, name } = &item.what else {
                if let (What::Class(class), Some(manual)) = (&item.what, manual) {
                    strip_tags(&mut item.lines, &["---@see"]);
                    if let Some(link) = manual.link(None, class) {
                        item.lines.push(format!("---@see {link}"));
                    }
                }
                return true;
            };
            let key = (owner.clone(), name.clone());
            let was_listed =
                item.lines.iter().any(|l| TAGS.iter().any(|t| l.trim().starts_with(t)));
            strip_tags(&mut item.lines, &TAGS);
            let Some(want) = wanted.get(&key).copied() else {
                // Listed by an older xmake only: not there anymore.
                return !was_listed;
            };
            seen.insert(key);
            if let Some(manual) = manual {
                strip_tags(&mut item.lines, &MANUAL_TAGS);
                item.lines.extend(manual_tags(manual, owner, name, want.method));
            }
            item.lines.extend(tags(want));
            true
        });
    }

    // The functions not written yet.
    wanted.retain(|key, _| !seen.contains(key));
    for ((owner, name), want) in wanted {
        let file_name = file_for(&files, &owner, want);
        let index = match files.iter().position(|f| f.name == file_name) {
            Some(index) => index,
            None => {
                files.push(File {
                    name: file_name.clone(),
                    items: vec![Item {
                        lines: vec!["---@meta".to_owned()],
                        code: None,
                        what: What::Other,
                    }],
                    vars: BTreeMap::new(),
                });
                files.len() - 1
            }
        };
        let file = &mut files[index];
        let var = file.var_of(&owner);
        let code = match (owner.as_str(), want.method) {
            (crate::GLOBALS, _) => format!("function {name}(...) end"),
            (_, true) => format!("function {var}:{name}(...) end"),
            (_, false) => format!("function {var}.{name}(...) end"),
        };
        let mut lines = vec![String::new()];
        if let Some(manual) = manual {
            lines.extend(manual_tags(manual, &owner, &name, want.method));
        }
        lines.extend(tags(want));
        // After the last item of the owner.
        let at = file
            .items
            .iter()
            .rposition(|i| match &i.what {
                What::Class(c) => *c == owner,
                What::Function { owner: o, .. } => *o == owner,
                What::Other => false,
            })
            .map_or(file.items.len(), |i| i + 1);
        file.items
            .insert(at, Item { lines, code: Some(code), what: What::Function { owner, name } });
    }

    let mut out: Vec<(String, String)> = files.iter().map(|f| (f.name.clone(), f.text())).collect();
    out.push((VALUES_FILE.to_owned(), values_file(db)));
    out.sort();
    out
}

fn tags(want: Wanted) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(kind) = want.kind {
        out.push(format!("---@xmake.kind {}", kind.as_str()));
    }
    if !want.contexts.is_empty() {
        out.push(format!("---@xmake.context {}", want.contexts.to_tag()));
    }
    out
}

/// The aliases of the values xmake knows.
fn values_file(db: &ApiDb) -> String {
    use std::fmt::Write as _;
    let mut out = String::from(
        "---@meta\n\n--- The values xmake knows, from `xmake show -l ...`: generated, do not edit.\n",
    );
    if let Some(version) = &db.version {
        let _ = writeln!(out, "---@xmake.version {version}");
    }
    let values = &db.values;
    let plain: [(&str, &[crate::Str]); 4] = [
        ("xmake.platform", &values.platforms),
        ("xmake.arch", &values.architectures),
        ("xmake.mode", &values.modes),
        ("xmake.rule", &values.rules),
    ];
    for (alias, list) in plain {
        let _ = writeln!(out, "\n---@alias {alias}");
        for value in list.iter() {
            let _ = writeln!(out, "---| {value:?}");
        }
    }
    let described: [(&str, &[crate::Named]); 2] =
        [("xmake.policy", &values.policies), ("xmake.toolchain", &values.toolchains)];
    for (alias, list) in described {
        let _ = writeln!(out, "\n---@alias {alias}");
        for named in list.iter() {
            let description = named.description.split_whitespace().collect::<Vec<_>>().join(" ");
            let _ = writeln!(out, "---| {:?} # {description}", named.name);
        }
    }
    out
}
