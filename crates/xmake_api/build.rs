//! Embeds `stubs/*.lua`, the typed and documented APIs (see `src/stubs.rs`),
//! as a table of `(name, text)`.
//!
//! Adding a file is enough: nothing to register.

use std::{fmt::Write as _, path::Path};

fn main() {
    let mut stub_table = String::new();
    // The stubs: `stubs/*.lua`.
    let stubs = Path::new(env!("CARGO_MANIFEST_DIR")).join("stubs");
    println!("cargo::rerun-if-changed={}", stubs.display());
    let mut files: Vec<_> = std::fs::read_dir(&stubs)
        .map(|e| {
            e.flatten()
                .map(|e| e.path())
                .filter(|p| p.extension().is_some_and(|e| e == "lua"))
                .collect()
        })
        .unwrap_or_default();
    files.sort();
    let _ = writeln!(stub_table, "pub(crate) static STUB_FILES: &[(&str, &str)] = &[");
    for file in files {
        let name = file.file_stem().and_then(|s| s.to_str()).unwrap_or_default().to_owned();
        let _ =
            writeln!(stub_table, "    ({name:?}, include_str!({:?})),", file.display().to_string());
    }
    let _ = writeln!(stub_table, "];");

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR is set by cargo");
    std::fs::write(Path::new(&out_dir).join("stubs_table.rs"), stub_table)
        .expect("write stubs_table.rs");
}
