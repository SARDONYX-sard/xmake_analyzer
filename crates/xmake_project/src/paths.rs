//! Resolving the paths written in scripts: `includes("../")`,
//! `add_files("src/**.c|main.c")`.
//!
//! Paths are relative to the directory of the script. `*` matches inside a
//! path segment, `**` across segments, and `|` starts exclusions, as in xmake.

use std::path::{Component, Path, PathBuf};

/// Directories a glob never descends into.
const SKIPPED_DIRS: &[&str] = &["build", "node_modules", "target"];

/// Stop matching after this many files: a glob over a huge tree is useless
/// as a navigation target anyway.
const MAX_MATCHES: usize = 256;

/// Removes `.` and `..` without touching the file system, so that paths
/// written differently compare equal.
pub fn normalize(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if !out.pop() {
                    out.push("..");
                }
            }
            other => out.push(other),
        }
    }
    out
}

/// The file of the module `module` imported by a script of `dir`: `a.b` is
/// `a/b.lua`, `a/b/main.lua` or `a/b/xmake.lua`. `None` for the modules of
/// xmake (`core.base.json`).
pub fn resolve_module(dir: &Path, module: &str) -> Option<PathBuf> {
    let relative: PathBuf = module.split('.').collect();
    let module = dir.join(relative);
    [module.with_extension("lua"), module.join("main.lua"), module.join("xmake.lua")]
        .into_iter()
        .find(|c| c.is_file())
        .map(|p| normalize(&p))
}

/// The description files `includes(value)` loads, from a script in `dir`:
/// a file, `dir/xmake.lua` for a directory, or the matches of a glob.
/// Builtin includes (`@builtin/check`) have no file.
pub fn resolve_include(dir: &Path, value: &str) -> Vec<PathBuf> {
    if value.starts_with('@') || value.is_empty() {
        return Vec::new();
    }
    if value.contains('*') {
        return glob(dir, value);
    }
    let path = normalize(&dir.join(value));
    if path.is_dir() {
        let script = path.join("xmake.lua");
        return if script.is_file() { vec![script] } else { Vec::new() };
    }
    if path.is_file() { vec![path] } else { Vec::new() }
}

/// The files a path argument (`add_files`, `add_headerfiles`, ...) names.
/// Directories are left out: an editor cannot open them.
pub fn resolve_paths(dir: &Path, value: &str) -> Vec<PathBuf> {
    // `src/*.c|main.c|test_*.c`: the first part, minus the exclusions.
    let mut parts = value.split('|');
    let pattern = parts.next().unwrap_or("").trim();
    if pattern.is_empty() || pattern.starts_with('$') {
        return Vec::new();
    }
    let excluded: Vec<&str> = parts.map(str::trim).filter(|p| !p.is_empty()).collect();
    if !pattern.contains('*') {
        let path = normalize(&dir.join(pattern));
        return if path.is_file() { vec![path] } else { Vec::new() };
    }
    let mut found = glob(dir, pattern);
    found.retain(|path| {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        !excluded.iter().any(|e| glob_match(e.as_bytes(), name.as_bytes()))
    });
    found
}

/// What a path argument names, for [`check_path`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PathKind {
    /// Files, globs allowed: `add_files("src/**.c|test/*.c")`.
    Files,
    /// A directory: `add_includedirs("include")`.
    Dir,
}

/// Whether a path argument matches something, `None` when it cannot be
/// checked (`$(builddir)/gen.c` names files the build creates).
pub fn check_path(dir: &Path, value: &str, kind: PathKind) -> Option<bool> {
    let value = value.trim();
    // `$(scriptdir)` is the directory of the script; other variables are
    // known at build time only.
    let value = match value.strip_prefix("$(scriptdir)") {
        Some(rest) => rest.trim_start_matches(['/', '\\']),
        None if value.contains("$(") => return None,
        None => value,
    };
    if value.is_empty() {
        return None;
    }
    match kind {
        PathKind::Dir if value.contains('*') => None,
        PathKind::Dir => Some(normalize(&dir.join(value)).is_dir()),
        PathKind::Files => {
            // `include/(**.h)`: the parentheses only tell what to keep when
            // installing.
            let pattern: String = value.chars().filter(|c| *c != '(' && *c != ')').collect();
            Some(!resolve_paths(dir, &pattern).is_empty())
        }
    }
}

