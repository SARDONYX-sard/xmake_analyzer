//! The modules of the project: `import("utils")` loads `utils.lua` (or
//! `utils/main.lua`) next to the script; its global functions, but those
//! starting with `_`, are its members.

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use xmake_syntax::{SyntaxKind, SyntaxNode, TextRange};

use crate::{AnalyzedFile, Project};

/// A module file of the project.
pub(crate) struct LocalModule {
    pub path: PathBuf,
    pub file: Arc<AnalyzedFile>,
}

/// The path of `file` in `project`.
pub(crate) fn path_of<'p>(project: &'p Project, file: &AnalyzedFile) -> Option<&'p Path> {
    project.files().find(|(_, f)| std::ptr::eq(f.as_ref(), file)).map(|(p, _)| p)
}

/// The file of `module` for a script in `dir`: `a.b` is `a/b.lua`,
/// `a/b/main.lua` or `a/b/xmake.lua`.
pub(crate) fn module_path(dir: &Path, module: &str) -> Option<PathBuf> {
    let relative: PathBuf = module.split('.').collect();
    let module = dir.join(relative);
    [module.with_extension("lua"), module.join("main.lua"), module.join("xmake.lua")]
        .into_iter()
        .find(|c| c.is_file())
}

/// The module `module` imported by `file`, read from the project or the disk.
pub(crate) fn local_module(
    project: &Project,
    file: &AnalyzedFile,
    module: &str,
) -> Option<LocalModule> {
    let dir = path_of(project, file)?.parent()?;
    let path = crate::paths::normalize(&module_path(dir, module)?);
    let file = match project.file(&path) {
        Some(file) => Arc::clone(file),
        None => {
            let text = std::fs::read_to_string(&path).ok()?;
            let name = path.file_name()?.to_string_lossy();
            Arc::new(AnalyzedFile::new(Arc::from(text), &name, project.dialect))
        }
    };
    Some(LocalModule { path, file })
}

/// The members of a module file: its global functions, but `_private` ones.
/// The function statements and their names.
pub(crate) fn exported_functions(file: &AnalyzedFile) -> Vec<(SyntaxNode<'_>, &str, TextRange)> {
    let Some(block) = file.root().child_node(SyntaxKind::BLOCK) else {
        return Vec::new();
    };
    block
        .children()
        .filter(|s| s.kind() == SyntaxKind::FUNCTION_STMT)
        .filter_map(|statement| {
            let name = xmake_types::global_function_name(statement)?;
            let range = statement.child_node(SyntaxKind::FUNC_NAME)?.trimmed_range();
            (!name.starts_with('_')).then_some((statement, name, range))
        })
        .collect()
}
