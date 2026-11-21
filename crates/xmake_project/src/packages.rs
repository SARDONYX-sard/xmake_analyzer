//! The package repositories: names, versions and configurations of the
//! packages `add_requires` can require.
//!
//! xmake clones its repositories (`xmake-repo`, those of `add_repositories`)
//! under `<globaldir>/repositories/<repo>/packages/<letter>/<name>/xmake.lua`.
//! Reading them directly is fast and works offline: the index only lists the
//! names; the details of one package are parsed when asked.

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use xmake_syntax::{
    Dialect,
    ast::{AstNode, CallExpr, Literal, TableExpr},
};

use crate::AnalyzedFile;

/// A package of a repository.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackageEntry {
    pub name: String,
    /// The repository: `xmake-repo`, ...
    pub repository: String,
    /// Its `xmake.lua`.
    pub path: PathBuf,
}

/// What a package definition declares.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PackageDetails {
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub license: Option<String>,
    /// As written in `add_versions`, oldest first as in most files.
    pub versions: Vec<String>,
    /// `add_configs(name, {description = ...})`.
    pub configs: Vec<(String, String)>,
}

/// The packages of the repositories, sorted by name.
#[derive(Clone, Debug, Default)]
pub struct PackageIndex {
    entries: Vec<PackageEntry>,
}

impl PackageIndex {
    /// Indexes the repositories under `globaldir/repositories` and the
    /// extra repository directories (`add_repositories` of the project).
    pub fn load(globaldir: Option<&Path>, extra: &[PathBuf]) -> PackageIndex {
        let mut entries = Vec::new();
        for dir in extra {
            let name =
                dir.file_name().map_or_else(String::new, |n| n.to_string_lossy().into_owned());
            index_repository(dir, &name, &mut entries);
        }
        if let Some(root) = globaldir.map(|g| g.join("repositories"))
            && let Ok(repos) = std::fs::read_dir(&root)
        {
            for repo in repos.flatten() {
                let name = repo.file_name().to_string_lossy().into_owned();
                index_repository(&repo.path(), &name, &mut entries);
            }
        }
        // Stable: the project repositories, indexed first, stay first.
        entries.sort_by(|a, b| a.name.cmp(&b.name));
        // A package of the project repository overrides the official one.
        entries.dedup_by(|later, earlier| later.name == earlier.name);
        PackageIndex { entries }
    }

    /// Whether no repository was found: checks are then skipped.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Every package.
    pub fn entries(&self) -> &[PackageEntry] {
        &self.entries
    }

    /// The package `name`.
    pub fn get(&self, name: &str) -> Option<&PackageEntry> {
        let index = self.entries.binary_search_by(|e| e.name.as_str().cmp(name)).ok()?;
        Some(&self.entries[index])
    }
}

/// `<repo>/packages/<letter>/<name>/xmake.lua`.
fn index_repository(repo: &Path, repository: &str, out: &mut Vec<PackageEntry>) {
    let Ok(letters) = std::fs::read_dir(repo.join("packages")) else {
        return;
    };
    for letter in letters.flatten().filter(|e| e.file_type().is_ok_and(|t| t.is_dir())) {
        let Ok(packages) = std::fs::read_dir(letter.path()) else {
            continue;
        };
        for package in packages.flatten() {
            let path = package.path().join("xmake.lua");
            if path.is_file() {
                let name = package.file_name().to_string_lossy().into_owned();
                out.push(PackageEntry { name, repository: repository.to_owned(), path });
            }
        }
    }
}

impl PackageEntry {
    /// Parses the definition of the package.
    pub fn details(&self) -> Option<PackageDetails> {
        let text = std::fs::read_to_string(&self.path).ok()?;
        Some(details_of(&text))
    }
}

/// The declarations of a `package(...)` definition.
pub fn details_of(text: &str) -> PackageDetails {
    let file = AnalyzedFile::with_kind(
        Arc::from(text),
        crate::FileKind::Description,
        Dialect::XMAKE_DEFAULT,
    );
    let mut details = PackageDetails::default();
    for call in file.root().descendants().filter_map(CallExpr::cast) {
        let Some(name) = call.callee_name() else {
            continue;
        };
        let first = || {
            call.arg(0)
                .and_then(Literal::cast)
                .and_then(|l| l.string_value())
                .map(|v| v.into_owned())
        };
        match name {
            "set_description" => details.description = first(),
            "set_homepage" => details.homepage = first(),
            "set_license" => details.license = first(),
            "add_versions" => details.versions.extend(first()),
            "add_configs" => {
                let Some(config) = first() else {
                    continue;
                };
                let table = call.arg(1).and_then(TableExpr::cast);
                let description = table
                    .and_then(|t| {
                        t.fields().find(|f| f.key_name().is_some_and(|k| k.text() == "description"))
                    })
                    .and_then(|f| f.value())
                    .and_then(Literal::cast)
                    .and_then(|l| l.string_value())
                    .map(|v| v.into_owned())
                    .unwrap_or_default();
                details.configs.push((config, description));
            }
            _ => {}
        }
    }
    details
}

/// The default global directory of xmake: `$XMAKE_GLOBALDIR`, else
/// `~/.xmake` (`%USERPROFILE%\AppData\Local\.xmake` on Windows).
pub fn default_globaldir() -> Option<PathBuf> {
    if let Some(dir) = std::env::var_os("XMAKE_GLOBALDIR") {
        return Some(PathBuf::from(dir).join(".xmake"));
    }
    if cfg!(windows) {
        let local = std::env::var_os("LOCALAPPDATA").map(PathBuf::from);
        local.map(|l| l.join(".xmake"))
    } else {
        std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".xmake"))
    }
}

