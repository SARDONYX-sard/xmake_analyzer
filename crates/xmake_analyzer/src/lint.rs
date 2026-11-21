//! `xmake_analyzer lint`: the diagnostics of a project on the command line.

use std::{path::PathBuf, sync::Arc};

use xmake_ide::{
    Level, Project, Severity,
    xmake_api::ApiDb,
    xmake_syntax::{Dialect, LuaVersion},
};

pub(crate) const USAGE: &str = "\
Check xmake projects: syntax, API misuses and the lints.

USAGE:
    xmake_analyzer lint [OPTIONS] [DIR]...

DIR is searched for xmake.lua files, followed with their includes()
(default: `.`).

OPTIONS:
    -A <LINT>          Allow a lint.
    -W <LINT>          Warn on a lint.
    -D <LINT>          Deny a lint: reported as an error.
    -D warnings        Fail on warnings too.
    --lua <VERSION>    5.1, 5.2, 5.3, 5.4 (default), 5.5 or luajit.
    --xmake            Use the API lists of the installed xmake.
    --fix              Apply the safe fixes (deprecated APIs, scope ends,
                       unused requires, missing requires, ...) and check
                       again.
    --list             List the lints and their default levels.

Exits with 1 when errors are found.";

/// Runs the command, returning the exit code.
#[allow(clippy::print_stdout, clippy::print_stderr)]
pub(crate) fn run(args: &[String]) -> i32 {
    let mut project = Project::new(ApiDb::builtin(), Dialect::XMAKE_DEFAULT);
    let mut levels = xmake_ide::LintLevels::default();
    let mut deny_warnings = false;
    let mut query_xmake = false;
    let mut fix = false;
    let mut roots: Vec<PathBuf> = Vec::new();
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        let level = match arg.as_str() {
            "-h" | "--help" => {
                println!("{USAGE}");
                return 0;
            }
            "--list" => {
                for lint in xmake_ide::LINTS {
                    let level = format!("{:?}", lint.default).to_lowercase();
                    println!("{:<22} {level:<5} {}", lint.name, lint.description);
                }
                return 0;
            }
            "--fix" => {
                fix = true;
                continue;
            }
            "--xmake" => {
                query_xmake = true;
                continue;
            }
            "--lua" => {
                match args.next().and_then(|v| LuaVersion::parse(v)) {
                    Some(lua) => project.dialect = Dialect { lua },
                    None => return usage_error("--lua expects 5.1 to 5.5 or luajit"),
                }
                continue;
            }
            "-A" => Level::Allow,
            "-W" => Level::Warn,
            "-D" => Level::Deny,
            other if other.starts_with('-') => {
                return usage_error(&format!("unknown option {other}"));
            }
            dir => {
                roots.push(PathBuf::from(dir));
                continue;
            }
        };
        match args.next().map(String::as_str) {
            Some("warnings") if level == Level::Deny => deny_warnings = true,
            Some(name) if levels.set(name, level) => {}
            Some(name) => return usage_error(&format!("unknown lint {name}; see --list")),
            None => return usage_error(&format!("{arg} expects a lint name")),
        }
    }
    if roots.is_empty() {
        roots.push(PathBuf::from("."));
    }
    let roots: Vec<PathBuf> =
        roots.iter().map(|r| std::path::absolute(r).unwrap_or_else(|_| r.clone())).collect();

    if query_xmake {
        match crate::probe::probe(std::path::Path::new("xmake")) {
            Ok(probe) => {
                project.xmake_version = Some(probe.version);
                project.db = probe.db;
                if probe.luajit {
                    project.dialect = Dialect { lua: LuaVersion::LuaJit };
                }
            }
            Err(err) => eprintln!("warning: {err}; using the builtin API lists"),
        }
    }
    for (path, text) in crate::scan::scan(&roots) {
        project.set_file(xmake_ide::normalize(&path), Arc::from(text));
    }
    project.update_includes();
    let globaldir = xmake_ide::default_globaldir();
    let repositories = project.local_repositories();
    project.packages = Arc::new(xmake_ide::PackageIndex::load(globaldir.as_deref(), &repositories));
    if fix && apply_fixes(&mut project, &levels) {
        // Report what is left.
        project.update_includes();
    }

    let mut files: Vec<_> = project.files().collect();
    files.sort_by(|a, b| a.0.cmp(b.0));
    let (mut errors, mut warnings) = (0, 0);
    for (path, file) in files {
        for d in xmake_ide::diagnostics(&project, file, &levels) {
            let pos = file.line_index.line_col(&file.text, d.range.start());
            let label = match d.severity {
                Severity::Error => {
                    errors += 1;
                    "error"
                }
                _ => {
                    warnings += 1;
                    "warning"
                }
            };
            println!(
                "{}:{}:{}: {label}[{}]: {}",
                path.display(),
                pos.line + 1,
                pos.col + 1,
                d.code,
                d.message
            );
        }
    }
    eprintln!("{errors} errors, {warnings} warnings");
    i32::from(errors > 0 || (deny_warnings && warnings > 0))
}

#[allow(clippy::print_stderr)]
fn usage_error(message: &str) -> i32 {
    eprintln!("error: {message}\n\n{USAGE}");
    2
}

/// Applies the safe fixes of every file and writes the changed files.
/// Returns whether any file changed.
#[allow(clippy::print_stderr)]
fn apply_fixes(project: &mut Project, levels: &xmake_ide::LintLevels) -> bool {
    let mut changed: Vec<(PathBuf, String)> = Vec::new();
    for (path, file) in project.files() {
        let diagnostics = xmake_ide::diagnostics(project, file, levels);
        let (text, applied) = xmake_ide::apply_safe_fixes(&file.text, &diagnostics);
        if applied > 0 {
            eprintln!("{}: {applied} fixes", path.display());
            changed.push((path.to_path_buf(), text));
        }
    }
    let any = !changed.is_empty();
    for (path, text) in changed {
        if let Err(err) = std::fs::write(&path, &text) {
            eprintln!("{}: {err}", path.display());
            continue;
        }
        project.set_file(path, Arc::from(text));
    }
    any
}
