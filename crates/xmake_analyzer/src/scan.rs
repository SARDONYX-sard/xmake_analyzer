//! Finding the description files of the workspace.

use std::path::{Path, PathBuf};

/// Directories never searched: build outputs, caches, VCS data.
const SKIPPED_DIRS: &[&str] = &["build", "node_modules", "target", "out", "vendor"];

/// Limits keeping a scan of a huge folder (e.g. a home directory) cheap.
const MAX_DEPTH: usize = 12;
const MAX_FILES: usize = 4096;

/// The `xmake.lua` files under `roots`, with their contents.
pub(crate) fn scan(roots: &[PathBuf]) -> Vec<(PathBuf, String)> {
    let mut found = Vec::new();
    for root in roots {
        visit(root, 0, &mut found);
    }
    found
}

fn visit(dir: &Path, depth: usize, found: &mut Vec<(PathBuf, String)>) {
    if depth > MAX_DEPTH || found.len() >= MAX_FILES {
        return;
    }
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let Ok(kind) = entry.file_type() else {
            continue;
        };
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            continue;
        };
        if kind.is_dir() {
            if !name.starts_with('.') && !SKIPPED_DIRS.contains(&name) {
                visit(&entry.path(), depth + 1, found);
            }
        } else if name.eq_ignore_ascii_case("xmake.lua") {
            let path = entry.path();
            match std::fs::read_to_string(&path) {
                Ok(text) => found.push((path, text)),
                Err(err) => log::warn!("cannot read {}: {err}", path.display()),
            }
        }
    }
}
