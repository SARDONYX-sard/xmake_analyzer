//! The xmake API database: which functions exist where.
//!
//! xmake scripts run in two contexts with different globals:
//!
//! - the **description** context: `xmake.lua` itself, where `target("foo")`
//!   opens a [`Scope`] and `set_kind(...)` configures it;
//! - the **script** context: the bodies of `on_build(function (target) ...)`
//!   and of imported modules, with the builtin modules (`os`, `path`, ...),
//!   the methods of instances (`target:name()`) and `import(...)`.
//!
//! Everything is written in the stubs (`stubs/<xmake version>/*.lua`, see
//! [`Stubs`]): the functions with their types and documentation, tagged with
//! where xmake has them, from which [`ApiDb::from_stubs`] builds the lists.
//! [`ApiDb::builtin`] reads the stubs embedded in the binary. The language
//! server asks the installed xmake for its own lists
//! ([`ApiDb::apply_show_apis`], [`ApiDb::apply_list`]) and writes stubs for it
//! ([`render::update`]), so that completion follows the xmake actually used.

mod docs;
mod overlay;
pub mod render;
mod signature;
mod stubs;
mod values;
mod version;

use std::borrow::Cow;

pub use crate::{
    docs::{
        api_doc, doc_link, doc_prototype, generated_api_doc, generated_member_doc, instance_doc,
        member_link, module_doc, prototype_params, summary,
    },
    overlay::{ListKind, OverlayError},
    signature::{api_params, render_signature},
    stubs::{
        Alias, BUILTIN_VERSION, Class, Contexts, Function, GLOBALS, Location, Param, Return, Stubs,
        class_of,
    },
    values::{ArgValues, BUILTIN_CONFIGS, TOOLS, arg_values, option_keys, scoped_arg_values},
    version::Version,
};

/// A string that is static for the builtin data and owned for data read from
/// the installed xmake.
pub type Str = Cow<'static, str>;

/// A description scope, opened by `target(...)`, `option(...)`, ...
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Scope {
    /// Outside of any scope: the project itself.
    Root,
    Target,
    Option,
    Rule,
    Package,
    Toolchain,
    Task,
}

impl Scope {
    /// Every scope but [`Scope::Root`].
    pub const NAMED: [Scope; 6] =
        [Scope::Target, Scope::Option, Scope::Rule, Scope::Package, Scope::Toolchain, Scope::Task];

    /// The name of the function opening the scope, `"root"` for the root.
    pub fn name(self) -> &'static str {
        match self {
            Scope::Root => "root",
            Scope::Target => "target",
            Scope::Option => "option",
            Scope::Rule => "rule",
            Scope::Package => "package",
            Scope::Toolchain => "toolchain",
            Scope::Task => "task",
        }
    }

    /// The scope opened by calling `name`, e.g. `target` or `target_end`.
    pub fn from_opener(name: &str) -> Option<(Scope, ScopeEdge)> {
        let (base, edge) = match name.strip_suffix("_end") {
            Some(base) => (base, ScopeEdge::End),
            None => (name, ScopeEdge::Begin),
        };
        let scope = Scope::NAMED.into_iter().find(|s| s.name() == base)?;
        Some((scope, edge))
    }

    /// The scopes whose APIs can be called in this scope, by priority: its
    /// own, then the root ones. The root scope also takes the target APIs,
    /// which then apply to every target.
    pub fn visible_scopes(self) -> &'static [Scope] {
        match self {
            Scope::Root => &[Scope::Root, Scope::Target],
            Scope::Target => &[Scope::Target, Scope::Root],
            Scope::Option => &[Scope::Option, Scope::Root],
            Scope::Rule => &[Scope::Rule, Scope::Root],
            Scope::Package => &[Scope::Package, Scope::Root],
            Scope::Toolchain => &[Scope::Toolchain, Scope::Root],
            Scope::Task => &[Scope::Task, Scope::Root],
        }
    }

    /// The instance passed to the script callbacks of this scope:
    /// `on_build(function (target) ...)` in a target or a rule.
    pub fn script_instance(self) -> Option<Instance> {
        match self {
            Scope::Target | Scope::Rule => Some(Instance::Target),
            Scope::Option => Some(Instance::Option),
            Scope::Package => Some(Instance::Package),
            Scope::Toolchain => Some(Instance::Toolchain),
            Scope::Root | Scope::Task => None,
        }
    }
}

/// Whether a scope function opens or closes its scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScopeEdge {
    /// `target("name")`
    Begin,
    /// `target_end()`
    End,
}

