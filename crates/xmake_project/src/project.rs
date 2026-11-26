//! The set of files of a project, and project-wide lookups.

use std::{
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
};

use rustc_hash::{FxHashMap, FxHashSet};
use xmake_api::{ApiDb, Scope, Version};
use xmake_syntax::{Dialect, TextRange};

use crate::{AnalyzedFile, FileKind, outline::Definition, paths};

/// Every analyzed file of the workspace plus the API database.
#[derive(Debug, Default)]
pub struct Project {
    pub db: ApiDb,
    pub dialect: Dialect,
    /// The version of the installed xmake, when known.
    pub xmake_version: Option<Version>,
    /// The packages of the repositories; empty until loaded.
    pub packages: Arc<crate::PackageIndex>,
    files: FxHashMap<PathBuf, Arc<AnalyzedFile>>,
    /// The files loaded by `includes()`: description files whatever their
    /// name.
    included: FxHashSet<PathBuf>,
    /// The projects of the files, computed on first use after a change:
    /// resolving the includes reads the disk, so not once per query.
    projects: OnceLock<Projects>,
}

/// The files connected by `includes()`, grouped.
#[derive(Debug, Default)]
struct Projects {
    /// The project of each file.
    of: FxHashMap<PathBuf, usize>,
    /// Per project: whether some `includes()` load nothing known.
    incomplete: Vec<bool>,
}

/// A definition and the file it is in.
#[derive(Clone, Copy, Debug)]
pub struct Located<'a, T> {
    pub path: &'a Path,
    pub file: &'a AnalyzedFile,
    pub item: T,
}

impl<'a> Located<'a, Definition> {
    /// The defined name.
    pub fn name(&self) -> &'a str {
        &self.file.text[self.item.name_range]
    }
}

impl<'a> Located<'a, TextRange> {
    /// The text of the range.
    pub fn text(&self) -> &'a str {
        &self.file.text[self.item]
    }
}

impl Project {
    /// Creates an empty project.
    pub fn new(db: ApiDb, dialect: Dialect) -> Project {
        Project {
            db,
            dialect,
            xmake_version: None,
            packages: Arc::default(),
            files: FxHashMap::default(),
            included: FxHashSet::default(),
            projects: OnceLock::new(),
        }
    }

    fn analyze(&self, path: &Path, text: Arc<str>) -> AnalyzedFile {
        if self.included.contains(path) {
            return AnalyzedFile::with_kind(text, FileKind::Description, self.dialect);
        }
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        AnalyzedFile::new(text, file_name, self.dialect)
    }

    /// Analyzes and stores `text` as the contents of `path`.
    pub fn set_file(&mut self, path: PathBuf, text: Arc<str>) -> Arc<AnalyzedFile> {
        let file = Arc::new(self.analyze(&path, text));
        let previous = self.files.insert(path, Arc::clone(&file));
        // Other text, same includes: the projects stay.
        let same_includes = previous.is_some_and(|p| {
            Project::include_values(&p) == Project::include_values(&file)
                && p.outline.unknown_includes.is_empty() == file.outline.unknown_includes.is_empty()
        });
        if !same_includes {
            self.projects = OnceLock::new();
        }
        file
    }

    /// The files loaded by the `includes()` of the project.
    pub fn included_files(&self) -> impl Iterator<Item = &Path> {
        self.included.iter().map(PathBuf::as_path)
    }

    /// The values of the `includes()` of `file`, to tell whether an edit
    /// changed them.
    pub fn include_values(file: &AnalyzedFile) -> Vec<&str> {
        let literal = file.outline.includes.iter().map(|&r| {
            // `"$(scriptdir)/sub"` is `"sub"`.
            let value = &file.text[r];
            value.strip_prefix("$(scriptdir)").map_or(value, |v| v.trim_start_matches(['/', '\\']))
        });
        literal.chain(file.outline.computed_includes.iter().map(String::as_str)).collect()
    }

