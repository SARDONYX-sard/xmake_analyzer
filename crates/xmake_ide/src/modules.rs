//! The modules of the project: `import("utils")` loads `utils.lua` (or
//! `utils/main.lua`) next to the script; its global functions, but those
//! starting with `_`, are its members.

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use xmake_syntax::{
    SyntaxKind, SyntaxNode, TextRange,
    ast::{AstNode, FieldExpr, NameRef},
};

use crate::{AnalyzedFile, BindingKind, Project, resolve::resolve};

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
    crate::paths::resolve_module(dir, module)
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

/// The uses of `name`, a function of the module file `module`, in the files
/// of the project: `m.name` with `m` imported from that file.
pub(crate) fn member_uses<'a>(
    project: &'a Project,
    module: &Path,
    name: &str,
) -> Vec<(&'a Path, TextRange)> {
    let mut out = Vec::new();
    for (path, file) in project.files() {
        let Some(dir) = path.parent() else {
            continue;
        };
        let fields = file.root().descendants().filter_map(FieldExpr::cast);
        for field in fields {
            let Some(member) = field.field().filter(|f| f.text() == name) else {
                continue;
            };
            let Some(ident) = field.base().and_then(NameRef::cast).and_then(|b| b.ident()) else {
                continue;
            };
            let Some(BindingKind::Import { module: imported }) =
                resolve(file, ident).map(|b| b.kind)
            else {
                continue;
            };
            let resolved = module_path(dir, imported).map(|p| crate::paths::normalize(&p));
            if resolved.as_deref() == Some(module) {
                out.push((path, member.text_range()));
            }
        }
    }
    out
}
