//! Asking the installed xmake for its version, Lua runtime and API lists,
//! and writing the stubs of that xmake (see [`xmake_ide::xmake_api::Stubs`]).
//!
//! Running xmake takes about a second per command, so the outputs are cached
//! on disk per xmake build, with the stubs written for it: later starts only
//! run `xmake --version`.

use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use xmake_ide::xmake_api::{ApiDb, ListKind, Stubs, Version, render};

/// What the installed xmake told.
pub(crate) struct Probe {
    pub(crate) version: Version,
    pub(crate) luajit: bool,
    /// The database of the stubs written for this xmake, now in use.
    pub(crate) db: ApiDb,
}

/// The lists of an xmake.
pub(crate) struct Lists {
    pub(crate) version: Version,
    /// `v3.1.2+HEAD.5d21e9daf`
    pub(crate) build: String,
    pub(crate) luajit: bool,
    /// The builtin database updated with the lists of this xmake.
    pub(crate) db: ApiDb,
}

/// Prints the description APIs of each scope with their kinds.
const DUMP_SCOPES_SCRIPT: &str = include_str!("dump_scopes.lua");

/// Prints the Lua runtime of xmake.
const RUNTIME_SCRIPT: &str = r#"function main()
    print(xmake.luajit and xmake.luajit() and "luajit" or "lua")
end
"#;

/// Runs xmake, writes its stubs (once per xmake build and version of this
/// server) and uses them, or returns why it failed. `progress` hears of each
/// step: generating takes some seconds.
pub(crate) fn probe(xmake: &Path, progress: &dyn Fn(&str)) -> Result<Probe, String> {
    let lists = lists(xmake, progress)?;
    let dir = cache_dir().map(|d| {
        d.join(sanitize(&lists.build))
            .join(format!("stubs-{}", env!("CARGO_PKG_VERSION")))
            .join(lists.version.to_string())
    });
    let stubs = dir.and_then(|dir| match stubs_in(&dir, &lists.db, progress) {
        Ok(stubs) => Some(stubs),
        Err(err) => {
            log::warn!(
                "cannot write the stubs of xmake {} to {}: {err}",
                lists.build,
                dir.display()
            );
            None
        }
    });
    let db = match stubs {
        Some(stubs) => {
            let dir = stubs.dir().map(|d| d.display().to_string()).unwrap_or_default();
            log::info!("stubs of xmake {} in {dir}", lists.build);
            ApiDb::from_stubs(Stubs::install(stubs))
        }
        // No stubs on the disk: the lists still hold.
        None => lists.db,
    };
    Ok(Probe { version: lists.version, luajit: lists.luajit, db })
}

/// The stubs of `db` in `dir`, written there first when missing.
fn stubs_in(dir: &Path, db: &ApiDb, progress: &dyn Fn(&str)) -> std::io::Result<Stubs> {
    if !dir.join(render::VALUES_FILE).is_file() {
        progress("writing the stubs");
        write_stubs(dir, &render::update(Stubs::builtin(), db, None))?;
    }
    Stubs::read_dir(dir)
}

/// Writes `files` (`(path, text)`, `path` relative with `/`) to `dir`: the
/// files are written first, then the `*.lua` files no longer there removed,
/// so that a failure leaves every file.
pub(crate) fn write_stubs(dir: &Path, files: &[(String, String)]) -> std::io::Result<()> {
    let mut written = rustc_hash::FxHashSet::default();
    for (path, text) in files {
        let path = dir.join(path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, text)?;
        written.insert(path);
    }
    remove_stale(dir, &written)
}

/// Removes the `*.lua` files under `dir` not in `keep`, and the directories
/// left empty.
fn remove_stale(dir: &Path, keep: &rustc_hash::FxHashSet<PathBuf>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)?.flatten() {
        let path = entry.path();
        if path.is_dir() {
            remove_stale(&path, keep)?;
            if std::fs::read_dir(&path)?.next().is_none() {
                std::fs::remove_dir(&path)?;
            }
        } else if path.extension().is_some_and(|e| e == "lua") && !keep.contains(&path) {
            std::fs::remove_file(&path)?;
        }
    }
    Ok(())
}

/// Deletes the cache: the outputs of xmake and the stubs written for it.
pub(crate) fn clear_cache() -> std::io::Result<Option<PathBuf>> {
    let Some(dir) = cache_dir() else {
        return Ok(None);
    };
    if dir.exists() {
        std::fs::remove_dir_all(&dir)?;
    }
    Ok(Some(dir))
}

