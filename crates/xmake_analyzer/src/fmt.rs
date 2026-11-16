//! `xmake_analyzer fmt`: formats files from the command line.

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use xmake_ide::{
    AnalyzedFile, FileKind,
    xmake_syntax::{Dialect, LuaVersion},
};

pub(crate) const USAGE: &str = "\
Format xmake scripts with the settings of .editorconfig.

USAGE:
    xmake_analyzer fmt [OPTIONS] [PATH]...

PATH is a file, or a directory searched for *.lua files (default: `.`).

OPTIONS:
    --check            Do not write: list the files that would change and
                       exit with 1 if there is any.
    --lua <VERSION>    5.1, 5.2, 5.3, 5.4 (default), 5.5 or luajit.
    --tabs <N>         Indent with N tabs per level.
    --spaces <N>       Indent with N spaces per level.
    --no-scope-indent  Do not indent the contents of target(), option(), ...";

/// Runs the command, returning the exit code.
#[allow(clippy::print_stdout, clippy::print_stderr)]
pub(crate) fn run(args: &[String]) -> i32 {
    let mut check = false;
    let mut dialect = Dialect::XMAKE_DEFAULT;
    let mut indent = None;
    let mut scope_indent = true;
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        let count =
            |value: Option<&String>| value.and_then(|v| v.parse::<u8>().ok()).filter(|&n| n > 0);
        match arg.as_str() {
            "-h" | "--help" => {
                println!("{USAGE}");
                return 0;
            }
            "--check" => check = true,
            "--no-scope-indent" => scope_indent = false,
            "--lua" => match args.next().and_then(|v| LuaVersion::parse(v)) {
                Some(lua) => dialect = Dialect { lua },
                None => return usage_error("--lua expects 5.1 to 5.5 or luajit"),
            },
            "--tabs" => match count(args.next()) {
                Some(n) => indent = Some(xmake_fmt::Indent::Tabs(n)),
                None => return usage_error("--tabs expects a positive number"),
            },
            "--spaces" => match count(args.next()) {
                Some(n) => indent = Some(xmake_fmt::Indent::Spaces(n)),
                None => return usage_error("--spaces expects a positive number"),
            },
            other if other.starts_with('-') => {
                return usage_error(&format!("unknown option {other}"));
            }
            path => paths.push(PathBuf::from(path)),
        }
    }
    if paths.is_empty() {
        paths.push(PathBuf::from("."));
    }

    let mut files = Vec::new();
    for path in &paths {
        collect(path, &mut files);
    }
    let mut changed = 0;
    let mut failed = 0;
    for file in &files {
        match format_file(file, dialect, indent, scope_indent) {
            Ok(None) => {}
            Ok(Some(_)) if check => {
                println!("{}", file.display());
                changed += 1;
            }
            Ok(Some(formatted)) => match std::fs::write(file, formatted) {
                Ok(()) => changed += 1,
                Err(err) => {
                    eprintln!("{}: {err}", file.display());
                    failed += 1;
                }
            },
            Err(err) => {
                eprintln!("{}: {err}", file.display());
                failed += 1;
            }
        }
    }
    if !check {
        eprintln!("formatted {changed} of {} files", files.len());
    }
    i32::from(failed > 0 || (check && changed > 0))
}

#[allow(clippy::print_stderr)]
fn usage_error(message: &str) -> i32 {
    eprintln!("error: {message}\n\n{USAGE}");
    2
}

/// The formatted text of `path`, `None` when it is already formatted.
fn format_file(
    path: &Path,
    dialect: Dialect,
    indent: Option<xmake_fmt::Indent>,
    scope_indent: bool,
) -> Result<Option<String>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    let text: Arc<str> = Arc::from(text);
    let is_description =
        AnalyzedFile::new(Arc::clone(&text), file_name, dialect).kind == FileKind::Description;
    let options = xmake_fmt::FormatOptions::for_file(path, |o| {
        o.indent_scopes = scope_indent && is_description;
    });
    // Explicit flags win over `.editorconfig`.
    let options = xmake_fmt::FormatOptions { indent: indent.unwrap_or(options.indent), ..options };
    let formatted = xmake_fmt::format(&text, dialect, options).map_err(|e| e.to_string())?;
    Ok((formatted != *text).then_some(formatted))
}

/// `path` itself, or the `*.lua` files under it.
fn collect(path: &Path, out: &mut Vec<PathBuf>) {
    if !path.is_dir() {
        out.push(path.to_path_buf());
        return;
    }
    let Ok(entries) = std::fs::read_dir(path) else {
        return;
    };
    let mut entries: Vec<_> = entries.flatten().map(|e| e.path()).collect();
    entries.sort();
    for entry in entries {
        let name = entry.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if entry.is_dir() {
            if !name.starts_with('.') && !matches!(name, "build" | "node_modules" | "target") {
                collect(&entry, out);
            }
        } else if name.ends_with(".lua") {
            out.push(entry);
        }
    }
}