/// How xmake registers an API, which tells what its arguments are.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ApiKind {
    /// `set_kind("binary")`: plain values.
    Values,
    /// `add_files("src/*.c")`: paths relative to the script, globs allowed.
    Paths,
    /// `set_configvar("NAME", value)`: a key and its values.
    KeyValues,
    /// `add_csnippets("name", "code")`: a dictionary entry.
    Dictionary,
    /// `add_linkgroups("a", "b", {...})`: a group of values.
    Groups,
    /// `on_build(function (target) ... end)`: a script callback.
    Script,
    /// A function with its own semantics, e.g. `is_plat`.
    Custom,
}

impl ApiKind {
    /// Parses the key of `X.apis()` tables.
    pub fn parse(s: &str) -> Option<ApiKind> {
        let kind = match s {
            "values" => ApiKind::Values,
            "paths" => ApiKind::Paths,
            "keyvalues" => ApiKind::KeyValues,
            "dictionary" => ApiKind::Dictionary,
            "groups" => ApiKind::Groups,
            "script" => ApiKind::Script,
            "custom" => ApiKind::Custom,
            _ => return None,
        };
        Some(kind)
    }

    /// The key parsed by [`ApiKind::parse`].
    pub fn as_str(self) -> &'static str {
        match self {
            ApiKind::Values => "values",
            ApiKind::Paths => "paths",
            ApiKind::KeyValues => "keyvalues",
            ApiKind::Dictionary => "dictionary",
            ApiKind::Groups => "groups",
            ApiKind::Script => "script",
            ApiKind::Custom => "custom",
        }
    }

    /// The kind of an API known by name only: callbacks by their prefix,
    /// values otherwise.
    pub fn infer(name: &str) -> ApiKind {
        let is_script = ["on_", "before_", "after_"].iter().any(|p| name.starts_with(p));
        if is_script { ApiKind::Script } else { ApiKind::Values }
    }

    /// Short description shown in hovers and completion details.
    pub fn describe(self) -> &'static str {
        match self {
            ApiKind::Values => "values",
            ApiKind::Paths => "paths",
            ApiKind::KeyValues => "key and values",
            ApiKind::Dictionary => "dictionary",
            ApiKind::Groups => "groups",
            ApiKind::Script => "script callback",
            ApiKind::Custom => "function",
        }
    }
}

/// An API of a description scope.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScopeApi {
    pub scope: Scope,
    pub name: Str,
    pub kind: ApiKind,
}

/// An object handed to scripts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Instance {
    Target,
    Option,
    Package,
    Rule,
    Toolchain,
}

impl Instance {
    /// Every instance.
    pub const ALL: [Instance; 5] = [
        Instance::Target,
        Instance::Option,
        Instance::Package,
        Instance::Rule,
        Instance::Toolchain,
    ];

    /// The name used in `xmake show -l apis`, e.g. `target` in `target:name`.
    pub fn name(self) -> &'static str {
        match self {
            Instance::Target => "target",
            Instance::Option => "option",
            Instance::Package => "package",
            Instance::Rule => "rule",
            Instance::Toolchain => "toolchain",
        }
    }

    /// Parses [`Instance::name`].
    pub fn parse(s: &str) -> Option<Instance> {
        Instance::ALL.into_iter().find(|i| i.name() == s)
    }
}

/// The methods of an [`Instance`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstanceMethods {
    pub instance: Instance,
    pub methods: Cow<'static, [Str]>,
}

/// A builtin module, e.g. `os` with `os.files`, `os.cp`, ...
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Module {
    pub name: Str,
    pub functions: Cow<'static, [Str]>,
}

/// A value with a description, e.g. a policy or a toolchain.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Named {
    pub name: Str,
    pub description: Str,
}

/// Values accepted by some APIs, read from `xmake show -l ...`.
#[derive(Clone, Debug)]
pub struct KnownValues {
    pub platforms: Cow<'static, [Str]>,
    pub architectures: Cow<'static, [Str]>,
    pub modes: Cow<'static, [Str]>,
    pub rules: Cow<'static, [Str]>,
    pub policies: Cow<'static, [Named]>,
    pub toolchains: Cow<'static, [Named]>,
}

/// The API database, see the crate documentation.
#[derive(Clone, Debug)]
pub struct ApiDb {
    /// The xmake version the data comes from.
    pub version: Option<Version>,
    /// Sorted by scope, then name.
    scope_apis: Cow<'static, [ScopeApi]>,
    /// Functions of the description context: `is_plat`, `includes`, ...
    description_functions: Cow<'static, [Str]>,
    /// Modules of the description context: `os`, `path`, ...
    description_modules: Cow<'static, [Module]>,
    /// Functions of the script context: `import`, `raise`, `try`, ...
    script_functions: Cow<'static, [Str]>,
    /// Modules of the script context.
    script_modules: Cow<'static, [Module]>,
    instance_methods: Cow<'static, [InstanceMethods]>,
    /// Importable functions, fully qualified: `core.base.json.decode`.
    extension_apis: Cow<'static, [Str]>,
    pub values: KnownValues,
}

