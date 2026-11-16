//! The parameters of the APIs, shown by hovers and signature help.

use crate::{ApiKind, Instance, Scope};

/// The parameters of `name`: special cases first, then from its kind.
pub fn api_params(name: &str, kind: Option<ApiKind>, scope: Scope) -> &'static [&'static str] {
    if let Some(params) = special_params(name) {
        return params;
    }
    if let Some((_, crate::ScopeEdge::Begin)) = Scope::from_opener(name) {
        return &["name: string", "options?: table | function ()"];
    }
    match kind {
        Some(ApiKind::Values | ApiKind::Groups) => &["values: string...", "options?: table"],
        Some(ApiKind::Paths) => &["paths: string...", "options?: table"],
        Some(ApiKind::KeyValues) => &["name: string", "values: any...", "options?: table"],
        Some(ApiKind::Dictionary) => &["name: string", "value: any"],
        Some(ApiKind::Script) => match scope.script_instance() {
            Some(Instance::Target) => &["script: function (target)"],
            Some(Instance::Option) => &["script: function (option)"],
            Some(Instance::Package) => &["script: function (package)"],
            Some(Instance::Toolchain) => &["script: function (toolchain)"],
            Some(Instance::Rule) | None => &["script: function (...)"],
        },
        Some(ApiKind::Custom) | None => &["..."],
    }
}

/// APIs whose parameters differ from what their kind tells.
fn special_params(name: &str) -> Option<&'static [&'static str]> {
    let params: &[&str] = match name {
        "is_plat" | "is_arch" | "is_mode" | "is_os" | "is_host" | "is_subhost" | "is_kind" => {
            &["values: string..."]
        }
        "is_config" => &["name: string", "values: string..."],
        "get_config" => &["name: string"],
        "has_config" | "has_package" => &["names: string..."],
        "set_policy" => &["name: string", "value: boolean | string"],
        "add_requires" => &["packages: string...", "options?: table"],
        "import" => &["module: string", "options?: table"],
        "includes" => &["paths: string..."],
        "set_xmakever" => &["version: string"],
        "raise" => &["format: string", "args: any..."],
        "print" | "cprint" | "vprint" => &["format: any", "args: any..."],
        "try" => &["block: {function, catch?, finally?}"],
        _ => return None,
    };
    Some(params)
}

/// `name(a, b)`.
pub fn render_signature(prefix: &str, name: &str, params: &[&str]) -> String {
    let mut out = String::with_capacity(prefix.len() + name.len() + 32);
    out.push_str(prefix);
    out.push_str(name);
    out.push('(');
    for (i, param) in params.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(param);
    }
    out.push(')');
    out
}
