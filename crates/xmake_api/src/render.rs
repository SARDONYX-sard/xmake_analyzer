//! Writing the stubs (see [`crate::Stubs`]) for a version of xmake: the
//! stubs of another version, with the functions xmake lists added to the
//! files of their context and those it lacks removed, the documentation and
//! types written by hand kept.
//!
//! The same code writes the stubs of the repository (`xmake_analyzer stubs`)
//! and the stubs of the installed xmake at run time.

use std::{collections::BTreeMap, fmt::Write as _};

use serde::Deserialize;

use crate::{ApiDb, ApiKind, Instance, Scope, Stubs};

/// The values xmake lists, written whole.
pub const VALUES_FILE: &str = "values.lua";

/// The sections of the manual (`tools/gen_doc_links.py --json`): links,
/// prototypes and parameters of the APIs, per page.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Manual {
    /// `(name, page, anchor)`
    pub links: Vec<(String, String, String)>,
    /// `(name, page, prototype)`
    pub prototypes: Vec<(String, String, String)>,
    /// `(name, page, parameter, description)`
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

/// The entry of `name` in a sorted table, on the page of `scope` when there
/// is one, else on the global pages, else the first.
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

/// The class of the APIs of `kind` of `scope`: `xmake.target.paths`.
pub fn scope_class(scope: Scope, kind: ApiKind) -> String {
    format!("xmake.{}.{}", scope.name(), kind.as_str())
}

/// The file of the APIs of `scope`.
pub fn scope_file(scope: Scope) -> String {
    format!("description/scopes/{}.lua", scope.name())
}

/// The file of the importable module `module`: `modules/core/base/json.lua`.
pub fn module_file(module: &str) -> String {
    format!("modules/{}.lua", module.replace('.', "/"))
}

