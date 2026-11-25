//! Embeds the stubs, the typed and documented APIs (see `src/stubs.rs`), as a
//! table of `(file name, text)`: those of the newest xmake under
//! `stubs/<version>/`.
//!
//! Adding a file is enough: nothing to register.

use std::{
    fmt::Write as _,
    path::{Path, PathBuf},
};

/// `3.1.1` as `[3, 1, 1]`, for ordering the versions.
fn version_key(dir: &Path) -> Option<Vec<u64>> {
    let name = dir.file_name()?.to_str()?;
    name.split('.').map(|p| p.parse().ok()).collect()
}

fn lua_files(dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<_> = std::fs::read_dir(dir)
        .map(|e| {
            e.flatten()
                .map(|e| e.path())
                .filter(|p| p.extension().is_some_and(|e| e == "lua"))
                .collect()
        })
        .unwrap_or_default();
    files.sort();
    files
}

fn main() {
    let stubs = Path::new(env!("CARGO_MANIFEST_DIR")).join("stubs");
    println!("cargo::rerun-if-changed={}", stubs.display());
    let newest = std::fs::read_dir(&stubs)
        .map(|e| e.flatten().map(|e| e.path()).filter(|p| p.is_dir()).collect::<Vec<_>>())
        .unwrap_or_default()
        .into_iter()
        .filter_map(|dir| Some((version_key(&dir)?, dir)))
        .max();
    let (version, dir) = match newest {
        Some((_, dir)) => {
            let name = dir.file_name().and_then(|n| n.to_str()).unwrap_or_default().to_owned();
            (name, dir)
        }
        None => (String::new(), stubs),
    };
    println!("cargo::rerun-if-changed={}", dir.display());

    let mut table = String::new();
    let _ = writeln!(table, "/// The xmake version of the embedded stubs.");
    let _ = writeln!(table, "pub const BUILTIN_VERSION: &str = {version:?};");
    let _ = writeln!(table, "pub(crate) static STUB_FILES: &[(&str, &str)] = &[");
    for file in lua_files(&dir) {
        let name = file.file_name().and_then(|s| s.to_str()).unwrap_or_default().to_owned();
        let _ = writeln!(table, "    ({name:?}, include_str!({:?})),", file.display().to_string());
    }
    let _ = writeln!(table, "];");

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR is set by cargo");
    std::fs::write(Path::new(&out_dir).join("stubs_table.rs"), table)
        .expect("write stubs_table.rs");
}