/// Splits `"zlib v1.3"`, `"conan::zlib/1.3"`, `"zlib >=1.2 <2"` into the
/// package name and the version requirement.
pub fn split_requirement(value: &str) -> (&str, &str) {
    let value = value.trim();
    let (name, rest) = value.split_once(char::is_whitespace).unwrap_or((value, ""));
    (name, rest.trim())
}

/// Whether a version requirement names one exact version (`1.2.3`,
/// `v1.3`), rather than a range, a branch or a commit.
pub fn is_exact_version(requirement: &str) -> bool {
    let digits = requirement.strip_prefix('v').unwrap_or(requirement);
    !digits.is_empty()
        && digits
            .split('.')
            .all(|part| !part.is_empty() && part.bytes().all(|b| b.is_ascii_digit()))
}

/// Whether `version` is in `versions`, ignoring a `v` prefix.
pub fn has_version(versions: &[String], version: &str) -> bool {
    let bare = |v: &str| v.strip_prefix('v').unwrap_or(v).to_owned();
    let wanted = bare(version);
    versions.iter().any(|v| bare(v) == wanted)
}

/// The keys of the options table of `add_requires`.
pub const REQUIRE_OPTIONS: &[(&str, &str)] = &[
    ("configs", "The configurations of the package: `{shared = true}`."),
    ("optional", "Do not fail when the package cannot be installed."),
    ("system", "`true`: only the system package; `false`: never the system one."),
    ("alias", "The name `add_packages` uses."),
    ("debug", "Build the debug version."),
    ("private", "Do not pass the package on to the dependents."),
    ("verify", "Verify the checksum of the sources."),
    ("external", "Use `-isystem` for the include directories."),
    ("build", "Always build the package from sources."),
    ("host", "Build the package for the host, e.g. a tool."),
    ("version", "The version, instead of writing it after the name."),
];

/// Configurations every package understands.
pub const COMMON_CONFIGS: &[(&str, &str)] = &[
    ("shared", "Build a shared library."),
    ("debug", "Build the debug version."),
    ("pic", "Build position independent code."),
    ("lto", "Enable link time optimization."),
    ("asan", "Enable the address sanitizer."),
    ("runtimes", "The C/C++ runtimes: `MD`, `MT`, `c++_shared`, ..."),
    ("toolchains", "The toolchains to build the package with."),
    ("cflags", "Extra C flags."),
    ("cxflags", "Extra C and C++ flags."),
    ("cxxflags", "Extra C++ flags."),
    ("ldflags", "Extra link flags."),
];

/// Compares `v1.2.10` and `1.3` numerically.
pub fn compare_versions(a: &str, b: &str) -> std::cmp::Ordering {
    let parts = |v: &str| -> Vec<u64> {
        v.trim_start_matches('v').split(['.', '-', '+']).map(|p| p.parse().unwrap_or(0)).collect()
    };
    parts(a).cmp(&parts(b))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn details() {
        let text = "package('zlib')\n    set_homepage('http://www.zlib.net')\n    set_description('Compression')\n    add_versions('v1.3', 'abc')\n    add_versions('v1.2.13', 'def')\n    add_configs('zutil', {description = 'Export zutil.h', default = false})\n";
        let d = details_of(text);
        assert_eq!(d.description.as_deref(), Some("Compression"));
        assert_eq!(d.versions, ["v1.3", "v1.2.13"]);
        assert_eq!(d.configs, [("zutil".to_owned(), "Export zutil.h".to_owned())]);
    }

    #[test]
    fn requirements() {
        assert_eq!(split_requirement("zlib v1.3"), ("zlib", "v1.3"));
        assert_eq!(split_requirement("fmt >=10 <11"), ("fmt", ">=10 <11"));
        assert!(is_exact_version("1.2.3") && is_exact_version("v1.3"));
        assert!(
            !is_exact_version("1.2.x") && !is_exact_version(">=1.0") && !is_exact_version("master")
        );
        let versions = ["v1.3".to_owned(), "1.2.13".to_owned()];
        assert!(
            has_version(&versions, "1.3")
                && has_version(&versions, "v1.2.13")
                && !has_version(&versions, "1.4")
        );
    }

    #[test]
    fn indexes_a_repository() {
        let dir = std::env::temp_dir().join(format!("xmake-ide-repo-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let package = dir.join("repositories/xmake-repo/packages/z/zlib");
        std::fs::create_dir_all(&package).expect("dirs");
        std::fs::write(package.join("xmake.lua"), "package('zlib')\n add_versions('v1.3', 'x')\n")
            .expect("file");
        let index = PackageIndex::load(Some(&dir), &[]);
        let zlib = index.get("zlib").expect("zlib");
        assert_eq!(zlib.repository, "xmake-repo");
        assert_eq!(zlib.details().map(|d| d.versions), Some(vec!["v1.3".to_owned()]));

        // A package of a local repository (`add_repositories`) overrides it.
        let local = dir.join("local-repo");
        let package = local.join("packages/z/zlib");
        std::fs::create_dir_all(&package).expect("dirs");
        std::fs::write(
            package.join("xmake.lua"),
            "package('zlib')
",
        )
        .expect("file");
        let index = PackageIndex::load(Some(&dir), &[local]);
        assert_eq!(index.get("zlib").expect("zlib").repository, "local-repo");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