    /// The directories of the local repositories of `add_repositories`, in
    /// every file: relative paths are relative to the script.
    pub fn local_repositories(&self) -> Vec<PathBuf> {
        let mut dirs: Vec<PathBuf> = self
            .files
            .iter()
            .flat_map(|(path, file)| {
                let dir = path.parent().unwrap_or(Path::new(""));
                file.outline.repositories.iter().map(move |repo| paths::normalize(&dir.join(repo)))
            })
            .filter(|dir| dir.is_dir())
            .collect();
        // The repositories xmake cloned for the projects (`add_repositories`
        // of a git URL): `<project>/.xmake/<plat>/<arch>/repositories/<name>`.
        let roots: Vec<&Path> = self
            .files
            .iter()
            .filter(|(_, f)| f.outline.remote_repositories)
            .filter_map(|(path, _)| path.parent())
            .collect();
        for root in roots {
            for dir in project_dirs(root) {
                dirs.extend(cloned_repositories(&dir));
            }
        }
        dirs.sort();
        dirs.dedup();
        dirs
    }

    /// Whether a file adds a repository the server may not see: a git URL,
    /// or a computed value.
    pub fn has_remote_repositories(&self) -> bool {
        self.files.values().any(|f| f.outline.remote_repositories)
    }

    /// The modules of `file`: `"utils"` of `import("utils")`.
    pub fn import_values(file: &AnalyzedFile) -> Vec<&str> {
        use xmake_syntax::ast::{AstNode, CallExpr, Literal};
        file.root()
            .descendants()
            .filter_map(CallExpr::cast)
            .filter(|c| c.callee_name() == Some("import"))
            .filter_map(|c| Literal::cast(c.arg(0)?)?.token())
            .filter(|t| t.kind() == xmake_syntax::SyntaxKind::STRING)
            .map(|t| &file.text[crate::outline::content_range(t)])
            .collect()
    }

    /// The module files of the project imported by its files (`import("utils")`
    /// next to the script), sorted.
    pub fn imported_modules(&self) -> Vec<PathBuf> {
        let mut out: Vec<PathBuf> = self
            .files
            .iter()
            .filter_map(|(path, file)| Some((path.parent()?, file)))
            .flat_map(|(dir, file)| {
                Project::import_values(file)
                    .into_iter()
                    .filter_map(move |module| paths::resolve_module(dir, module))
            })
            .collect();
        out.sort();
        out.dedup();
        out
    }

    /// Follows the `includes()` of every file: loads the included files from
    /// the disk, analyzes them as description files, and again for what
    /// they include. Returns whether the set of included files changed.
    pub fn update_includes(&mut self) -> bool {
        let mut changed = false;
        // Each round may load files including others: stop when stable.
        for _ in 0..32 {
            let mut targets = FxHashSet::default();
            for (path, file) in &self.files {
                let Some(dir) = path.parent() else {
                    continue;
                };
                for value in Project::include_values(file) {
                    targets.extend(paths::resolve_include(dir, value));
                }
            }
            if targets == self.included {
                break;
            }
            changed = true;
            let previous = std::mem::replace(&mut self.included, targets);
            // Files no longer included get their own kind back; newly
            // included ones become description files.
            let affected: Vec<PathBuf> =
                previous.symmetric_difference(&self.included).cloned().collect();
            for path in affected {
                let text = match self.files.get(&path) {
                    Some(file) => Arc::clone(&file.text),
                    None => match std::fs::read_to_string(&path) {
                        Ok(text) => Arc::from(text),
                        Err(_) => continue,
                    },
                };
                self.set_file(path, text);
            }
        }
        changed
    }

    /// Forgets `path`.
    pub fn remove_file(&mut self, path: &Path) {
        self.files.remove(path);
        self.projects = OnceLock::new();
    }

    /// The analysis of `path`.
    pub fn file(&self, path: &Path) -> Option<&Arc<AnalyzedFile>> {
        self.files.get(path)
    }

