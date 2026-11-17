//! The variables xmake substitutes in strings: `"$(scriptdir)/src"`,
//! `"$(env HOME)"`, `"$(mode)"`, ...
//!
//! What xmake does (`core/project/project.lua`, `core/base/filter.lua`):
//! `$(shell cmd)`, `$(env NAME)` and `$(reg path)` run, read the environment
//! or the registry; `$(builddir)` is the build directory; any other name is a
//! config (`xmake f --name=...`: `plat`, `mode`, the options of the project)
//! or, when unset, one of the directories and hosts below. The values shown
//! are those known here; the configs depend on `xmake f`.

use std::{borrow::Cow, path::Path};

use xmake_api::Scope;
use xmake_syntax::{TextRange, TextSize};

use crate::{AnalyzedFile, Project};

/// The builtin variables: name and documentation.
pub(crate) const BUILTINS: &[(&str, &str)] = &[
    ("scriptdir", "The directory of the current `xmake.lua`."),
    ("projectdir", "The root directory of the project."),
    (
        "builddir",
        "The build directory: `build` under the project, unless `xmake f -o` says another.",
    ),
    ("buildir", "Deprecated: use `$(builddir)`."),
    ("curdir", "The current directory of xmake."),
    ("tmpdir", "The temporary directory of xmake."),
    ("configdir", "The directory of the configuration: `.xmake/<host>/<arch>` under the project."),
    ("globaldir", "The global directory of xmake: packages, repositories, global config."),
    ("programdir", "The directory of the xmake program and its scripts."),
    ("os", "The operating system of the target platform: `windows`, `linux`, `macosx`, ..."),
    ("host", "The operating system xmake runs on: `windows`, `linux`, `macosx`, ..."),
    ("subhost", "The subsystem xmake runs in: `msys`, `cygwin`, or the host."),
    ("plat", "The target platform (`xmake f -p`)."),
    ("arch", "The target architecture (`xmake f -a`)."),
    ("mode", "The build mode (`xmake f -m`)."),
];

/// The commands: prefix, snippet and documentation.
pub(crate) const COMMANDS: &[(&str, &str, &str)] = &[
    ("env", "env ${1:NAME}", "The value of an environment variable: `$(env PATH)`."),
    (
        "shell",
        "shell ${1:command}",
        "The output of a shell command: `$(shell git rev-parse HEAD)`.",
    ),
    ("reg", "reg ${1:HKEY_LOCAL_MACHINE\\\\path;name}", "A value of the Windows registry."),
];

/// The `$(...)` around `offset` in the string token `text` starting at
/// `start`: its inner range and text (`scriptdir`, `env HOME`).
pub(crate) fn variable_at(
    text: &str,
    start: TextSize,
    offset: TextSize,
) -> Option<(TextRange, &str)> {
    let at = usize::from(offset - start);
    let open = text[..at.min(text.len())].rfind("$(")? + 2;
    let close = open + text[open..].find(')')?;
    // `$(a) x $(b)`: the cursor must be inside this one.
    if at > close || text[open..at.min(close)].contains('(') {
        return None;
    }
    let inner = &text[open..close];
    let range =
        TextRange::new(start + TextSize::new(open as u32), start + TextSize::new(close as u32));
    Some((range, inner))
}

/// The value of the variable `name` in `file`, when known here.
pub(crate) fn value(project: &Project, file: &AnalyzedFile, name: &str) -> Option<String> {
    let path = project.path_of(file);
    let display = |p: &Path| p.display().to_string().replace('\\', "/");
    if let Some(variable) = name.strip_prefix("env ") {
        return std::env::var(variable.trim()).ok();
    }
    match name {
        "scriptdir" => path.and_then(Path::parent).map(display),
        "projectdir" => projectdir(project, file).map(display),
        "builddir" | "buildir" => projectdir(project, file).map(|d| display(&d.join("build"))),
        "globaldir" => crate::default_globaldir().map(|d| display(&d)),
        "host" | "subhost" => Some(host().to_owned()),
        _ => None,
    }
}

/// The directory of the root `xmake.lua` of the project of `file`: the
/// highest among the files `includes()` connects.
fn projectdir<'a>(project: &'a Project, file: &AnalyzedFile) -> Option<&'a Path> {
    let scope = project.scope_of(file);
    let dirs = scope.files().filter_map(|(path, _)| {
        let is_root = path.file_name().is_some_and(|n| n == "xmake.lua");
        is_root.then(|| path.parent()).flatten()
    });
    dirs.min_by_key(|d| d.components().count())
        .or_else(|| project.path_of(file).and_then(Path::parent))
}