/// Runs xmake for its lists, or returns why it failed.
pub(crate) fn lists(xmake: &Path, progress: &dyn Fn(&str)) -> Result<Lists, String> {
    let work_dir = std::env::temp_dir().join("xmake_analyzer_probe");
    std::fs::create_dir_all(&work_dir)
        .map_err(|e| format!("cannot create {}: {e}", work_dir.display()))?;

    progress("xmake --version");
    let banner = run(xmake, ["--version"], &work_dir)?;
    let build =
        banner.split_whitespace().find(|w| w.starts_with('v')).map(|w| w.trim_end_matches(','));
    let (Some(build), Some(version)) = (build, Version::from_xmake_banner(&banner)) else {
        return Err(format!("unexpected `xmake --version` output: {banner}"));
    };
    log::info!("found xmake {build}");

    let cache = cache_dir().map(|dir| dir.join(sanitize(build)));
    let cached =
        |name: &str, compute: &dyn Fn() -> Result<String, String>| -> Result<String, String> {
            let file = cache.as_ref().map(|dir| dir.join(name));
            if let Some(text) = file.as_ref().and_then(|f| std::fs::read_to_string(f).ok()) {
                return Ok(text);
            }
            let text = compute()?;
            if let (Some(dir), Some(file)) = (&cache, &file) {
                let saved = std::fs::create_dir_all(dir).and_then(|()| std::fs::write(file, &text));
                if let Err(err) = saved {
                    log::warn!("cannot cache {}: {err}", file.display());
                }
            }
            Ok(text)
        };

    let runtime = cached("runtime.txt", &|| {
        let script = work_dir.join("runtime.lua");
        std::fs::write(&script, RUNTIME_SCRIPT).map_err(|e| e.to_string())?;
        run(xmake, [OsStr::new("l"), script.as_os_str()], &work_dir)
    })?;
    let luajit = runtime.lines().any(|l| l.trim() == "luajit");

    let mut db = ApiDb::builtin();
    db.version = Some(version);
    let apis = cached("apis.json", &|| {
        progress("xmake show -l apis");
        run_json(xmake, &["show", "-l", "apis", "--json"], &work_dir)
    })?;
    if let Err(err) = db.apply_show_apis(&apis) {
        log::warn!("xmake show -l apis: {err}");
    }
    let scopes = cached("scopes.json", &|| {
        progress("the description scopes");
        let script = work_dir.join("dump_scopes.lua");
        std::fs::write(&script, DUMP_SCOPES_SCRIPT).map_err(|e| e.to_string())?;
        let mut out = run(xmake, [OsStr::new("l"), script.as_os_str()], &work_dir)?;
        let start = out.find('{').ok_or("no JSON from dump_scopes.lua")?;
        out.drain(..start);
        Ok(out)
    });
    match scopes.map(|json| db.apply_scope_kinds(&json).map_err(|e| e.to_string())) {
        Ok(Ok(())) => {}
        Ok(Err(err)) | Err(err) => log::warn!("the kinds of the scope APIs: {err}"),
    }
    for kind in ListKind::ALL {
        let name = kind.show_name();
        let json = cached(&format!("{name}.json"), &|| {
            progress(&format!("xmake show -l {name}"));
            run_json(xmake, &["show", "-l", name, "--json"], &work_dir)
        });
        match json.map(|json| db.apply_list(kind, &json).map_err(|e| e.to_string())) {
            Ok(Ok(())) => {}
            Ok(Err(err)) | Err(err) => log::warn!("xmake show -l {name}: {err}"),
        }
    }
    Ok(Lists { version, build: build.to_owned(), luajit, db })
}

/// Runs `xmake args` and returns its standard output.
fn run<I, S>(xmake: &Path, args: I, cwd: &Path) -> Result<String, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = Command::new(xmake);
    command
        .args(args)
        .current_dir(cwd)
        .env("COLORTERM", "nocolor")
        .env("XMAKE_THEME", "plain")
        .stdin(Stdio::null())
        .stderr(Stdio::null());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt as _;
        // Do not flash a console window.
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        command.creation_flags(CREATE_NO_WINDOW);
    }
    let output = command.output().map_err(|e| format!("cannot run {}: {e}", xmake.display()))?;
    if !output.status.success() {
        return Err(format!("{} exited with {}", xmake.display(), output.status));
    }
    String::from_utf8(output.stdout).map_err(|_| "xmake printed invalid UTF-8".to_owned())
}

