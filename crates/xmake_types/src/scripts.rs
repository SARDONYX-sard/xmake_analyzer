//! The script APIs of the description: `on_build`, `before_link`, ... take
//! more than the `script` of their prototype in the manual.
//!
//! ```lua
//! on_build(function (target) end)                    -- the script
//! on_run("main")                                     -- a script by name
//! on_build("windows|x64", "linux", function (target) end)  -- for those only
//! on_build(function (target) end, {extra = true})   -- an extra config
//! ```
//!
//! What xmake does (`interpreter:_api_register_xxx_script`): a last
//! dictionary is the extra config; the last argument left is the script;
//! the ones before are `plat|arch` patterns. LuaCATS has no signature for
//! patterns before a script: the second overload names one, `...` more.

use xmake_syntax::{SyntaxKind, SyntaxNode};

use crate::ty::luacats_param;

/// What an argument of a script API is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScriptArg {
    /// `"windows|x64"`: the platforms and architectures the script is for.
    Pattern,
    Script,
    /// `{...}` after the script.
    Extra,
}

/// The LuaCATS type of the script of a script API, when `prototype` is one:
/// `fun(target: target)|string` for `on_build(script: function (target))`.
pub fn script_param(prototype: &str) -> Option<String> {
    // `on_prepare_files(script: function (...), {jobgraph})`: the options
    // that may follow are the extra config.
    let params = xmake_api::prototype_params(prototype);
    let (&param, rest) = params.split_first()?;
    if !rest.iter().all(|p| p.starts_with('{')) {
        return None;
    }
    let (name, ty) = param.split_once(':')?;
    (name.trim() == "script" && ty.trim_start().starts_with("function"))
        .then(|| luacats_param(param).into_owned())
}

/// The overloads of a script API: the script alone, and the script after
/// patterns.
pub fn script_overloads(prototype: &str) -> Option<[String; 2]> {
    let script = script_param(prototype)?;
    let name = prototype.split('(').next().unwrap_or(prototype);
    Some([format!("{name}({script})"), format!("{name}(pattern: string, ...: string, {script})")])
}

/// The role of each of `args`, the arguments of a call of a script API.
pub fn script_args(args: &[SyntaxNode<'_>]) -> Vec<ScriptArg> {
    let count = args.len();
    let extra = count >= 2 && args[count - 1].kind() == SyntaxKind::TABLE_EXPR;
    let script = count - usize::from(extra);
    (0..count)
        .map(|i| match i {
            _ if extra && i == count - 1 => ScriptArg::Extra,
            _ if i + 1 == script => ScriptArg::Script,
            _ => ScriptArg::Pattern,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use xmake_syntax::{Dialect, parse};

    use super::*;

    fn roles(call: &str) -> Vec<ScriptArg> {
        let parsed = parse(call, Dialect::XMAKE_DEFAULT);
        let root = parsed.root(call);
        let args = root.descendants().find(|n| n.kind() == SyntaxKind::ARG_LIST).expect("args");
        let args: Vec<_> = args.children().collect();
        script_args(&args)
    }

    #[test]
    fn overloads_and_roles() {
        let [alone, patterns] =
            script_overloads("on_build(script: function (target))").expect("script");
        assert_eq!(alone, "on_build(script: fun(target: target)|string)");
        assert_eq!(
            patterns,
            "on_build(pattern: string, ...: string, script: fun(target: target)|string)"
        );
        assert!(script_overloads("set_kind(kind: string)").is_none());
        assert!(
            script_param("on_prepare_files(script: function (target, jobgraph), {jobgraph})")
                .is_some()
        );

        use ScriptArg::{Extra, Pattern, Script};
        assert_eq!(roles("on_build(function () end)"), [Script]);
        assert_eq!(roles("on_run('main')"), [Script]);
        assert_eq!(
            roles("on_build('windows', 'linux', function () end)"),
            [Pattern, Pattern, Script]
        );
        assert_eq!(roles("on_build(function () end, {a = 1})"), [Script, Extra]);
    }
}