/// The host as xmake names it.
fn host() -> &'static str {
    match std::env::consts::OS {
        "macos" => "macosx",
        "freebsd" | "openbsd" | "netbsd" | "dragonfly" => "bsd",
        other => other,
    }
}

/// Every variable of the project with its documentation: the builtins, then
/// the options of the project and the configs of `xmake f`.
pub(crate) fn names<'a>(
    project: &'a Project,
    file: &'a AnalyzedFile,
) -> Vec<(&'a str, Cow<'a, str>)> {
    let mut out: Vec<(&str, Cow<'_, str>)> =
        BUILTINS.iter().map(|&(n, d)| (n, Cow::Borrowed(d))).collect();
    for (_, other) in project.scope_of(file).files() {
        for def in other.outline.definitions.iter().filter(|d| d.scope == Scope::Option) {
            let name = &other.text[def.name_range];
            out.push((
                name,
                Cow::Owned(format!("The option `{name}` of the project (`xmake f --{name}=...`).")),
            ));
        }
    }
    for &(name, doc) in xmake_api::BUILTIN_CONFIGS {
        out.push((name, Cow::Borrowed(doc)));
    }
    out
}

/// The hover of `$(name)`.
pub(crate) fn hover(project: &Project, file: &AnalyzedFile, name: &str) -> String {
    let signature = format!("```lua\n$({name})\n```");
    let doc: Cow<'_, str> = match name.split_once(' ') {
        Some((command, _)) => match COMMANDS.iter().find(|c| c.0 == command) {
            Some(c) => Cow::Borrowed(c.2),
            None => Cow::Borrowed("Not a variable of xmake."),
        },
        None => match names(project, file).into_iter().find(|(n, _)| *n == name) {
            Some((_, doc)) => doc,
            None => Cow::Borrowed("A config: `xmake f --<name>=...`; empty when unset."),
        },
    };
    let value = match (name, value(project, file, name)) {
        (_, Some(value)) => format!("\n\nValue: `{value}`"),
        // `$(shell ...)` is not run for a hover.
        _ => String::new(),
    };
    format!("{signature}\n\n---\n\n{doc}{value}")
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, sync::Arc};

    use xmake_api::ApiDb;
    use xmake_syntax::Dialect;

    use crate::{completions, hover};

    fn setup(text: &str) -> (Project, Arc<AnalyzedFile>, u32) {
        let offset = text.find('|').expect("marker") as u32;
        let text = text.replacen('|', "", 1);
        let mut project = Project::new(ApiDb::builtin(), Dialect::XMAKE_DEFAULT);
        let path = std::env::temp_dir().join("proj").join("xmake.lua");
        let file = project.set_file(PathBuf::from(&path), Arc::from(text.as_str()));
        (project, file, offset)
    }

    use super::*;

    #[test]
    fn hovers() {
        let (project, file, offset) = setup("add_includedirs('$(script|dir)/include')");
        let found = hover(&project, &file, TextSize::new(offset)).expect("hover");
        assert_eq!(&file.text[found.range], "scriptdir");
        let dir = std::env::temp_dir().join("proj").display().to_string().replace('\\', "/");
        assert!(found.markdown.contains(&format!("Value: `{dir}`")), "{}", found.markdown);

        let (project, file, offset) = setup("option('lto')\noption_end()\nprint('$(l|to)')");
        let found = hover(&project, &file, TextSize::new(offset)).expect("hover");
        assert!(found.markdown.contains("The option `lto`"), "{}", found.markdown);
    }

    #[test]
    fn completion() {
        let labels = |text: &str| {
            let (project, file, offset) = setup(text);
            let items = completions(&project, &file, None, TextSize::new(offset));
            items.into_iter().map(|i| i.label.into_owned()).collect::<Vec<_>>()
        };
        let inside = labels("add_files('$(scr|')");
        assert_eq!(inside.first().map(String::as_str), Some("scriptdir"), "{inside:?}");
        let all = labels("option('lto')\noption_end()\nadd_files('$(|)')");
        assert!(all.iter().any(|l| l == "lto") && all.iter().any(|l| l == "env"), "{all:?}");
        let dollar = labels("add_files('$|')");
        assert!(dollar.iter().any(|l| l == "$(projectdir)"), "{dollar:?}");
    }
}
