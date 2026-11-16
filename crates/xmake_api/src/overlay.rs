//! Replacing the builtin tables with the lists of the installed xmake.
//!
//! The JSON documents are deserialized without copying the names (they borrow
//! the input), and a name already in the builtin tables reuses the static
//! string: only names the builtin data does not know are allocated.

use std::{borrow::Cow, collections::BTreeMap, fmt};

use serde::Deserialize;

use crate::{ApiDb, ApiKind, Instance, InstanceMethods, Module, Named, Scope, ScopeApi, Str};

/// A list printed by `xmake show -l <name> --json`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListKind {
    Platforms,
    Architectures,
    BuildModes,
    Rules,
    Policies,
    Toolchains,
}

impl ListKind {
    /// Every list.
    pub const ALL: [ListKind; 6] = [
        ListKind::Platforms,
        ListKind::Architectures,
        ListKind::BuildModes,
        ListKind::Rules,
        ListKind::Policies,
        ListKind::Toolchains,
    ];

    /// The argument of `xmake show -l`.
    pub fn show_name(self) -> &'static str {
        match self {
            ListKind::Platforms => "platforms",
            ListKind::Architectures => "architectures",
            ListKind::BuildModes => "buildmodes",
            ListKind::Rules => "rules",
            ListKind::Policies => "policies",
            ListKind::Toolchains => "toolchains",
        }
    }
}

/// Invalid JSON from xmake.
#[derive(Debug)]
pub struct OverlayError(serde_json::Error);

impl fmt::Display for OverlayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid xmake output: {}", self.0)
    }
}

impl std::error::Error for OverlayError {}

/// The output of `xmake show -l apis --json`. Missing keys keep the builtin
/// table.
#[derive(Deserialize)]
struct ShowApis<'a> {
    #[serde(borrow, default)]
    description_scope_apis: Option<Vec<Cow<'a, str>>>,
    #[serde(borrow, default)]
    description_builtin_apis: Option<Vec<Cow<'a, str>>>,
    #[serde(borrow, default)]
    description_builtin_module_apis: Option<Vec<Cow<'a, str>>>,
    #[serde(borrow, default)]
    script_builtin_apis: Option<Vec<Cow<'a, str>>>,
    #[serde(borrow, default)]
    script_builtin_module_apis: Option<Vec<Cow<'a, str>>>,
    #[serde(borrow, default)]
    script_instance_apis: Option<Vec<Cow<'a, str>>>,
    #[serde(borrow, default)]
    script_extension_module_apis: Option<Vec<Cow<'a, str>>>,
}

#[derive(Deserialize)]
struct NamedJson<'a> {
    #[serde(borrow)]
    name: Cow<'a, str>,
    #[serde(borrow, default)]
    description: Option<Cow<'a, str>>,
}

/// Returns the static string equal to `name` from `known` when there is one,
/// else an owned copy.
fn reuse(known: &[Str], name: &str) -> Str {
    match known.iter().find(|k| *k == name) {
        Some(k) => k.clone(),
        None => Cow::Owned(name.to_owned()),
    }
}

/// [`reuse`] over a sorted list.
fn reuse_sorted(known: &[Str], name: &str) -> Str {
    match known.binary_search_by(|k| (**k).cmp(name)) {
        Ok(i) => known[i].clone(),
        Err(_) => Cow::Owned(name.to_owned()),
    }
}

