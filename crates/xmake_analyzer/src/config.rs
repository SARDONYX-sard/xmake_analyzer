//! Settings, from `initializationOptions` and `workspace/didChangeConfiguration`.
//!
//! ```json
//! {
//!   "xmakePath": "xmake",
//!   "luaVersion": "auto",
//!   "queryXmake": true,
//!   "diagnostics": { "enable": true },
//!   "format": { "indentScopes": true, "maxBlankLines": 1 },
//!   "codeLens": { "runTask": false, "references": false },
//!   "lints": { "unused-local": "allow", "undefined-global": "deny" }
//! }
//! ```
//!
//! `codeLens.runTask` adds a "Run" lens on tasks, invoking the client command
//! `xmake-analyzer.runTask` with the task name and the file URI: only clients
//! providing it set it.
//!
//! The indentation itself comes from the editor and `.editorconfig`.
//!
//! The settings may also be nested under an `xmake-analyzer` key, which is
//! how editors send a whole configuration section.

use std::path::PathBuf;

use serde::Deserialize;
use xmake_ide::xmake_syntax::LuaVersion;

/// The Lua version scripts are checked against.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum LuaSetting {
    /// The runtime of the installed xmake (Lua 5.4 or LuaJIT).
    #[default]
    Auto,
    Fixed(LuaVersion),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Config {
    /// The xmake executable.
    pub(crate) xmake_path: PathBuf,
    pub(crate) lua: LuaSetting,
    /// Whether to read the API lists from the installed xmake.
    pub(crate) query_xmake: bool,
    pub(crate) diagnostics: bool,
    pub(crate) format: FormatSettings,
    /// Lenses running tasks, see the module documentation.
    pub(crate) run_task_lens: bool,
    /// Lenses counting the references of targets, options, ... invoking the
    /// client command `xmake-analyzer.showReferences(uri, position,
    /// locations)` with protocol values.
    pub(crate) references_lens: bool,
    /// The levels of the lints: `allow`, `warn` or `deny` by name.
    pub(crate) lints: xmake_ide::LintLevels,
}

/// Formatter settings not covered by the editor and `.editorconfig`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct FormatSettings {
    /// Indent the contents of `target(...)`, `option(...)`, ...
    pub(crate) indent_scopes: bool,
    pub(crate) max_blank_lines: usize,
}

impl Default for FormatSettings {
    fn default() -> FormatSettings {
        FormatSettings { indent_scopes: true, max_blank_lines: 1 }
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            xmake_path: PathBuf::from("xmake"),
            lua: LuaSetting::Auto,
            query_xmake: true,
            diagnostics: true,
            format: FormatSettings::default(),
            run_task_lens: false,
            references_lens: false,
            lints: xmake_ide::LintLevels::default(),
        }
    }
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
struct RawConfig {
    xmake_path: Option<String>,
    lua_version: Option<String>,
    query_xmake: Option<bool>,
    diagnostics: Option<RawDiagnostics>,
    format: Option<RawFormat>,
    code_lens: Option<RawCodeLens>,
    lints: Option<std::collections::BTreeMap<String, String>>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
struct RawCodeLens {
    run_task: Option<bool>,
    references: Option<bool>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
struct RawFormat {
    indent_scopes: Option<bool>,
    max_blank_lines: Option<usize>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct RawDiagnostics {
    enable: Option<bool>,
}

impl Config {
    /// Reads the settings, keeping the defaults for missing or invalid ones.
    pub(crate) fn from_json(value: Option<&serde_json::Value>) -> Config {
        let mut config = Config::default();
        let Some(mut value) = value else {
            return config;
        };
        if let Some(section) = value.get("xmake-analyzer") {
            value = section;
        }
        let raw = match RawConfig::deserialize(value) {
            Ok(raw) => raw,
            Err(err) => {
                log::warn!("invalid settings: {err}");
                return config;
            }
        };
        if let Some(path) = raw.xmake_path.filter(|p| !p.trim().is_empty()) {
            config.xmake_path = PathBuf::from(path);
        }
        if let Some(lua) = raw.lua_version.as_deref() {
            config.lua = match lua.trim() {
                "" | "auto" => LuaSetting::Auto,
                other => match LuaVersion::parse(other) {
                    Some(version) => LuaSetting::Fixed(version),
                    None => {
                        log::warn!("unknown `luaVersion` {other:?}, using the xmake runtime");
                        LuaSetting::Auto
                    }
                },
            };
        }
        config.query_xmake = raw.query_xmake.unwrap_or(config.query_xmake);
        config.diagnostics = raw.diagnostics.and_then(|d| d.enable).unwrap_or(config.diagnostics);
        for (name, level) in raw.lints.iter().flatten() {
            match xmake_ide::Level::parse(level) {
                Some(level) if config.lints.set(name, level) => {}
                Some(_) => log::warn!("unknown lint {name:?} in `lints`"),
                None => log::warn!("invalid level {level:?} of lint {name:?}: allow, warn or deny"),
            }
        }
        config.references_lens =
            raw.code_lens.as_ref().and_then(|c| c.references).unwrap_or(config.references_lens);
        config.run_task_lens =
            raw.code_lens.and_then(|c| c.run_task).unwrap_or(config.run_task_lens);
        if let Some(format) = raw.format {
            config.format.indent_scopes =
                format.indent_scopes.unwrap_or(config.format.indent_scopes);
            config.format.max_blank_lines =
                format.max_blank_lines.unwrap_or(config.format.max_blank_lines);
        }
        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let json = serde_json::json!({ "xmake-analyzer": { "luaVersion": "luajit", "diagnostics": { "enable": false } } });
        let config = Config::from_json(Some(&json));
        assert_eq!(config.lua, LuaSetting::Fixed(LuaVersion::LuaJit));
        assert!(!config.diagnostics);
        assert!(config.query_xmake);
        assert_eq!(Config::from_json(None), Config::default());
    }
}
