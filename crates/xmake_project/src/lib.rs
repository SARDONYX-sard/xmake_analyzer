//! The files of an xmake project, analyzed: the tree, the outline (scopes,
//! definitions, includes, requires), the bindings, and the project around
//! them (the include graph, the package repositories).
//!
//! Everything the IDE features, the types and the lints share.

pub mod annotations;
pub mod args;
pub mod context;
pub mod doc_comment;
pub mod file;
pub mod fuzzy;
pub mod line_index;
pub mod outline;
pub mod packages;
pub mod paths;
pub mod project;
pub mod resolve;
pub mod tables;

pub use xmake_api;
pub use xmake_syntax::{self, TextRange, TextSize};

pub use crate::{
    args::{call_arg_values, enclosing_call, is_flags_api, option_of_value},
    context::CodeContext,
    file::{AnalyzedFile, FileKind},
    line_index::{LineCol, LineIndex},
    outline::{Callback, Definition, Outline},
    packages::{PackageDetails, PackageEntry, PackageIndex, compare_versions, default_globaldir},
    paths::{PathKind, check_path, normalize, resolve_include, resolve_module, resolve_paths},
    project::{Located, Project, ProjectScope},
    resolve::{Binding, BindingKind, resolve, visible_bindings},
};