/// Groups `os.cp` style names into modules.
fn group_modules(names: &[Cow<'_, str>], known: &[Module]) -> Vec<Module> {
    let mut grouped: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for full in names {
        if let Some((module, func)) = full.rsplit_once('.') {
            grouped.entry(module).or_default().push(func);
        }
    }
    grouped
        .into_iter()
        .map(|(module, mut funcs)| {
            funcs.sort_unstable();
            funcs.dedup();
            let old = known.iter().find(|m| m.name == module);
            let old_funcs: &[Str] = old.map_or(&[], |m| &m.functions);
            let functions = funcs.iter().map(|f| reuse(old_funcs, f)).collect::<Vec<_>>();
            let name = old.map_or_else(|| Cow::Owned(module.to_owned()), |m| m.name.clone());
            Module { name, functions: Cow::Owned(functions) }
        })
        .collect()
}

impl ApiDb {
    /// Replaces the API tables with the output of
    /// `xmake show -l apis --json`.
    pub fn apply_show_apis(&mut self, json: &str) -> Result<(), OverlayError> {
        let show: ShowApis<'_> = serde_json::from_str(json).map_err(OverlayError)?;

        if let Some(names) = &show.description_scope_apis {
            self.apply_scope_apis(names, show.description_builtin_apis.as_deref());
        }
        if let Some(names) = &show.description_builtin_apis {
            let root: Vec<&str> = self.scope_apis(Scope::Root).iter().map(|a| &*a.name).collect();
            let functions = names
                .iter()
                .filter(|n| !root.contains(&&***n))
                .map(|n| reuse(&self.description_functions, n))
                .collect::<Vec<_>>();
            self.description_functions = Cow::Owned(functions);
        }
        if let Some(names) = &show.description_builtin_module_apis {
            self.description_modules = Cow::Owned(group_modules(names, &self.description_modules));
        }
        if let Some(names) = &show.script_builtin_apis {
            let functions = names.iter().map(|n| reuse(&self.script_functions, n)).collect();
            self.script_functions = Cow::Owned(functions);
        }
        if let Some(names) = &show.script_builtin_module_apis {
            self.script_modules = Cow::Owned(group_modules(names, &self.script_modules));
        }
        if let Some(names) = &show.script_instance_apis {
            self.apply_instance_methods(names);
        }
        if let Some(names) = &show.script_extension_module_apis {
            let mut apis =
                names.iter().map(|n| reuse_sorted(&self.extension_apis, n)).collect::<Vec<_>>();
            apis.sort_unstable();
            apis.dedup();
            self.extension_apis = Cow::Owned(apis);
        }
        Ok(())
    }

    /// `names` are `scope.api`; the root APIs are those of `builtin_names`
    /// already known as root APIs (xmake lists them with the builtins).
    fn apply_scope_apis(&mut self, names: &[Cow<'_, str>], builtin_names: Option<&[Cow<'_, str>]>) {
        let mut apis: Vec<ScopeApi> = Vec::with_capacity(names.len() + 32);
        for full in names {
            let Some((scope_name, name)) = full.split_once('.') else {
                continue;
            };
            let Some(scope) = Scope::NAMED.into_iter().find(|s| s.name() == scope_name) else {
                continue;
            };
            let api = match self.scope_api(scope, name) {
                Some(known) => known.clone(),
                None => ScopeApi {
                    scope,
                    name: Cow::Owned(name.to_owned()),
                    kind: ApiKind::infer(name),
                },
            };
            apis.push(api);
        }
        // Root APIs, and task APIs which `xmake show` does not list.
        let still_root = |a: &&ScopeApi| match builtin_names {
            Some(builtins) => builtins.contains(&a.name),
            None => true,
        };
        apis.extend(self.scope_apis(Scope::Root).iter().filter(still_root).cloned());
        apis.extend(self.scope_apis(Scope::Task).iter().cloned());
        apis.sort_unstable_by(|a, b| (a.scope, &a.name).cmp(&(b.scope, &b.name)));
        apis.dedup_by(|a, b| a.scope == b.scope && a.name == b.name);
        self.scope_apis = Cow::Owned(apis);
    }

    fn apply_instance_methods(&mut self, names: &[Cow<'_, str>]) {
        let mut tables = Vec::with_capacity(Instance::ALL.len());
        for instance in Instance::ALL {
            let known = self.instance_methods(instance);
            let prefix_len = instance.name().len() + 1;
            let mut methods = names
                .iter()
                .filter(|n| n.len() > prefix_len && n.starts_with(instance.name()))
                .filter(|n| n.as_bytes()[prefix_len - 1] == b':')
                .map(|n| reuse_sorted(known, &n[prefix_len..]))
                .collect::<Vec<_>>();
            methods.sort_unstable();
            methods.dedup();
            tables.push(InstanceMethods { instance, methods: Cow::Owned(methods) });
        }
        self.instance_methods = Cow::Owned(tables);
    }

    /// Replaces a list of values with the output of
    /// `xmake show -l <kind> --json`.
    pub fn apply_list(&mut self, kind: ListKind, json: &str) -> Result<(), OverlayError> {
        let values = &mut self.values;
        match kind {
            ListKind::Platforms => apply_names(&mut values.platforms, json),
            ListKind::BuildModes => apply_names(&mut values.modes, json),
            ListKind::Rules => apply_names(&mut values.rules, json),
            ListKind::Architectures => {
                // `{"windows": ["x86", "x64"], ...}`: the union of all.
                let by_plat: BTreeMap<Cow<'_, str>, Vec<Cow<'_, str>>> =
                    serde_json::from_str(json).map_err(OverlayError)?;
                let mut archs = by_plat
                    .values()
                    .flatten()
                    .map(|a| reuse_sorted(&values.architectures, a))
                    .collect::<Vec<_>>();
                archs.sort_unstable();
                archs.dedup();
                values.architectures = Cow::Owned(archs);
                Ok(())
            }
            ListKind::Policies => apply_named(&mut values.policies, json),
            ListKind::Toolchains => apply_named(&mut values.toolchains, json),
        }
    }
}

fn apply_names(list: &mut Cow<'static, [Str]>, json: &str) -> Result<(), OverlayError> {
    let names: Vec<Cow<'_, str>> = serde_json::from_str(json).map_err(OverlayError)?;
    let mut owned = names.iter().map(|n| reuse_sorted(list, n)).collect::<Vec<_>>();
    owned.sort_unstable();
    owned.dedup();
    *list = Cow::Owned(owned);
    Ok(())
}

fn apply_named(list: &mut Cow<'static, [Named]>, json: &str) -> Result<(), OverlayError> {
    let items: Vec<NamedJson<'_>> = serde_json::from_str(json).map_err(OverlayError)?;
    let mut owned = Vec::with_capacity(items.len());
    for item in &items {
        let known = list.iter().find(|n| n.name == item.name);
        let description = item.description.as_deref().unwrap_or("");
        let named = match known {
            // Same name and text: keep the static strings.
            Some(k) if k.description == description => k.clone(),
            Some(k) => {
                Named { name: k.name.clone(), description: Cow::Owned(description.to_owned()) }
            }
            None => Named {
                name: Cow::Owned(item.name.clone().into_owned()),
                description: Cow::Owned(description.to_owned()),
            },
        };
        owned.push(named);
    }
    owned.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    *list = Cow::Owned(owned);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_apis_replaces_and_reuses() {
        let mut db = ApiDb::builtin();
        let json = r#"{
            "description_scope_apis": ["target.set_kind", "target.set_brand_new", "target.on_magic"],
            "description_builtin_apis": ["includes", "is_plat", "set_project"],
            "script_instance_apis": ["target:name", "target:new_method", "package:version"]
        }"#;
        db.apply_show_apis(json).expect("valid json");

        // Kept: known kind, static name.
        let set_kind = db.scope_api(Scope::Target, "set_kind").expect("set_kind");
        assert!(matches!(set_kind.name, Cow::Borrowed(_)));
        // New APIs, kind inferred.
        assert_eq!(
            db.scope_api(Scope::Target, "set_brand_new").map(|a| a.kind),
            Some(ApiKind::Values)
        );
        assert_eq!(db.scope_api(Scope::Target, "on_magic").map(|a| a.kind), Some(ApiKind::Script));
        // Gone with this xmake.
        assert!(db.scope_api(Scope::Target, "add_files").is_none());
        // Root APIs listed among the builtins are kept, the others dropped.
        assert!(db.scope_api(Scope::Root, "set_project").is_some());
        assert!(db.scope_api(Scope::Root, "add_requires").is_none());
        assert!(db.is_description_function("includes"));
        assert!(!db.is_description_function("set_project"));
        // Task APIs are not listed by xmake but stay.
        assert!(db.scope_api(Scope::Task, "on_run").is_some());

        let methods = db.instance_methods(Instance::Target);
        assert_eq!(methods, ["name", "new_method"]);
        assert!(matches!(methods[0], Cow::Borrowed(_)));
        assert_eq!(db.instance_methods(Instance::Package), ["version"]);
    }

    #[test]
    fn lists() {
        let mut db = ApiDb::builtin();
        db.apply_list(ListKind::Architectures, r#"{"a": ["x64", "arm64"], "b": ["x64"]}"#)
            .expect("json");
        assert_eq!(&*db.values.architectures, ["arm64", "x64"]);
        db.apply_list(
            ListKind::Toolchains,
            r#"[{"name": "zig", "description": "Zig"}, {"name": "new"}]"#,
        )
        .expect("json");
        assert_eq!(db.values.toolchains.len(), 2);
        assert!(db.apply_list(ListKind::Rules, "{").is_err());
    }
}
