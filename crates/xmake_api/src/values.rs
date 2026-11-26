//! The values some APIs accept as string arguments, for completion and checks.

/// Where the accepted values of an argument come from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArgValues {
    /// A fixed list of `(value, description)`.
    Fixed(&'static [(&'static str, &'static str)]),
    /// [`crate::KnownValues::platforms`].
    Platforms,
    /// [`crate::KnownValues::architectures`].
    Architectures,
    /// [`crate::KnownValues::modes`].
    Modes,
    /// [`crate::KnownValues::rules`] plus the rules of the project.
    Rules,
    /// [`crate::KnownValues::policies`].
    Policies,
    /// [`crate::KnownValues::toolchains`] plus those of the project.
    Toolchains,
    /// The targets of the project.
    Targets,
    /// The options of the project.
    Options,
    /// The options of the project and the builtin configurations
    /// ([`BUILTIN_CONFIGS`]): what `get_config` and `is_config` read.
    Configs,
    /// The packages required by the project.
    Packages,
    /// Importable modules: `core.base.json`, ...
    ImportModules,
}

/// The values of the alias `name` of the stubs (`types/values.lua`):
/// `(value, description)`.
pub fn values(name: &str) -> &'static [(&'static str, &'static str)] {
    crate::Stubs::get().alias(name).map_or(&[], |a| a.values.as_slice())
}

/// The keys of the options class `xmake.options.<name>` of the stubs
/// (`types/options.lua`): `(key, description)`.
fn options(name: &str) -> &'static [(&'static str, &'static str)] {
    let class = crate::Stubs::get().class(&format!("xmake.options.{name}"));
    class.map_or(&[], |c| c.field_docs.as_slice())
}

/// The configurations every project has (`xmake f --help`): what
/// `get_config("plat")` reads.
pub fn builtin_configs() -> &'static [(&'static str, &'static str)] {
    values("xmake.value.config")
}

/// The tools of xmake (`modules/core/tools/*.lua`): what `{tools = "cl"}` and
/// `"cl::/W4"` of the flags name.
pub fn tools() -> &'static [(&'static str, &'static str)] {
    values("xmake.value.tool")
}

impl ArgValues {
    /// Whether every valid value is known, so others can be reported.
    pub fn is_closed(self) -> bool {
        matches!(self, ArgValues::Fixed(_))
    }
}

/// The values the `arg`-th argument of `name` accepts in `scope` (`None` in
/// scripts): some APIs mean other things in some scopes.
pub fn scoped_arg_values(scope: Option<crate::Scope>, name: &str, arg: usize) -> Option<ArgValues> {
    use crate::Scope;
    match (scope, name) {
        // The dependencies of a rule are rules; of a package, packages of
        // the repository.
        (Some(Scope::Rule), "add_deps") => Some(ArgValues::Rules),
        (Some(Scope::Package), "add_deps") => None,
        // Rules, packages and toolchains have their own kinds.
        (
            Some(Scope::Rule | Scope::Package | Scope::Toolchain | Scope::Option | Scope::Task),
            "set_kind",
        ) => None,
        _ => arg_values(name, arg),
    }
}

/// The keys of the options table an API takes as last argument:
/// `add_files("a.c", {defines = "A"})`. Empty when unknown.
pub fn option_keys(
    name: &str,
    kind: Option<crate::ApiKind>,
) -> &'static [(&'static str, &'static str)] {
    match name {
        "add_files" | "add_extrafiles" => options("files"),
        "add_tests" => options("test"),
        "add_deps" => options("dep"),
        "set_version" => options("version"),
        "add_headerfiles" | "add_installfiles" => options("install"),
        "add_configfiles" => options("configfile"),
        _ if name.starts_with("add_") && name.ends_with("flags") => options("flags"),
        _ if name.starts_with("add_")
            && matches!(kind, Some(crate::ApiKind::Values | crate::ApiKind::Paths)) =>
        {
            options("visibility")
        }
        _ => &[],
    }
}

/// The values the `arg`-th argument of the API `name` accepts.
///
/// Values APIs take any number of values, so `arg` only matters for APIs
/// whose later arguments differ (`set_policy(name, value)`).
pub fn arg_values(name: &str, arg: usize) -> Option<ArgValues> {
    let values = match name {
        "set_kind" | "is_kind" => ArgValues::Fixed(values("xmake.value.kind")),
        "set_languages" => ArgValues::Fixed(values("xmake.value.language")),
        "set_warnings" => ArgValues::Fixed(values("xmake.value.warning")),
        "set_optimize" => ArgValues::Fixed(values("xmake.value.optimize")),
        "set_symbols" => ArgValues::Fixed(values("xmake.value.symbol")),
        "set_strip" => ArgValues::Fixed(values("xmake.value.strip")),
        "set_fpmodels" => ArgValues::Fixed(values("xmake.value.fpmodel")),
        "set_runtimes" => ArgValues::Fixed(values("xmake.value.runtime")),
        "set_exceptions" => ArgValues::Fixed(values("xmake.value.exception")),
        "set_encodings" => ArgValues::Fixed(values("xmake.value.encoding")),
        "set_category" => ArgValues::Fixed(values("xmake.value.category")),
        "is_plat" | "is_os" | "is_host" | "is_subhost" | "set_plat" | "set_allowedplats"
        | "set_defaultplat" => ArgValues::Platforms,
        "is_arch" | "is_subarch" | "set_arch" | "set_allowedarchs" | "set_defaultarchs" => {
            ArgValues::Architectures
        }
        "is_mode" | "set_allowedmodes" | "set_defaultmode" => ArgValues::Modes,
        "add_rules" | "set_rules" => ArgValues::Rules,
        "set_policy" if arg == 0 => ArgValues::Policies,
        "set_toolchains" | "add_toolchains" => ArgValues::Toolchains,
        "add_deps" => ArgValues::Targets,
        "add_options" | "set_options" => ArgValues::Options,
        "has_config" => ArgValues::Configs,
        "get_config" | "is_config" | "set_config" if arg == 0 => ArgValues::Configs,
        "add_packages" | "has_package" => ArgValues::Packages,
        "import" if arg == 0 => ArgValues::ImportModules,
        _ => return None,
    };
    Some(values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookups() {
        assert!(matches!(arg_values("set_kind", 0), Some(ArgValues::Fixed(_))));
        assert_eq!(arg_values("set_policy", 1), None);
        assert_eq!(arg_values("add_deps", 3), Some(ArgValues::Targets));
        assert_eq!(arg_values("print", 0), None);
        assert!(option_keys("add_tests", None).iter().any(|(k, _)| *k == "runargs"));
        assert!(option_keys("add_cxxflags", None).iter().any(|(k, _)| *k == "force"));
        assert!(
            option_keys("add_includedirs", Some(crate::ApiKind::Paths))
                .iter()
                .any(|(k, _)| *k == "public")
        );
        assert!(option_keys("set_kind", Some(crate::ApiKind::Values)).is_empty());
    }
}