impl Default for ApiDb {
    fn default() -> ApiDb {
        ApiDb::builtin()
    }
}

impl ApiDb {
    /// The database of the stubs embedded in the binary.
    pub fn builtin() -> ApiDb {
        static BUILTIN: std::sync::OnceLock<ApiDb> = std::sync::OnceLock::new();
        BUILTIN.get_or_init(|| ApiDb::from_stubs(Stubs::builtin())).clone()
    }

    /// The database of the tagged functions of `stubs`, see [`Stubs`].
    pub fn from_stubs(stubs: &'static Stubs) -> ApiDb {
        let scopes: Vec<(String, Scope)> = std::iter::once(Scope::Root)
            .chain(Scope::NAMED)
            .map(|s| (render::scope_class(s), s))
            .collect();
        let mut scope_apis = Vec::new();
        let (mut description_functions, mut script_functions) = (Vec::new(), Vec::new());
        let mut description_modules: std::collections::BTreeMap<&str, Vec<Str>> =
            std::collections::BTreeMap::new();
        let mut script_modules = description_modules.clone();
        let mut instance_methods: std::collections::BTreeMap<&str, Vec<Str>> =
            std::collections::BTreeMap::new();
        let mut extension_apis = Vec::new();
        for f in stubs.functions() {
            if let Some(kind) = f.kind
                && let Some((_, scope)) = scopes.iter().find(|(c, _)| c == f.owner)
            {
                scope_apis.push(ScopeApi { scope: *scope, name: Cow::Borrowed(f.name), kind });
            }
            let c = f.contexts;
            if f.owner == GLOBALS {
                if c.description {
                    description_functions.push(Cow::Borrowed(f.name));
                }
                if c.script {
                    script_functions.push(Cow::Borrowed(f.name));
                }
                continue;
            }
            if c.description {
                description_modules.entry(f.owner).or_default().push(Cow::Borrowed(f.name));
            }
            if c.script {
                script_modules.entry(f.owner).or_default().push(Cow::Borrowed(f.name));
            }
            if c.instance {
                instance_methods.entry(f.owner).or_default().push(Cow::Borrowed(f.name));
            }
            if c.import {
                extension_apis.push(Cow::Owned(format!("{}.{}", f.owner, f.name)));
            }
        }
        scope_apis.sort_by(|a, b| (a.scope, &a.name).cmp(&(b.scope, &b.name)));
        scope_apis.dedup_by(|a, b| a.scope == b.scope && a.name == b.name);
        let modules = |map: std::collections::BTreeMap<&'static str, Vec<Str>>| -> Vec<Module> {
            map.into_iter()
                .map(|(name, mut functions)| {
                    functions.sort();
                    functions.dedup();
                    Module { name: Cow::Borrowed(name), functions: Cow::Owned(functions) }
                })
                .collect()
        };
        let sorted = |mut list: Vec<Str>| -> Cow<'static, [Str]> {
            list.sort();
            list.dedup();
            Cow::Owned(list)
        };
        let alias = |name: &str| -> Vec<Str> {
            stubs
                .alias(name)
                .map(|a| a.values.iter().map(|(v, _)| Cow::Borrowed(*v)).collect())
                .unwrap_or_default()
        };
        let named = |name: &str| -> Cow<'static, [Named]> {
            let values = stubs.alias(name).map(|a| {
                a.values
                    .iter()
                    .map(|(v, d)| Named { name: Cow::Borrowed(*v), description: Cow::Borrowed(*d) })
                    .collect()
            });
            Cow::Owned(values.unwrap_or_default())
        };
        ApiDb {
            version: stubs.version.and_then(Version::parse),
            scope_apis: Cow::Owned(scope_apis),
            description_functions: sorted(description_functions),
            description_modules: Cow::Owned(modules(description_modules)),
            script_functions: sorted(script_functions),
            script_modules: Cow::Owned(modules(script_modules)),
            instance_methods: Cow::Owned(
                Instance::ALL
                    .into_iter()
                    .map(|instance| InstanceMethods {
                        instance,
                        methods: sorted(
                            instance_methods.get(instance.name()).cloned().unwrap_or_default(),
                        ),
                    })
                    .collect(),
            ),
            extension_apis: sorted(extension_apis),
            values: KnownValues {
                platforms: Cow::Owned(alias("xmake.platform")),
                architectures: Cow::Owned(alias("xmake.arch")),
                modes: Cow::Owned(alias("xmake.mode")),
                rules: Cow::Owned(alias("xmake.rule")),
                policies: named("xmake.policy"),
                toolchains: named("xmake.toolchain"),
            },
        }
    }

    /// The APIs of `scope`, sorted by name.
    pub fn scope_apis(&self, scope: Scope) -> &[ScopeApi] {
        let all = &*self.scope_apis;
        let start = all.partition_point(|a| a.scope < scope);
        let end = all.partition_point(|a| a.scope <= scope);
        &all[start..end]
    }

    /// The API `name` of `scope`.
    pub fn scope_api(&self, scope: Scope, name: &str) -> Option<&ScopeApi> {
        let apis = self.scope_apis(scope);
        apis.binary_search_by(|a| (*a.name).cmp(name)).ok().map(|i| &apis[i])
    }

    /// The API `name` as callable in `scope`, see [`Scope::visible_scopes`].
    pub fn visible_api(&self, scope: Scope, name: &str) -> Option<&ScopeApi> {
        scope.visible_scopes().iter().find_map(|&s| self.scope_api(s, name))
    }

    /// Every scope defining an API named `name`.
    pub fn scopes_of<'a>(&'a self, name: &'a str) -> impl Iterator<Item = Scope> + 'a {
        let all = [Scope::Root].into_iter().chain(Scope::NAMED);
        all.filter(move |&s| self.scope_api(s, name).is_some())
    }

    /// The functions available anywhere in the description context.
    pub fn description_functions(&self) -> &[Str] {
        &self.description_functions
    }

    /// The modules of the description context.
    pub fn description_modules(&self) -> &[Module] {
        &self.description_modules
    }

    /// The global functions of the script context.
    pub fn script_functions(&self) -> &[Str] {
        &self.script_functions
    }

    /// The modules of the script context.
    pub fn script_modules(&self) -> &[Module] {
        &self.script_modules
    }

    /// The methods of `instance`.
    pub fn instance_methods(&self, instance: Instance) -> &[Str] {
        let found = self.instance_methods.iter().find(|m| m.instance == instance);
        found.map_or(&[], |m| &m.methods)
    }

    /// Importable functions, fully qualified: `core.base.json.decode`.
    pub fn extension_apis(&self) -> &[Str] {
        &self.extension_apis
    }

    /// Whether `name` is a function of the description context.
    pub fn is_description_function(&self, name: &str) -> bool {
        self.description_functions.iter().any(|f| f == name)
    }

    /// The module `name` of the description (`script == false`) or script
    /// context.
    pub fn module(&self, name: &str, script: bool) -> Option<&Module> {
        let modules = if script { &self.script_modules } else { &self.description_modules };
        modules.iter().find(|m| m.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_lookups() {
        let db = ApiDb::builtin();
        let set_kind = db.scope_api(Scope::Target, "set_kind").expect("set_kind");
        assert_eq!(set_kind.kind, ApiKind::Values);
        assert_eq!(db.scope_api(Scope::Target, "add_files").map(|a| a.kind), Some(ApiKind::Paths));
        assert_eq!(db.scope_api(Scope::Target, "on_build").map(|a| a.kind), Some(ApiKind::Script));
        assert!(db.scope_api(Scope::Option, "set_kind").is_none());
        assert!(db.scopes_of("add_cflags").any(|s| s == Scope::Option));
        assert!(db.instance_methods(Instance::Target).iter().any(|m| m == "targetfile"));
        assert!(db.module("os", true).is_some_and(|m| m.functions.iter().any(|f| f == "cp")));
        assert!(db.scope_api(Scope::Task, "on_run").is_some());
        assert!(db.visible_api(Scope::Root, "add_rules").is_some());
        assert!(db.visible_api(Scope::Option, "add_requires").is_some());
        assert!(db.visible_api(Scope::Option, "set_kind").is_none());
    }

    #[test]
    fn scope_apis_are_sorted() {
        let db = ApiDb::builtin();
        let sorted =
            db.scope_apis.windows(2).all(|w| (w[0].scope, &w[0].name) < (w[1].scope, &w[1].name));
        assert!(sorted, "generated SCOPE_APIS must be sorted and unique");
    }

    #[test]
    fn scope_openers() {
        assert_eq!(Scope::from_opener("target"), Some((Scope::Target, ScopeEdge::Begin)));
        assert_eq!(Scope::from_opener("rule_end"), Some((Scope::Rule, ScopeEdge::End)));
        assert_eq!(Scope::from_opener("set_kind"), None);
    }
}
