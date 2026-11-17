//! The lints: every non-syntax diagnostic has a name and a level.
//!
//! Levels come from the defaults below, then the settings
//! ([`LintLevels::set`]), then comments in the file:
//!
//! ```lua
//! -- xmake-analyzer: allow(unused-local)        -- this line and the next
//! local unused = 1
//! -- xmake-analyzer: allow-file(undefined-global)  -- the whole file
//! ```

use xmake_syntax::{SyntaxKind, TextSize};

use crate::{AnalyzedFile, Diagnostic, Severity};

/// How a lint is reported.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Level {
    /// Not reported.
    Allow,
    /// A warning.
    Warn,
    /// An error.
    Deny,
}

impl Level {
    /// Parses `allow`, `warn` or `deny` (also `off`, `warning`, `error`).
    pub fn parse(s: &str) -> Option<Level> {
        match s.trim().to_ascii_lowercase().as_str() {
            "allow" | "off" | "none" => Some(Level::Allow),
            "warn" | "warning" => Some(Level::Warn),
            "deny" | "error" => Some(Level::Deny),
            _ => None,
        }
    }
}

/// A lint and its documentation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Lint {
    pub name: &'static str,
    pub default: Level,
    pub description: &'static str,
}

macro_rules! lints {
    ($($name:literal => $level:ident, $description:literal;)*) => {
        /// Every lint, sorted by name.
        pub static LINTS: &[Lint] = &[$(Lint { name: $name, default: Level::$level, description: $description }),*];
    };
}

// `Deny` for what makes xmake fail (it raises), `Warn` for what xmake
// accepts: a missing target in `add_deps` and an unmatched `add_files` are
// skipped (with a warning for the latter), so they warn here too.
lints! {
    "dependency-cycle" => Deny, "Targets depending on each other through `add_deps`: xmake fails with `circular dependency`.";
    "deprecated-api" => Warn, "An API replaced by another one, e.g. `add_subdirs` by `includes`.";
    "deprecated-function" => Warn, "A call of a function annotated `---@deprecated`.";
    "duplicate-definition" => Warn, "Two targets, options, rules, ... with the same name.";
    "invalid-annotation" => Warn, "A malformed LuaCATS annotation: an unknown tag (`---@parm`), a missing type, unbalanced brackets.";
    "mismatched-scope-end" => Warn, "A `*_end()` closing another scope than the open one.";
    "mismatched-type" => Warn, "An argument of the wrong type: `set_default(\"false\")`, `os.cp(\"a\", 1)`.";
    "missing-require" => Warn, "`add_packages` of a package no `add_requires` requires.";
    "self-dependency" => Deny, "A target in its own `add_deps`: xmake fails with `circular dependency`.";
    "undefined-global" => Warn, "A name defined nowhere: not a local, a global of the project, an API or a builtin.";
    "unknown-api" => Deny, "A call looking like an API (`set_*`, `add_*`, ...) that xmake does not have: xmake fails calling it.";
    "unknown-function" => Deny, "A function a builtin module does not have, e.g. `os.cpp`: the script fails calling `nil`.";
    "unknown-method" => Deny, "A method the instance of a script does not have, e.g. `target:frobnicate()`: the script fails calling `nil`.";
    "unknown-option" => Warn, "`add_options` of an option the project does not define.";
    "unknown-package" => Deny, "`add_requires` of a package no repository has: xmake fails with `package(x) not found`.";
    "unknown-package-version" => Deny, "`add_requires` of a version the package does not have: xmake fails with `version(x) not found`.";
    "unknown-param" => Warn, "`---@param` of a name the function below does not have.";
    "unknown-rule" => Deny, "`add_rules` of a rule neither builtin nor defined by the project: xmake fails with `unknown rule`.";
    "unknown-target" => Warn, "`add_deps` of a target the project does not define.";
    "unknown-toolchain" => Deny, "`set_toolchains` of a toolchain neither builtin nor defined by the project: xmake fails with `toolchain not found`.";
    "unknown-type" => Warn, "A type in an annotation neither builtin, a class of the stubs, nor declared with `---@class`, `---@alias`, `---@enum` or `---@generic`.";
    "unknown-value" => Warn, "A value an API does not accept, e.g. `set_kind(\"bin\")`.";
    "unmatched-path" => Warn, "A path matching nothing: `add_files` of a missing file, `add_includedirs` of a missing directory, a glob without match.";
    "unresolved-include" => Warn, "`includes` of a path loading nothing: no such file, nor a directory with an `xmake.lua`.";
    "unused-local" => Warn, "A local variable or function never used. Prefix it with `_` to keep it.";
    "unused-require" => Warn, "A package required with `add_requires` that no `add_packages` uses.";
    "wrong-scope" => Deny, "An API called in a scope it does not belong to, e.g. `set_kind` in an option: xmake fails with `cannot be called in`.";
    "xmake-version" => Deny, "`set_xmakever` newer than the installed xmake: xmake refuses to load the project.";
}