// === The model ===

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Param {
    pub name: String,
    pub optional: bool,
    pub ty: String,
    pub doc: String,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Func {
    pub owner: String,
    pub name: String,
    pub method: bool,
    pub doc: String,
    pub params: Vec<Param>,
    /// `(type, name)`
    pub returns: Vec<(String, String)>,
    pub see: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Class {
    pub name: String,
    pub doc: String,
    pub call: Option<String>,
    pub see: Option<String>,
    pub fields: Vec<Param>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Alias {
    pub name: String,
    pub ty: String,
    pub values: Vec<(String, String)>,
}

/// A stub file: its classes, each followed by its functions, then the
/// globals, then the aliases.
#[derive(Clone, Debug, Default)]
pub(crate) struct File {
    pub classes: Vec<Class>,
    pub functions: Vec<Func>,
    pub aliases: Vec<Alias>,
}

/// Every file, by path.
pub(crate) type Files = BTreeMap<String, File>;

impl File {
    fn class_mut(&mut self, name: &str) -> &mut Class {
        if let Some(i) = self.classes.iter().position(|c| c.name == name) {
            return &mut self.classes[i];
        }
        self.classes.push(Class { name: name.to_owned(), ..Class::default() });
        self.classes.last_mut().expect("pushed")
    }

    fn has(&self, owner: &str, name: &str) -> bool {
        self.functions.iter().any(|f| f.owner == owner && f.name == name)
    }

    fn text(&self, path: &str) -> String {
        let mut out = String::from("---@meta\n");
        let globals_table = path.starts_with("description/") || path.starts_with("script/");
        let mut vars: Vec<(String, String)> = Vec::new();
        for class in &self.classes {
            let var = var_name(&class.name, globals_table && !path.contains("/scopes/"), &vars);
            vars.push((class.name.clone(), var.clone()));
            out.push('\n');
            push_doc(&mut out, &class.doc);
            let _ = writeln!(out, "---@class {}", class.name);
            if let Some(call) = &class.call {
                let _ = writeln!(out, "---@overload {call}");
            }
            for field in &class.fields {
                push_typed(&mut out, "field", field);
            }
            if let Some(see) = &class.see {
                let _ = writeln!(out, "---@see {see}");
            }
            if var == class.name {
                let _ = writeln!(out, "{var} = {{}}");
            } else {
                let _ = writeln!(out, "local {var} = {{}}");
            }
            for f in self.functions.iter().filter(|f| f.owner == class.name) {
                push_function(&mut out, f, &var);
            }
        }
        for f in self.functions.iter().filter(|f| !self.classes.iter().any(|c| c.name == f.owner)) {
            let var = if f.owner == crate::GLOBALS { String::new() } else { f.owner.clone() };
            push_function(&mut out, f, &var);
        }
        for alias in &self.aliases {
            out.push('\n');
            if alias.values.is_empty() {
                let _ = writeln!(out, "---@alias {} {}", alias.name, alias.ty);
                continue;
            }
            let _ = writeln!(out, "---@alias {}", alias.name);
            for (value, doc) in &alias.values {
                let doc = one_line(doc);
                if doc.is_empty() {
                    let _ = writeln!(out, "---| {value:?}");
                } else {
                    let _ = writeln!(out, "---| {value:?} # {doc}");
                }
            }
        }
        out
    }
}

/// The table of `class` in a file: the global itself for the builtin
/// modules (`os = {}`), else a local named after it.
fn var_name(class: &str, global: bool, taken: &[(String, String)]) -> String {
    let ident =
        |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_');
    if global && ident(class) {
        return class.to_owned();
    }
    let base = match class.strip_prefix("xmake.") {
        Some(rest) => rest.replace('.', "_"),
        None => class
            .rsplit('.')
            .next()
            .unwrap_or(class)
            .replace(|c: char| !c.is_ascii_alphanumeric(), "_"),
    };
    let mut var = base.clone();
    let mut n = 2;
    while taken.iter().any(|(_, v)| *v == var)
        || matches!(var.as_str(), "end" | "and" | "or" | "not" | "function" | "local")
    {
        var = format!("{base}{n}");
        n += 1;
    }
    var
}

fn one_line(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn push_doc(out: &mut String, doc: &str) {
    if doc.is_empty() {
        return;
    }
    for line in doc.lines() {
        if line.is_empty() {
            out.push_str("---\n");
        } else {
            let _ = writeln!(out, "--- {line}");
        }
    }
}

fn push_typed(out: &mut String, tag: &str, p: &Param) {
    let optional = if p.optional { "?" } else { "" };
    let doc = one_line(&p.doc);
    let line = format!("---@{tag} {}{optional} {} {doc}", p.name, p.ty);
    let _ = writeln!(out, "{}", line.trim_end());
}

fn push_function(out: &mut String, f: &Func, var: &str) {
    out.push('\n');
    push_doc(out, &f.doc);
    for p in &f.params {
        push_typed(out, "param", p);
    }
    for (ty, name) in &f.returns {
        let _ = writeln!(out, "{}", format!("---@return {ty} {name}").trim_end());
    }
    if let Some(see) = &f.see {
        let _ = writeln!(out, "---@see {see}");
    }
    // `function f(files, ...) end`: nothing after `...` in Lua.
    let mut args: Vec<&str> = Vec::new();
    for p in &f.params {
        args.push(&p.name);
        if p.name == "..." {
            break;
        }
    }
    let args = args.join(", ");
    let name = &f.name;
    let _ = match (var.is_empty(), f.method) {
        (true, _) => writeln!(out, "function {name}({args}) end"),
        (false, true) => writeln!(out, "function {var}:{name}({args}) end"),
        (false, false) => writeln!(out, "function {var}.{name}({args}) end"),
    };
}

/// The model of parsed stubs.
pub(crate) fn model(stubs: &Stubs) -> Files {
    let mut files = Files::new();
    for class in stubs.classes() {
        let file = files.entry(class.location.file.to_owned()).or_default();
        file.classes.push(Class {
            name: class.name.to_owned(),
            doc: class.doc.clone(),
            call: class.call.map(str::to_owned),
            see: class.see.map(str::to_owned),
            fields: class
                .fields
                .iter()
                .map(|f| Param {
                    name: f.name.to_owned(),
                    optional: f.optional,
                    ty: f.ty.to_owned(),
                    doc: f.doc.to_owned(),
                })
                .collect(),
        });
    }
    for f in stubs.functions() {
        let file = files.entry(f.location.file.to_owned()).or_default();
        file.functions.push(func_of(f));
    }
    for (path, text) in stubs.files() {
        let file = files.entry((*path).to_owned()).or_default();
        let parsed = Stubs::parse_files(vec![(path, text)]);
        file.aliases = parsed
            .aliases()
            .iter()
            .map(|a| Alias {
                name: a.name.to_owned(),
                ty: a.ty.to_owned(),
                values: a.values.iter().map(|(v, d)| ((*v).to_owned(), (*d).to_owned())).collect(),
            })
            .collect();
    }
    files
}

fn func_of(f: &crate::Function) -> Func {
    Func {
        owner: f.owner.to_owned(),
        name: f.name.to_owned(),
        method: f.method,
        doc: f.doc.clone(),
        params: f
            .params
            .iter()
            .map(|p| Param {
                name: p.name.to_owned(),
                optional: p.optional,
                ty: p.ty.to_owned(),
                doc: p.doc.to_owned(),
            })
            .collect(),
        returns: f.returns.iter().map(|r| (r.ty.to_owned(), r.name.to_owned())).collect(),
        see: f.see.map(str::to_owned),
    }
}

/// `(file, owner, name, method)` of every function `db` lists.
fn wanted(db: &ApiDb) -> Vec<(String, String, String, bool)> {
    let mut out = Vec::new();
    for scope in std::iter::once(Scope::Root).chain(Scope::NAMED) {
        for api in db.scope_apis(scope) {
            out.push((
                scope_file(scope),
                scope_class(scope, api.kind),
                api.name.to_string(),
                false,
            ));
        }
    }
    for name in db.description_functions() {
        out.push((
            "description/globals.lua".into(),
            crate::GLOBALS.into(),
            name.to_string(),
            false,
        ));
    }
    for name in db.script_functions() {
        out.push(("script/globals.lua".into(), crate::GLOBALS.into(), name.to_string(), false));
    }
    for (dir, modules) in
        [("description", db.description_modules()), ("script", db.script_modules())]
    {
        for module in modules {
            for name in module.functions.iter() {
                let file = format!("{dir}/{}.lua", module.name);
                out.push((file, module.name.to_string(), name.to_string(), false));
            }
        }
    }
    for instance in Instance::ALL {
        for name in db.instance_methods(instance) {
            let file = format!("script/instances/{}.lua", instance.name());
            out.push((file, instance.name().into(), name.to_string(), true));
        }
    }
    for api in db.extension_apis() {
        if let Some((module, name)) = api.rsplit_once('.') {
            out.push((module_file(module), module.to_owned(), name.to_owned(), false));
        }
    }
    out
}

/// Whether a function of `owner` in `path` is one xmake lists (removed when
/// it lists it no more), not a type only.
fn is_listed(path: &str, owner: &str) -> bool {
    let stem = path.rsplit('/').next().unwrap_or(path).trim_end_matches(".lua");
    if path.starts_with("description/scopes/") {
        return owner.starts_with("xmake.") && owner != "xmake.api";
    }
    if let Some(module) = path.strip_prefix("modules/") {
        return owner == module.trim_end_matches(".lua").replace('/', ".");
    }
    if path.starts_with("description/") || path.starts_with("script/") {
        return owner == stem || (stem == "globals" && owner == crate::GLOBALS);
    }
    false
}

/// The stubs `previous` updated for `db`: `(path, text)`, sorted.
pub fn update(previous: &Stubs, db: &ApiDb, manual: Option<&Manual>) -> Vec<(String, String)> {
    let mut files = model(previous);
    update_model(&mut files, db);
    if let Some(manual) = manual {
        apply_manual(&mut files, manual);
    }
    render(&files)
}

/// The text of `files`, sorted by path.
pub(crate) fn render(files: &Files) -> Vec<(String, String)> {
    files.iter().map(|(path, file)| (path.clone(), file.text(path))).collect()
}

fn update_model(files: &mut Files, db: &ApiDb) {
    let wanted = wanted(db);
    // The documentation of each function, to give it to the new ones:
    // `os.getenv` of the description from that of the scripts.
    let mut known: BTreeMap<(String, String), Func> = BTreeMap::new();
    for file in files.values() {
        for f in &file.functions {
            let key = (f.owner.clone(), f.name.clone());
            let better = known.get(&key).is_none_or(|k| k.doc.is_empty() && !f.doc.is_empty());
            if better {
                known.insert(key, f.clone());
            }
        }
    }
    let wanted_set: std::collections::BTreeSet<(&str, &str, &str)> =
        wanted.iter().map(|(p, o, n, _)| (p.as_str(), o.as_str(), n.as_str())).collect();
    for (path, file) in files.iter_mut() {
        // Typed by hand: kept, xmake may have it without listing it.
        file.functions.retain(|f| {
            !is_listed(path, &f.owner)
                || wanted_set.contains(&(path.as_str(), &f.owner, &f.name))
                || !f.doc.is_empty()
                || !f.returns.is_empty()
        });
    }
    for (path, owner, name, method) in &wanted {
        let file = files.entry(path.clone()).or_default();
        if file.has(owner, name) {
            continue;
        }
        if owner != crate::GLOBALS {
            file.class_mut(owner);
        }
        // A scope API of another kind or scope, a function of the other
        // context: its documentation.
        let inherited = known.get(&(owner.clone(), name.clone())).cloned().or_else(|| {
            known
                .iter()
                .find(|((o, n), _)| {
                    n == name && o.starts_with("xmake.") && owner.starts_with("xmake.")
                })
                .map(|(_, f)| f.clone())
        });
        let mut f = inherited.unwrap_or_default();
        f.owner = owner.clone();
        f.name = name.clone();
        f.method = *method;
        file.functions.push(f);
    }
    // Classes left without functions in the files of the lists.
    for (path, file) in files.iter_mut() {
        let used: Vec<String> = file.functions.iter().map(|f| f.owner.clone()).collect();
        file.classes.retain(|c| {
            !is_listed(path, &c.name)
                || used.contains(&c.name)
                || c.call.is_some()
                || !c.doc.is_empty()
        });
    }
    files.retain(|_, f| !f.classes.is_empty() || !f.functions.is_empty() || !f.aliases.is_empty());
    files.insert(VALUES_FILE.to_owned(), values_file(db));
}

/// The prototypes and links of `manual` in the stubs.
fn apply_manual(files: &mut Files, manual: &Manual) {
    for (path, file) in files.iter_mut() {
        let scope = path
            .strip_prefix("description/scopes/")
            .map(|s| s.trim_end_matches(".lua"))
            .and_then(|s| std::iter::once(Scope::Root).chain(Scope::NAMED).find(|x| x.name() == s));
        for f in &mut file.functions {
            let key = match (scope, f.owner.as_str()) {
                (Some(_), _) | (None, crate::GLOBALS) => f.name.clone(),
                (None, owner) => {
                    let short = owner.rsplit('.').next().unwrap_or(owner);
                    format!("{short}{}{}", if f.method { ':' } else { '.' }, f.name)
                }
            };
            if let Some(link) = manual.link(scope, &key) {
                f.see = Some(link);
            }
            let manual_params =
                scope.is_some() || (f.owner == crate::GLOBALS && f.params.is_empty());
            if manual_params && let Some((prototype, docs)) = manual.prototype(scope, &key) {
                f.params = params_of_prototype(prototype, &docs);
            }
        }
        for class in &mut file.classes {
            if let Some(link) = manual.link(None, &class.name) {
                class.see = Some(link);
            }
        }
    }
}

/// The parameters of a prototype of the manual: `files: string|array, ...,
/// {defines, languages}` are `files: string|string[]`, `...`, and an options
/// table.
pub(crate) fn params_of_prototype(prototype: &str, docs: &[(&str, &str)]) -> Vec<Param> {
    let doc_of = |name: &str| {
        docs.iter().find(|(n, _)| *n == name).map(|(_, d)| one_line(d)).unwrap_or_default()
    };
    let mut out: Vec<Param> = Vec::new();
    for param in crate::prototype_params(prototype) {
        if param == "..." {
            let ty = out.last().map_or_else(|| "any".to_owned(), |p| p.ty.clone());
            out.push(Param { name: "...".into(), optional: false, ty, doc: doc_of("...") });
            continue;
        }
        if let Some(keys) = param.strip_prefix('{').and_then(|p| p.strip_suffix('}')) {
            let fields: Vec<String> = keys
                .split(',')
                .map(str::trim)
                .filter(|k| !k.is_empty())
                .map(|k| format!("{}?: any", k.trim_start_matches('$')))
                .collect();
            let ty = format!("{{{}}}", fields.join(", "));
            out.push(Param { name: "opts".into(), optional: true, ty, doc: String::new() });
            continue;
        }
        let (name, ty) = param.split_once(':').unwrap_or((param, "any"));
        let name = name.trim();
        let optional = name.ends_with('?');
        let name = name.trim_end_matches('?');
        out.push(Param {
            name: name.into(),
            optional,
            ty: luacats_type(ty.trim()),
            doc: doc_of(name),
        });
    }
    out
}

/// A type of the manual in LuaCATS: `string|array` is `string|string[]`,
/// `function (target)` is `fun(target: target)`.
fn luacats_type(manual: &str) -> String {
    let parts: Vec<&str> = manual.split('|').map(str::trim).collect();
    let base = parts
        .iter()
        .find(|p| **p != "array" && !p.starts_with("function"))
        .copied()
        .unwrap_or("any");
    let mut out: Vec<String> = Vec::new();
    for part in parts {
        let ty = if part == "array" {
            format!("{base}[]")
        } else if let Some(rest) = part.strip_prefix("function") {
            let names = rest.trim().trim_start_matches('(').trim_end_matches(')');
            let params: Vec<String> = names
                .split(',')
                .map(str::trim)
                .filter(|n| !n.is_empty())
                .map(|n| {
                    let ty = if Instance::parse(n).is_some() { n } else { "any" };
                    format!("{n}: {ty}")
                })
                .collect();
            format!("fun({})", params.join(", "))
        } else {
            part.to_owned()
        };
        if !out.contains(&ty) {
            out.push(ty);
        }
    }
    out.join("|")
}

/// The aliases of the values xmake lists.
fn values_file(db: &ApiDb) -> File {
    let v = &db.values;
    let plain = |name: &str, list: &[crate::Str]| Alias {
        name: name.to_owned(),
        ty: String::new(),
        values: list.iter().map(|x| (x.to_string(), String::new())).collect(),
    };
    let named = |name: &str, list: &[crate::Named]| Alias {
        name: name.to_owned(),
        ty: String::new(),
        values: list.iter().map(|x| (x.name.to_string(), x.description.to_string())).collect(),
    };
    File {
        aliases: vec![
            plain("xmake.value.platform", &v.platforms),
            plain("xmake.value.arch", &v.architectures),
            plain("xmake.value.mode", &v.modes),
            plain("xmake.value.rule", &v.rules),
            named("xmake.value.policy", &v.policies),
            named("xmake.value.toolchain", &v.toolchains),
        ],
        ..File::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Writing the shipped stubs for their own lists changes nothing: what
    /// is parsed is written back.
    #[test]
    fn round_trip() {
        let stubs = Stubs::builtin();
        let db = ApiDb::from_stubs(stubs);
        let files = update(stubs, &db, None);
        let shipped: Vec<(String, String)> =
            stubs.files().iter().map(|(p, t)| ((*p).to_owned(), t.replace("\r\n", "\n"))).collect();
        let changed: Vec<&str> = files
            .iter()
            .filter(|(p, t)| shipped.iter().find(|(q, _)| q == p).is_none_or(|(_, u)| u != t))
            .map(|(p, _)| p.as_str())
            .collect();
        assert!(changed.is_empty(), "{changed:?}");
        assert_eq!(files.len(), shipped.len());
    }
}
