//! The diagnostics of xmake scripts: syntax errors, the checks against the
//! API database and the project, the type checks, and the lints that name
//! them, with their levels and fixes.

mod annotations;
mod diagnostics;
mod fixes;
mod globals;
mod lints;
mod relations;
mod typecheck;

use xmake_project::{
    AnalyzedFile, FileKind, LineCol, Project, compare_versions, context, fuzzy, outline, packages,
    paths, resolve, tables,
};
use xmake_types::{Type, type_of};

pub use crate::{
    diagnostics::{Diagnostic, Fix, Severity, diagnostics},
    fixes::{allow_fix, apply_safe_fixes, insert_import},
    lints::{LINTS, Level, Lint, LintLevels, lint},
};