/// The lint named `name`.
pub fn lint(name: &str) -> Option<&'static Lint> {
    LINTS.binary_search_by(|l| l.name.cmp(name)).ok().map(|i| &LINTS[i])
}

/// Levels set by the user, over the defaults.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LintLevels {
    overrides: Vec<(&'static str, Level)>,
}

impl LintLevels {
    /// Sets the level of `name`; `false` for an unknown lint.
    pub fn set(&mut self, name: &str, level: Level) -> bool {
        let Some(lint) = lint(name) else {
            return false;
        };
        self.overrides.retain(|(n, _)| *n != lint.name);
        self.overrides.push((lint.name, level));
        true
    }

    /// The level of the lint `name`; non-lint diagnostics (syntax) are errors.
    pub fn level(&self, name: &str) -> Level {
        let set = self.overrides.iter().find(|(n, _)| *n == name).map(|(_, l)| *l);
        set.or_else(|| lint(name).map(|l| l.default)).unwrap_or(Level::Deny)
    }
}

/// Applies the levels and the `allow` comments of `file` to `diagnostics`.
pub(crate) fn apply(
    levels: &LintLevels,
    file: &AnalyzedFile,
    diagnostics: &mut Vec<Diagnostic<'_>>,
) {
    let allows = allow_comments(file);
    diagnostics.retain_mut(|d| {
        if lint(d.code).is_none() {
            return true;
        }
        let level = levels.level(d.code);
        let line = file.line_index.line_col(&file.text, d.range.start()).line;
        let allowed = allows.iter().any(|a| {
            a.names.split(',').any(|n| n.trim() == d.code)
                && (a.whole_file || a.line == line || a.line + 1 == line)
        });
        d.severity = match level {
            Level::Allow => return false,
            Level::Warn => Severity::Warning,
            Level::Deny => Severity::Error,
        };
        !allowed
    });
}

struct Allow<'a> {
    line: u32,
    whole_file: bool,
    names: &'a str,
}

/// The `-- xmake-analyzer: allow(...)` comments.
fn allow_comments(file: &AnalyzedFile) -> Vec<Allow<'_>> {
    const MARKER: &str = "xmake-analyzer:";
    let mut out = Vec::new();
    for token in file.root().descendant_tokens().filter(|t| t.kind() == SyntaxKind::LINE_COMMENT) {
        let Some((_, rest)) = token.text().split_once(MARKER) else {
            continue;
        };
        let rest = rest.trim_start();
        let (whole_file, rest) = match rest.strip_prefix("allow-file(") {
            Some(rest) => (true, rest),
            None => match rest.strip_prefix("allow(") {
                Some(rest) => (false, rest),
                None => continue,
            },
        };
        let names = rest.split(')').next().unwrap_or("");
        let line = line_of(file, token.text_range().start());
        out.push(Allow { line, whole_file, names });
    }
    out
}

fn line_of(file: &AnalyzedFile, offset: TextSize) -> u32 {
    file.line_index.line_col(&file.text, offset).line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorted_and_found() {
        assert!(LINTS.windows(2).all(|w| w[0].name < w[1].name), "LINTS must be sorted");
        assert_eq!(lint("unused-local").map(|l| l.default), Some(Level::Warn));
        let mut levels = LintLevels::default();
        assert!(levels.set("unused-local", Level::Allow));
        assert!(!levels.set("nope", Level::Allow));
        assert_eq!(levels.level("unused-local"), Level::Allow);
        assert_eq!(levels.level("self-dependency"), Level::Deny);
        assert_eq!(levels.level("syntax"), Level::Deny);
    }
}
