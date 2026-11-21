//! IDE features for xmake scripts: completion, hover, inlay hints,
//! navigation and symbols.
//!
//! Everything works on an [`AnalyzedFile`] (text, tree, outline) of a
//! [`Project`] (all files plus the API database). Results borrow from the
//! project: conversion to the protocol, and its allocations, happen in the
//! server.
//!
//! The layers below are re-exported: the project ([`xmake_project`]), the
//! types ([`xmake_types`]) and the diagnostics ([`xmake_lint`]).

mod assists;
mod completion;
mod hover;
mod inlay;
mod navigation;
#[cfg(test)]
mod package_tests;
mod semantic;
mod signature;
mod symbols;
mod tables;
mod variables;

use xmake_project::{context, doc_comment, outline, packages, paths};
use xmake_types as types;

pub use xmake_lint::{
    self, Diagnostic, Fix, LINTS, Level, Lint, LintLevels, Severity, allow_fix, apply_safe_fixes,
    diagnostics, lint,
};
pub use xmake_project::{
    self, AnalyzedFile, Binding, BindingKind, Callback, CodeContext, Definition, FileKind, LineCol,
    LineIndex, Located, Outline, PackageDetails, PackageEntry, PackageIndex, PathKind, Project,
    ProjectScope, TextRange, TextSize, check_path, default_globaldir, normalize, resolve,
    resolve_include, resolve_paths, visible_bindings, xmake_api, xmake_syntax,
};
pub use xmake_types::{self, Type, type_of};

pub use crate::{
    assists::{Assist, assists},
    completion::{CompletionItem, CompletionKind, completions},
    hover::{Hover, hover},
    inlay::{InlayHint, InlayKind, inlay_hints},
    navigation::{
        NavTarget, code_lenses, goto_definition, highlights, prepare_rename, references, rename,
    },
    semantic::{SemanticToken, TokenModifier, TokenType, semantic_tokens},
    signature::{SignatureHelp, signature_help},
    symbols::{
        Fold, Symbol, SymbolKind, document_symbols, folding_ranges, runnable_tasks,
        workspace_symbols,
    },
};