/// Runs xmake and keeps its output from the first JSON delimiter on: xmake
/// may print warnings first.
fn run_json(xmake: &Path, args: &[&str], cwd: &Path) -> Result<String, String> {
    let mut out = run(xmake, args, cwd)?;
    let start =
        out.find(['{', '[']).ok_or_else(|| format!("no JSON in `xmake {}`", args.join(" ")))?;
    out.drain(..start);
    Ok(out)
}

/// Where the outputs of xmake are cached.
fn cache_dir() -> Option<PathBuf> {
    let base = if cfg!(windows) {
        std::env::var_os("LOCALAPPDATA").map(PathBuf::from)
    } else {
        let xdg = std::env::var_os("XDG_CACHE_HOME").map(PathBuf::from);
        xdg.or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".cache")))
    };
    base.map(|b| b.join("xmake_analyzer").join("cache"))
}

/// A build string usable as a directory name.
fn sanitize(build: &str) -> String {
    build
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '.' || c == '-' { c } else { '_' })
        .collect()
}

const STUBS_USAGE: &str =
    "xmake_analyzer stubs: writes the stubs of an xmake, from the embedded ones.

USAGE:
    xmake_analyzer stubs [--xmake PATH] [--manual FILE] [--out DIR]
    xmake_analyzer stubs --clean
    xmake_analyzer stubs --rewrite

    --xmake PATH   The xmake to read the lists from (default: xmake).
    --manual FILE  The sections of the manual, from `tools/gen_doc_links.py`:
                   replaces the prototypes and links of the stubs.
    --out DIR      Where to write them (default: crates/xmake_api/stubs/<version>).
    --clean        Delete the cache of the server: the outputs of xmake and the
                   stubs written for each xmake.
    --rewrite      Write the shipped stubs again for their own lists, without
                   xmake: after editing them by hand.

The documentation and types of the shipped stubs are kept; the functions the
xmake lists are added or tagged, those it lacks removed.";

/// `xmake_analyzer stubs`: returns the exit code.
#[allow(clippy::print_stdout, clippy::print_stderr)]
pub(crate) fn run_stubs_command(args: &[String]) -> i32 {
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("{STUBS_USAGE}");
        return 0;
    }
    if args.iter().any(|a| a == "--clean") {
        return match clear_cache() {
            Ok(Some(dir)) => {
                println!("deleted {}", dir.display());
                0
            }
            Ok(None) => 0,
            Err(err) => {
                eprintln!("error: cannot delete the cache: {err}");
                1
            }
        };
    }
    let option = |name: &str| args.iter().position(|a| a == name).and_then(|i| args.get(i + 1));
    if args.iter().any(|a| a == "--rewrite") {
        let stubs = Stubs::builtin();
        let Some(dir) = stubs.dir() else {
            eprintln!("error: no stubs found");
            return 1;
        };
        let out = option("--out").map_or_else(|| dir.to_owned(), PathBuf::from);
        let files = render::update(stubs, &ApiDb::from_stubs(stubs), None);
        if let Err(err) = write_stubs(&out, &files) {
            eprintln!("error: cannot write {}: {err}", out.display());
            return 1;
        }
        println!("wrote the stubs to {}", out.display());
        return 0;
    }
    let xmake = option("--xmake").map_or_else(|| PathBuf::from("xmake"), PathBuf::from);
    let lists = match lists(&xmake, &|step| eprintln!("{step}")) {
        Ok(lists) => lists,
        Err(err) => {
            eprintln!("error: {err}");
            return 1;
        }
    };
    let manual = match option("--manual").map(std::fs::read_to_string) {
        None => None,
        Some(Ok(json)) => match render::Manual::from_json(&json) {
            Ok(manual) => Some(manual),
            Err(err) => {
                eprintln!("error: invalid manual: {err}");
                return 1;
            }
        },
        Some(Err(err)) => {
            eprintln!("error: cannot read the manual: {err}");
            return 1;
        }
    };
    let out = option("--out").map_or_else(
        || PathBuf::from("crates/xmake_api/stubs").join(lists.version.to_string()),
        PathBuf::from,
    );
    let files = render::update(Stubs::builtin(), &lists.db, manual.as_ref());
    if let Err(err) = write_stubs(&out, &files) {
        eprintln!("error: cannot write {}: {err}", out.display());
        return 1;
    }
    println!("wrote the stubs of xmake {} to {}", lists.build, out.display());
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitized_names() {
        assert_eq!(sanitize("v3.1.1+HEAD.3ba37a0d4"), "v3.1.1_HEAD.3ba37a0d4");
    }
}
