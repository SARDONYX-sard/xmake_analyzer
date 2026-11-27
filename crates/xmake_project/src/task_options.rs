//! The options of a task, from its menu, for `option.get("name")` in its
//! scripts:
//!
//! ```lua
//! task("hello")
//!     set_menu {options = {{"n", "name", "kv", nil, "The name."}}}
//!     on_run(function ()
//!         import("core.base.option")
//!         print(option.get("name"))
//!     end)
//! ```

use xmake_api::Scope;
use xmake_syntax::{
    SyntaxKind, SyntaxToken, TextSize,
    ast::{AstNode, CallExpr, FieldExpr, Literal, NameRef, TableExpr},
};

use crate::{AnalyzedFile, BindingKind, resolve::resolve};

/// The options xmake gives every task: `xmake hello -v` works without a
/// `verbose` in the menu.
pub const COMMON_OPTIONS: &[(&str, &str)] = &[
    ("verbose", "Print lots of verbose information (`-v`)."),
    ("diagnosis", "Print lots of diagnosis information (`-D`)."),
    ("quiet", "Quiet operation (`-q`)."),
    ("yes", "Input yes by default if need user confirm (`-y`)."),
    ("root", "Allow to run xmake as root."),
    ("profile", "Print performance data for debugging."),
    ("backtrace", "Print backtrace information for debugging."),
    ("project", "Change to the given project directory (`-P`)."),
    ("file", "Read a given xmake.lua file (`-F`)."),
    ("help", "Print this help message and exit (`-h`)."),
    ("version", "Print the version number and exit."),
];

/// An option of the menu of a task: `--name`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TaskOption {
    pub long: String,
    pub description: String,
}

/// The options of the menu of the task holding `offset`; `None` outside of
/// a task, or when its menu is not written as literals (then any name may
/// exist).
pub fn task_options(file: &AnalyzedFile, offset: TextSize) -> Option<Vec<TaskOption>> {
    let task = file
        .outline
        .definitions
        .iter()
        .find(|d| d.scope == Scope::Task && d.full_range.contains(offset))?;
    let root = file.root();
    let menu = root
        .descendants()
        .filter(|n| task.full_range.contains_range(n.text_range()))
        .filter_map(CallExpr::cast)
        .find(|c| c.callee_name() == Some("set_menu"));
    // No menu: the task takes the common options only.
    let Some(menu) = menu else {
        return Some(Vec::new());
    };
    let table = TableExpr::cast(menu.arg(0)?)?;
    let Some(options) =
        table.fields().find(|f| f.key_name().is_some_and(|k| k.text() == "options"))
    else {
        return Some(Vec::new());
    };
    let options = TableExpr::cast(options.value()?)?;
    let mut out = Vec::new();
    for option in options.fields() {
        let option = TableExpr::cast(option.value()?)?;
        let values: Vec<_> = option.fields().filter(|f| f.key_name().is_none()).collect();
        let string = |i: usize| -> Option<String> {
            Some(Literal::cast(values.get(i)?.value()?)?.string_value()?.into_owned())
        };
        // `{}` separates groups in the help.
        if values.is_empty() {
            continue;
        }
        out.push(TaskOption { long: string(1)?, description: string(4).unwrap_or_default() });
    }
    Some(out)
}

/// Whether the string `token` is the name of `option.get("name")`, with
/// `option` the module `core.base.option`.
pub fn is_option_get(file: &AnalyzedFile, token: SyntaxToken<'_>) -> bool {
    let Some((call, index)) = crate::args::enclosing_call(token) else {
        return false;
    };
    let Some(field) = call.callee().and_then(FieldExpr::cast) else {
        return false;
    };
    let is_get = field.field().is_some_and(|f| f.text() == "get");
    let module = field
        .base()
        .and_then(NameRef::cast)
        .and_then(|n| n.ident())
        .and_then(|ident| resolve(file, ident))
        .map(|b| b.kind);
    index == 0
        && is_get
        && matches!(module, Some(BindingKind::Import { module: "core.base.option" }))
        && token.kind() == SyntaxKind::STRING
}
