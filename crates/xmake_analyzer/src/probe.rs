//! Asking the installed xmake for its version, Lua runtime and API lists.
//!
//! Running xmake takes about a second per command, so the outputs are cached
//! on disk per xmake build: later starts only run `xmake --version`.

use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use xmake_ide::xmake_api::{ApiDb, ListKind, Version};

/// What the installed xmake told.
pub(crate) struct Probe {
    pub(crate) version: Version,
    pub(crate) luajit: bool,
    /// The builtin database updated with the lists of this xmake.
    pub(crate) db: ApiDb,
}

/// Prints the Lua runtime of xmake.
const RUNTIME_SCRIPT: &str = r#"function main()
    print(xmake.luajit and xmake.luajit() and "luajit" or "lua")
end
"#;

/// Runs xmake, or returns why it failed.
pub(crate) fn probe(xmake: &Path) -> Result<Probe, String> {
    let work_dir = std::env::temp_dir().join("xmake_analyzer_probe");
    std::fs::create_dir_all(&work_dir)
        .map_err(|e| format!("cannot create {}: {e}", work_dir.display()))?;

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
    let apis =
        cached("apis.json", &|| run_json(xmake, &["show", "-l", "apis", "--json"], &work_dir))?;
    if let Err(err) = db.apply_show_apis(&apis) {
        log::warn!("xmake show -l apis: {err}");
    }
    for kind in ListKind::ALL {
        let name = kind.show_name();
        let json = cached(&format!("{name}.json"), &|| {
            run_json(xmake, &["show", "-l", name, "--json"], &work_dir)
        });
        match json.map(|json| db.apply_list(kind, &json).map_err(|e| e.to_string())) {
            Ok(Ok(())) => {}
            Ok(Err(err)) | Err(err) => log::warn!("xmake show -l {name}: {err}"),
        }
    }
    Ok(Probe { version, luajit, db })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitized_names() {
        assert_eq!(sanitize("v3.1.1+HEAD.3ba37a0d4"), "v3.1.1_HEAD.3ba37a0d4");
    }
}