    /// Every file.
    pub fn files(&self) -> impl Iterator<Item = (&Path, &Arc<AnalyzedFile>)> {
        self.files.iter().map(|(p, f)| (p.as_path(), f))
    }

    /// Re-analyzes every file, e.g. after the dialect changed.
    pub fn reanalyze(&mut self) {
        let reanalyzed: Vec<_> = self
            .files
            .iter()
            .map(|(path, file)| {
                (path.clone(), Arc::new(self.analyze(path, Arc::clone(&file.text))))
            })
            .collect();
        for (path, file) in reanalyzed {
            self.files.insert(path, file);
        }
        self.projects = OnceLock::new();
    }

    /// Every definition of `scope` (targets, options, ...) in the project.
    pub fn definitions(&self, scope: Scope) -> impl Iterator<Item = Located<'_, Definition>> {
        self.files().flat_map(move |(path, file)| {
            let defs = file.outline.definitions.iter().filter(move |d| d.scope == scope);
            defs.map(move |&item| Located { path, file, item })
        })
    }

    /// The package names required with `add_requires`.
    pub fn requires(&self) -> impl Iterator<Item = Located<'_, TextRange>> {
        self.files().flat_map(|(path, file)| {
            file.outline.requires.iter().map(move |&item| Located { path, file, item })
        })
    }

    /// The path `file` is stored at.
    pub fn path_of(&self, file: &AnalyzedFile) -> Option<&Path> {
        self.files.iter().find(|(_, f)| std::ptr::eq(f.as_ref(), file)).map(|(p, _)| p.as_path())
    }

    /// The project `file` belongs to: the files connected to it by
    /// `includes()`, in either direction. Names (targets, options, ...) are
    /// looked up there, not in other projects of the workspace.
    pub fn scope_of<'a>(&'a self, file: &AnalyzedFile) -> ProjectScope<'a> {
        let Some(start) =
            self.files.iter().find(|(_, f)| std::ptr::eq(f.as_ref(), file)).map(|(p, _)| p)
        else {
            return ProjectScope { members: Vec::new(), incomplete: false };
        };
        let projects = self.projects.get_or_init(|| self.group_projects());
        let Some(&id) = projects.of.get(start) else {
            return ProjectScope { members: Vec::new(), incomplete: false };
        };
        let members = self
            .files
            .iter()
            .filter(|(p, _)| projects.of.get(*p) == Some(&id))
            .map(|(p, f)| (p.as_path(), f))
            .collect();
        ProjectScope { members, incomplete: projects.incomplete[id] }
    }

    /// The files grouped by the `includes()` connecting them.
    fn group_projects(&self) -> Projects {
        // Undirected edges between a file and the files it includes.
        let mut neighbors: FxHashMap<&Path, Vec<&Path>> = FxHashMap::default();
        // Files whose includes are not all known.
        let mut partial: FxHashSet<&Path> = FxHashSet::default();
        for (path, other) in &self.files {
            let Some(dir) = path.parent() else {
                continue;
            };
            if !other.outline.unknown_includes.is_empty() {
                partial.insert(path.as_path());
            }
            for value in Project::include_values(other) {
                let targets = self.included_paths(dir, value);
                if targets.is_empty() && !value.starts_with('@') && !value.contains('*') {
                    partial.insert(path.as_path());
                }
                for target in targets {
                    neighbors.entry(path.as_path()).or_default().push(target);
                    neighbors.entry(target).or_default().push(path.as_path());
                }
            }
        }
        let mut projects = Projects::default();
        for start in self.files.keys() {
            if projects.of.contains_key(start) {
                continue;
            }
            let id = projects.incomplete.len();
            let mut incomplete = false;
            let mut stack = vec![start.as_path()];
            while let Some(path) = stack.pop() {
                if projects.of.contains_key(path) {
                    continue;
                }
                projects.of.insert(path.to_path_buf(), id);
                incomplete |= partial.contains(path);
                stack.extend(neighbors.get(path).into_iter().flatten().copied());
            }
            projects.incomplete.push(incomplete);
        }
        projects
    }

    /// The known files `includes(value)` of a file in `dir` loads.
    pub fn included_paths<'a>(&'a self, dir: &Path, value: &str) -> Vec<&'a Path> {
        let mut out: Vec<&'a Path> = Vec::new();
        let mut push = |path: PathBuf| {
            if let Some((known, _)) = self.files.get_key_value(&path) {
                out.push(known.as_path());
            }
        };
        if !value.contains('*') {
            // Known files first: they need not exist on the disk.
            let path = paths::normalize(&dir.join(value));
            push(path.join("xmake.lua"));
            push(path);
        }
        for path in paths::resolve_include(dir, value) {
            push(path);
        }
        out.sort_unstable();
        out.dedup();
        out
    }
}

