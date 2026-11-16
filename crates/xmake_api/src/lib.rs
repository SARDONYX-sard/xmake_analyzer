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
//! [`ApiDb::builtin`] is generated from a recent xmake (see
//! `tools/gen_builtin.py`) and costs no allocation: every table is a borrowed
//! `Cow`. The language server then asks the installed xmake for its own lists
//! ([`ApiDb::apply_show_apis`], [`ApiDb::apply_list`]), which replaces the
//! affected tables: only those become owned. This keeps completion in sync
//! with the xmake version actually used, older or newer.

mod doc_links;
mod docs;
mod generated;
mod overlay;
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
    stubs::{Class, Function, GLOBALS, Param, Return, Stubs, class_of},
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
    /// The database generated from the xmake listed in [`generated::VERSION`].
    pub fn builtin() -> ApiDb {
        use generated as g;
        ApiDb {
            version: Version::parse(g::VERSION),
            scope_apis: Cow::Borrowed(g::SCOPE_APIS),
            description_functions: Cow::Borrowed(g::DESCRIPTION_FUNCTIONS),
            description_modules: Cow::Borrowed(g::DESCRIPTION_MODULES),
            script_functions: Cow::Borrowed(g::SCRIPT_FUNCTIONS),
            script_modules: Cow::Borrowed(g::SCRIPT_MODULES),
            instance_methods: Cow::Borrowed(g::INSTANCE_METHODS),
            extension_apis: Cow::Borrowed(g::EXTENSION_APIS),
            values: KnownValues {
                platforms: Cow::Borrowed(g::PLATFORMS),
                architectures: Cow::Borrowed(g::ARCHITECTURES),
                modes: Cow::Borrowed(g::MODES),
                rules: Cow::Borrowed(g::RULES),
                policies: Cow::Borrowed(g::POLICIES),
                toolchains: Cow::Borrowed(g::TOOLCHAINS),
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