/// The files under `dir` matching `pattern`, sorted.
fn glob(dir: &Path, pattern: &str) -> Vec<PathBuf> {
    let pattern = pattern.replace('\\', "/");
    // Walk from the longest prefix without wildcards.
    let wild = pattern.find('*').unwrap_or(pattern.len());
    let base_len = pattern[..wild].rfind('/').map_or(0, |i| i + 1);
    let base = normalize(&dir.join(&pattern[..base_len]));
    let rest = &pattern[base_len..];
    let mut found = Vec::new();
    walk(&base, "", rest.as_bytes(), &mut found);
    found.sort();
    found
}

fn walk(dir: &Path, relative: &str, pattern: &[u8], found: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        if found.len() >= MAX_MATCHES {
            return;
        }
        let Ok(name) = entry.file_name().into_string() else {
            continue;
        };
        let path = if relative.is_empty() { name.clone() } else { format!("{relative}/{name}") };
        let Ok(kind) = entry.file_type() else {
            continue;
        };
        if kind.is_dir() {
            if !name.starts_with('.') && !SKIPPED_DIRS.contains(&name.as_str()) {
                walk(&entry.path(), &path, pattern, found);
            }
        } else if glob_match(pattern, path.as_bytes()) {
            found.push(entry.path());
        }
    }
}

/// Matches `text` (a `/`-separated relative path) against an xmake pattern.
fn glob_match(pattern: &[u8], text: &[u8]) -> bool {
    match pattern.first() {
        None => text.is_empty(),
        Some(b'*') if pattern.get(1) == Some(&b'*') => {
            // `**/x` also matches `x` at this level.
            let rest = &pattern[2..];
            let rest_after_slash = rest.strip_prefix(b"/").unwrap_or(rest);
            glob_match(rest_after_slash, text)
                || (0..=text.len()).any(|i| glob_match(rest, &text[i..]))
        }
        Some(b'*') => {
            let rest = &pattern[1..];
            (0..=text.len())
                .take_while(|&i| i == 0 || text[i - 1] != b'/')
                .any(|i| glob_match(rest, &text[i..]))
        }
        Some(&c) => {
            let same = text
                .first()
                .is_some_and(|&t| t == c || (cfg!(windows) && t.eq_ignore_ascii_case(&c)));
            same && glob_match(&pattern[1..], &text[1..])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn patterns() {
        assert!(glob_match(b"*.c", b"main.c"));
        assert!(!glob_match(b"*.c", b"sub/main.c"));
        assert!(glob_match(b"**.c", b"sub/deep/main.c"));
        assert!(glob_match(b"**/xmake.lua", b"xmake.lua"));
        assert!(glob_match(b"**/xmake.lua", b"a/b/xmake.lua"));
        assert!(!glob_match(b"**/xmake.lua", b"a/b/other.lua"));
    }

    #[test]
    fn normalizes() {
        assert_eq!(normalize(Path::new("a/b/../c/./d")), PathBuf::from("a/c/d"));
    }

    #[test]
    fn resolves_on_disk() {
        let dir = std::env::temp_dir().join(format!("xmake-ide-paths-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("lib/sub")).expect("dirs");
        std::fs::create_dir_all(dir.join("app")).expect("dirs");
        for file in [
            "xmake.lua",
            "lib/xmake.lua",
            "lib/sub/xmake.lua",
            "lib/a.c",
            "lib/main.c",
            "rules.lua",
        ] {
            std::fs::write(dir.join(file), "").expect("file");
        }
        let app = dir.join("app");
        assert_eq!(resolve_include(&app, "../"), [normalize(&dir.join("xmake.lua"))]);
        assert_eq!(resolve_include(&app, "../rules.lua"), [normalize(&dir.join("rules.lua"))]);
        assert_eq!(resolve_include(&dir, "lib/**/xmake.lua").len(), 2);
        assert!(resolve_include(&dir, "@builtin/check").is_empty());
        let sources = resolve_paths(&dir.join("lib"), "*.c|main.c");
        assert_eq!(sources, [dir.join("lib").join("a.c")]);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