/// The files of one project, see [`Project::scope_of`].
#[derive(Clone, Debug, Default)]
pub struct ProjectScope<'a> {
    members: Vec<(&'a Path, &'a Arc<AnalyzedFile>)>,
    /// Some `includes()` of the project load nothing known (a missing
    /// directory, a computed path): names may come from there, so they are
    /// not reported as unknown.
    pub incomplete: bool,
}

impl<'a> ProjectScope<'a> {
    /// The files of the project.
    pub fn files(&self) -> impl Iterator<Item = (&'a Path, &'a Arc<AnalyzedFile>)> + '_ {
        self.members.iter().copied()
    }

    /// The definitions of `scope` (targets, options, ...) in the project.
    pub fn definitions(&self, scope: Scope) -> impl Iterator<Item = Located<'a, Definition>> + '_ {
        self.files().flat_map(move |(path, file)| {
            let defs = file.outline.definitions.iter().filter(move |d| d.scope == scope);
            defs.map(move |&item| Located { path, file, item })
        })
    }

    /// The package names required with `add_requires` in the project.
    pub fn requires(&self) -> impl Iterator<Item = Located<'a, TextRange>> + '_ {
        self.files().flat_map(|(path, file)| {
            file.outline.requires.iter().map(move |&item| Located { path, file, item })
        })
    }
}

/// `dir` and its ancestors: where the `.xmake` of the project may be.
fn project_dirs(dir: &Path) -> impl Iterator<Item = PathBuf> + '_ {
    dir.ancestors().take(4).map(Path::to_path_buf)
}

/// The repositories under `<dir>/.xmake/<plat>/<arch>/repositories/`.
fn cloned_repositories(dir: &Path) -> Vec<PathBuf> {
    let read = |d: &Path| -> Vec<PathBuf> {
        std::fs::read_dir(d)
            .map(|e| e.flatten().map(|e| e.path()).filter(|p| p.is_dir()).collect())
            .unwrap_or_default()
    };
    let mut out = Vec::new();
    for plat in read(&dir.join(".xmake")) {
        for arch in read(&plat) {
            out.extend(
                read(&arch.join("repositories"))
                    .into_iter()
                    .filter(|r| r.join("packages").is_dir()),
            );
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use xmake_api::ApiDb;

    use super::*;

    /// `add_repositories` of a git URL: the clone of xmake under `.xmake`.
    #[test]
    fn cloned_repositories() {
        let dir = std::env::temp_dir().join(format!("xmake-project-clone-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let repo = dir.join(".xmake/windows/x64/repositories/my_repo");
        std::fs::create_dir_all(repo.join("packages/z/zlib")).expect("dirs");
        let mut project = Project::new(ApiDb::builtin(), Dialect::XMAKE_DEFAULT);
        let text =
            "add_repositories('my_repo ' .. (os.getenv('X') or 'https://example.com/r.git'))\n";
        project.set_file(dir.join("xmake.lua"), Arc::from(text));
        assert!(project.has_remote_repositories());
        assert_eq!(project.local_repositories(), [repo]);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
