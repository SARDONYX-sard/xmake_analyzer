//! The structure of a description file: which scope each offset is in, and
//! what the file defines.
//!
//! xmake opens scopes with plain calls executed in order:
//!
//! ```lua
//! target("foo")            -- the target scope starts after this call
//!     set_kind("binary")
//!     if is_plat("windows") then add_links("user32") end
//! target_end()             -- or the next `target(...)`, `option(...)`, ...
//! ```
//!
//! so the scope at an offset is the one opened by the last scope call before
//! it, in source order, looking inside `if`/`do`/loops but not inside
//! functions, which run later as scripts. A function passed to a scope call
//! (`target("foo", function () ... end)`) or to `namespace` is a description
//! body with that scope.
//!
//! Names are not copied: the outline stores ranges into the text.

use std::borrow::Cow;

use xmake_api::{Scope, ScopeEdge, Version};
use xmake_syntax::{
    SyntaxKind, SyntaxNode, SyntaxToken, TextRange, TextSize,
    ast::{AstNode, Block, CallExpr, Literal},
};

/// A change of scope at `start`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ScopeRegion {
    start: TextSize,
    scope: Scope,
}

/// Something a scope call defines: `target("foo")`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Definition {
    pub scope: Scope,
    /// The name, inside the quotes.
    pub name_range: TextRange,
    /// The scope call.
    pub call_range: TextRange,
    /// From the scope call to the end of the scope.
    pub full_range: TextRange,
}

/// A function passed to a script API: `on_build(function (target) ... end)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Callback {
    /// The `FUNCTION_EXPR`.
    pub function_range: TextRange,
    /// The scope the API was called in.
    pub scope: Scope,
}

/// A problem found while tracking scopes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutlineError {
    pub range: TextRange,
    pub message: Cow<'static, str>,
    /// A fix: the callee and the `_end` function to call instead.
    pub replacement: Option<(TextRange, &'static str)>,
}

/// See the module documentation.
#[derive(Clone, Debug, Default)]
pub struct Outline {
    /// Sorted by `start`.
    regions: Vec<ScopeRegion>,
    pub definitions: Vec<Definition>,
    pub callbacks: Vec<Callback>,
    /// Function bodies that are description code (scope and namespace
    /// callbacks).
    pub description_bodies: Vec<TextRange>,
    /// Package names of `add_requires`, inside the quotes, without version.
    pub requires: Vec<TextRange>,
    /// The `alias` option of the `add_requires` of each of `requires`: the
    /// name `add_packages` uses then.
    pub require_aliases: Vec<Option<TextRange>>,
    /// String arguments of `includes`, inside the quotes.
    pub includes: Vec<TextRange>,
    /// `includes(path.join(os.scriptdir(), "../clang"))`: computed paths,
    /// relative to the script.
    pub computed_includes: Vec<String>,
    /// Arguments of `includes` that cannot be evaluated: the project may
    /// define more than what is known.
    pub unknown_includes: Vec<TextRange>,
    /// `add_repositories("local-repo repo")`: the paths of the local
    /// repositories, as written (relative to the script, or absolute).
    pub repositories: Vec<String>,
    /// `set_xmakever("2.8.0")`.
    pub xmakever: Option<(Version, TextRange)>,
    pub errors: Vec<OutlineError>,
}

impl Outline {
    /// Computes the outline of a description file.
    pub fn build(root: SyntaxNode<'_>) -> Outline {
        let mut walker = Walker {
            outline: Outline::default(),
            current: Scope::Root,
            open: None,
            wrappers: Vec::new(),
        };
        if let Some(block) = root.children().find_map(Block::cast) {
            walker.block(block);
        }
        walker.close_definition(root.text_range().end());
        walker.outline
    }

    /// The description scope at `offset`.
    pub fn scope_at(&self, offset: TextSize) -> Scope {
        let i = self.regions.partition_point(|r| r.start <= offset);
        i.checked_sub(1).map_or(Scope::Root, |i| self.regions[i].scope)
    }

    /// The callback whose function is exactly `range`.
    pub fn callback(&self, function_range: TextRange) -> Option<&Callback> {
        self.callbacks.iter().find(|c| c.function_range == function_range)
    }
}

struct Walker {
    outline: Outline,
    current: Scope,
    /// The definition whose scope is open, to close its `full_range`.
    open: Option<usize>,
    /// Functions defining a scope named by a parameter: `snapshot_lib` of
    /// `local function snapshot_lib(name) target(name, ...) end`, with the
    /// scope and the index of the parameter.
    wrappers: Vec<(String, Scope, usize)>,
}

impl Walker {
    fn enter(&mut self, start: TextSize, scope: Scope) {
        self.current = scope;
        let regions = &mut self.outline.regions;
        // Keep `start` strictly increasing: a later change at the same
        // offset wins.
        if regions.last().is_some_and(|r| r.start == start) {
            regions.pop();
        }
        regions.push(ScopeRegion { start, scope });
    }

    fn close_definition(&mut self, end: TextSize) {
        let Some(index) = self.open.take() else {
            return;
        };
        let def = &mut self.outline.definitions[index];
        def.full_range = TextRange::new(def.call_range.start(), end.max(def.call_range.end()));
    }

    fn block(&mut self, block: Block<'_>) {
        for stmt in block.statements() {
            self.statement(stmt);
        }
    }

    fn statement(&mut self, stmt: SyntaxNode<'_>) {
        match stmt.kind() {
            SyntaxKind::CALL_STMT => {
                if let Some(call) = stmt.children().find_map(CallExpr::cast) {
                    self.call(call);
                }
            }
            // Control flow runs in order: look inside.
            SyntaxKind::IF_STMT
            | SyntaxKind::ELSEIF_CLAUSE
            | SyntaxKind::ELSE_CLAUSE
            | SyntaxKind::DO_STMT
            | SyntaxKind::WHILE_STMT
            | SyntaxKind::REPEAT_STMT
            | SyntaxKind::NUMERIC_FOR_STMT
            | SyntaxKind::GENERIC_FOR_STMT => {
                for child in stmt.children() {
                    match child.kind() {
                        SyntaxKind::BLOCK => self.block(Block::cast(child).expect("block")),
                        SyntaxKind::ELSEIF_CLAUSE | SyntaxKind::ELSE_CLAUSE => {
                            self.statement(child);
                        }
                        _ => {}
                    }
                }
            }
            // `function toolchain_clang() toolchain("clang") ... end` called
            // at the top level: a helper of the description, not a script.
            SyntaxKind::FUNCTION_STMT | SyntaxKind::LOCAL_FUNCTION_STMT => {
                self.wrapper(stmt);
                let Some(body) = stmt.children().find_map(Block::cast) else {
                    return;
                };
                if describes(body) {
                    let outer = self.current;
                    self.outline.description_bodies.push(stmt.text_range());
                    self.enter(body.syntax().text_range().start(), Scope::Root);
                    self.block(body);
                    self.close_definition(body.syntax().text_range().end());
                    self.enter(body.syntax().text_range().end(), outer);
                }
            }
            _ => {}
        }
    }

    /// Records `stmt` when it defines a scope named by one of its parameters:
    /// `local function lib(name) target(name, function () ... end) end`.
    fn wrapper(&mut self, stmt: SyntaxNode<'_>) {
        let name = match stmt.kind() {
            SyntaxKind::LOCAL_FUNCTION_STMT => stmt
                .children()
                .find(|n| n.kind() == SyntaxKind::NAME)
                .and_then(|n| n.child_token(SyntaxKind::IDENT)),
            _ => stmt.child_node(SyntaxKind::FUNC_NAME).and_then(|n| {
                let mut idents = n.child_tokens().filter(|t| !t.kind().is_trivia());
                let ident = idents.next().filter(|t| t.kind() == SyntaxKind::IDENT)?;
                idents.next().is_none().then_some(ident)
            }),
        };
        let Some(name) = name else {
            return;
        };
        let Some(params) = stmt.descendants().find_map(xmake_syntax::ast::ParamList::cast) else {
            return;
        };
        let params: Vec<&str> = params.params().filter_map(|p| Some(p.ident()?.text())).collect();
        let Some(body) = stmt.children().find_map(Block::cast) else {
            return;
        };
        for call in body.syntax().descendants().filter_map(CallExpr::cast) {
            let opener = call.callee_name().and_then(Scope::from_opener);
            let Some((scope, ScopeEdge::Begin)) = opener else {
                continue;
            };
            let arg = call.arg(0).and_then(xmake_syntax::ast::NameRef::cast);
            let index = arg.and_then(|a| params.iter().position(|p| *p == a.text()));
            if let Some(index) = index {
                self.wrappers.push((name.text().to_owned(), scope, index));
            }
        }
    }

    fn call(&mut self, call: CallExpr<'_>) {
        let Some(name) = call.callee_name() else {
            return;
        };
        // `snapshot_lib("hkdump-snapshot")`: a target named by the caller.
        let wrapped: Vec<(Scope, usize)> = self
            .wrappers
            .iter()
            .filter(|(w, _, _)| w == name)
            .map(|&(_, scope, index)| (scope, index))
            .collect();
        for (scope, index) in wrapped {
            let token = call
                .arg(index)
                .and_then(Literal::cast)
                .and_then(|l| l.token())
                .filter(|t| t.kind().is_string());
            if let Some(token) = token {
                let call_range = call.syntax().text_range();
                let name_range = content_range(token);
                let full_range = call_range;
                self.outline.definitions.push(Definition {
                    scope,
                    name_range,
                    call_range,
                    full_range,
                });
            }
        }
        if let Some((scope, edge)) = Scope::from_opener(name) {
            match edge {
                ScopeEdge::Begin => self.begin_scope(call, scope),
                ScopeEdge::End => self.end_scope(call, scope),
            }
            return;
        }
        match name {
            "namespace" => {
                let current = self.current;
                self.description_body(call, current);
            }
            "includes" | "add_subfiles" | "add_subdirs" => {
                for arg in call.arg_list().into_iter().flat_map(|a| a.args()) {
                    let token =
                        Literal::cast(arg).and_then(|l| l.token()).filter(|t| t.kind().is_string());
                    if let Some(token) = token {
                        self.outline.includes.push(content_range(token));
                    } else if let Some(path) = script_relative_path(arg) {
                        self.outline.computed_includes.push(path);
                    } else if arg.kind() != SyntaxKind::TABLE_EXPR {
                        self.outline.unknown_includes.push(arg.text_range());
                    }
                }
            }
            "add_repositories" => {
                for (_, value) in string_args(call) {
                    // `"name url [branch]"`: a path, not a git URL.
                    let mut words = value.split_whitespace();
                    let (Some(_), Some(url)) = (words.next(), words.next()) else {
                        continue;
                    };
                    if !url.contains("://") && !url.starts_with("git@") {
                        self.outline.repositories.push(url.to_owned());
                    }
                }
            }
            "add_requires" => {
                let first = self.outline.requires.len();
                for (token, value) in string_args(call) {
                    // `"zlib 1.2.x"`: the name is the first word;
                    // `"conan::zlib/1.2.13"`: without the version.
                    let word = value.split_whitespace().next().unwrap_or("");
                    let name = if word.contains("::") {
                        word.split('/').next().unwrap_or(word)
                    } else {
                        word
                    };
                    let name_len = name.len();
                    let start = content_range(token).start();
                    let range = TextRange::at(start, TextSize::new(name_len as u32));
                    self.outline.requires.push(range);
                    self.outline.require_aliases.push(None);
                }
                let tables = call.arg_list().into_iter().flat_map(|a| a.args());
                for table in tables.filter_map(xmake_syntax::ast::TableExpr::cast) {
                    let alias =
                        table.fields().find(|f| f.key_name().is_some_and(|k| k.text() == "alias"));
                    let token = alias
                        .and_then(|f| f.value())
                        .and_then(Literal::cast)
                        .and_then(|l| l.token());
                    // An alias names one package.
                    let single = self.outline.requires.len() == first + 1;
                    if let Some(token) = token.filter(|t| t.kind().is_string() && single) {
                        let alias = content_range(token);
                        for slot in &mut self.outline.require_aliases[first..] {
                            *slot = Some(alias);
                        }
                    }
                }
            }
            "set_xmakever" => {
                if let Some((token, value)) = string_args(call).next() {
                    let version = Version::parse(&value);
                    self.outline.xmakever = version.map(|v| (v, token.text_range()));
                }
            }
            _ if is_script_api(name) => {
                let scope = self.current;
                for function in function_args(call) {
                    let function_range = function.text_range();
                    self.outline.callbacks.push(Callback { function_range, scope });
                }
            }
            _ => {}
        }
    }

    fn begin_scope(&mut self, call: CallExpr<'_>, scope: Scope) {
        let call_range = call.syntax().text_range();
        self.close_definition(call_range.start());

        if let Some((token, _)) = string_args(call).next() {
            let name_range = content_range(token);
            let full_range = call_range;
            self.outline.definitions.push(Definition { scope, name_range, call_range, full_range });
            self.open = Some(self.outline.definitions.len() - 1);
        }

        // `target("foo", {kind = "binary"})` and `target("foo", function ()
        // ... end)` define the target in one call: the scope does not stay
        // open after it.
        let one_shot = call.arg_list().is_some_and(|args| {
            args.args().nth(1).is_some_and(|a| {
                matches!(a.kind(), SyntaxKind::TABLE_EXPR | SyntaxKind::FUNCTION_EXPR)
            })
        });
        if one_shot {
            self.description_body(call, scope);
            self.close_definition(call_range.end());
            self.enter(call_range.end(), Scope::Root);
        } else {
            self.enter(call_range.end(), scope);
        }
    }

    fn end_scope(&mut self, call: CallExpr<'_>, scope: Scope) {
        let call_range = call.syntax().text_range();
        if self.current != scope {
            let message = format!(
                "`{}_end()` without an open {} scope (current scope: {})",
                scope.name(),
                scope.name(),
                self.current.name()
            );
            let callee = call.callee().map(|c| c.text_range());
            let replacement = callee.zip(end_function(self.current));
            self.outline.errors.push(OutlineError {
                range: call_range,
                message: message.into(),
                replacement,
            });
        }
        self.close_definition(call_range.end());
        self.enter(call_range.end(), Scope::Root);
    }

    /// Walks the functions passed to `call` as description code of `scope`.
    fn description_body(&mut self, call: CallExpr<'_>, scope: Scope) {
        let outer = self.current;
        for function in function_args(call) {
            let Some(body) = function.children().find_map(Block::cast) else {
                continue;
            };
            let body_range = body.syntax().text_range();
            self.outline.description_bodies.push(function.text_range());
            self.enter(body_range.start(), scope);
            self.block(body);
            self.enter(body_range.end(), outer);
        }
        self.current = outer;
    }
}

/// The function closing `scope`.
pub fn end_function(scope: Scope) -> Option<&'static str> {
    match scope {
        Scope::Root => None,
        Scope::Target => Some("target_end"),
        Scope::Option => Some("option_end"),
        Scope::Rule => Some("rule_end"),
        Scope::Package => Some("package_end"),
        Scope::Toolchain => Some("toolchain_end"),
        Scope::Task => Some("task_end"),
    }
}

/// Whether a function body calls scope functions or description APIs at its
/// top level.
fn describes(body: Block<'_>) -> bool {
    let calls = body.statements().filter(|s| s.kind() == SyntaxKind::CALL_STMT);
    let mut names =
        calls.filter_map(|s| s.children().find_map(CallExpr::cast)).filter_map(|c| c.callee_name());
    names.any(|n| {
        Scope::from_opener(n).is_some()
            || n.starts_with("set_")
            || n.starts_with("add_")
            || n == "includes"
    })
}

/// Evaluates `path.join(os.scriptdir(), "a", "b")` and `os.scriptdir() ..
/// "/a"` to `a/b` and `/a`: paths relative to the script.
fn script_relative_path(expr: SyntaxNode<'_>) -> Option<String> {
    let is_scriptdir = |node: SyntaxNode<'_>| {
        let text: String = node.text().split_whitespace().collect();
        text == "os.scriptdir()" || text == "\"$(scriptdir)\"" || text == "'$(scriptdir)'"
    };
    if let Some(call) = CallExpr::cast(expr) {
        let callee: String = call.callee()?.text().split_whitespace().collect();
        if callee != "path.join" {
            return None;
        }
        let mut args = call.arg_list()?.args();
        if !is_scriptdir(args.next()?) {
            return None;
        }
        let mut parts = Vec::new();
        for arg in args {
            parts.push(Literal::cast(arg)?.string_value()?.into_owned());
        }
        return Some(parts.join("/"));
    }
    // `os.scriptdir() .. "/sub"`
    if expr.kind() == SyntaxKind::BIN_EXPR {
        let mut sides = expr.children();
        let (left, right) = (sides.next()?, sides.next()?);
        if is_scriptdir(left) {
            let rest = Literal::cast(right)?.string_value()?;
            return Some(rest.trim_start_matches(['/', '\\']).to_owned());
        }
    }
    None
}

/// `on_build`, `before_link`, `after_install`, ...
pub fn is_script_api(name: &str) -> bool {
    ["on_", "before_", "after_"].iter().any(|p| name.starts_with(p))
}

/// The `FUNCTION_EXPR` arguments of `call`.
fn function_args<'a>(call: CallExpr<'a>) -> impl Iterator<Item = SyntaxNode<'a>> {
    let args = call.arg_list().into_iter().flat_map(|a| a.args());
    args.filter(|a| a.kind() == SyntaxKind::FUNCTION_EXPR)
}

/// The string literal arguments of `call`, with their values.
fn string_args<'a>(call: CallExpr<'a>) -> impl Iterator<Item = (SyntaxToken<'a>, Cow<'a, str>)> {
    let args = call.arg_list().into_iter().flat_map(|a| a.args());
    args.filter_map(Literal::cast).filter_map(|lit| Some((lit.token()?, lit.string_value()?)))
}

/// The range of a string token without its delimiters.
pub fn content_range(token: SyntaxToken<'_>) -> TextRange {
    let range = token.text_range();
    let text = token.text();
    let (open, close) = match token.kind() {
        SyntaxKind::LONG_STRING => {
            let level = text[1..].bytes().take_while(|&b| b == b'=').count() as u32;
            let closed = text.ends_with(']') && text.len() as u32 >= 2 * (level + 2);
            (level + 2, if closed { level + 2 } else { 0 })
        }
        _ => {
            let quote = text.as_bytes()[0];
            let closed = text.len() >= 2 && text.as_bytes()[text.len() - 1] == quote;
            (1, u32::from(closed))
        }
    };
    let start = range.start() + TextSize::new(open);
    let end = (range.end() - TextSize::new(close)).max(start);
    TextRange::new(start, end)
}

#[cfg(test)]
mod tests {
    use xmake_syntax::{Dialect, parse};

    use super::*;

    fn outline(text: &str) -> Outline {
        Outline::build(parse(text, Dialect::XMAKE_DEFAULT).root(text))
    }

    fn scope_at_marker(text: &str) -> Scope {
        let offset = text.find('$').expect("marker");
        outline(text).scope_at(TextSize::new(offset as u32))
    }

    #[test]
    fn scopes_follow_calls() {
        assert_eq!(scope_at_marker("target('a')\n $"), Scope::Target);
        assert_eq!(scope_at_marker("target('a')\ntarget_end()\n$"), Scope::Root);
        assert_eq!(scope_at_marker("target('a')\noption('b')\n$"), Scope::Option);
        assert_eq!(scope_at_marker("$target('a')"), Scope::Root);
        assert_eq!(scope_at_marker("if x then target('a') end\n$"), Scope::Target);
        assert_eq!(scope_at_marker("target('a', function () $ end)\n"), Scope::Target);
        assert_eq!(scope_at_marker("target('a', function () end)\n$"), Scope::Root);
        assert_eq!(scope_at_marker("target('a', {kind = 'binary'})\n$"), Scope::Root);
    }

    #[test]
    fn defined_by_helpers() {
        let text = "local function lib(name, arch)\n  target(name, function () set_kind('static') end)\nend\nlib('core')\nlib('core-x64', 'x64')\nlib(computed)\n";
        let o = outline(text);
        let names: Vec<_> = o.definitions.iter().map(|d| &text[d.name_range]).collect();
        assert_eq!(names, ["core", "core-x64"]);
        assert!(o.definitions.iter().all(|d| d.scope == Scope::Target));
    }

    #[test]
    fn definitions_and_callbacks() {
        let text = "add_requires('zlib 1.2.x', \"fmt\")\ntarget('app')\n  on_build(function (t) end)\ntarget_end()\nrule('r')\n";
        let o = outline(text);
        let names: Vec<_> = o.definitions.iter().map(|d| &text[d.name_range]).collect();
        assert_eq!(names, ["app", "r"]);
        let requires: Vec<_> = o.requires.iter().map(|r| &text[*r]).collect();
        assert_eq!(requires, ["zlib", "fmt"]);
        assert_eq!(o.callbacks.len(), 1);
        assert_eq!(o.callbacks[0].scope, Scope::Target);
        let app = &o.definitions[0];
        assert!(text[app.full_range].ends_with("target_end()"));
        assert!(o.errors.is_empty());
    }

    #[test]
    fn local_repositories() {
        let o = outline(
            "add_repositories('local-repo repo', 'xmake-repo https://github.com/xmake-io/xmake-repo.git dev')
add_repositories('ssh git@github.com:a/b.git')
",
        );
        assert_eq!(o.repositories, ["repo"]);
    }

    #[test]
    fn mismatched_end() {
        let o = outline("target('a')\noption_end()\n");
        assert_eq!(o.errors.len(), 1);
        assert!(o.errors[0].message.contains("option_end"));
    }

    #[test]
    fn description_helpers() {
        let text = "function define(name)
  toolchain(name)
    set_kind('standalone')
    $
end
define('a')
";
        assert_eq!(scope_at_marker(text), Scope::Toolchain);
        let o = outline(&text.replace('$', ""));
        assert_eq!(o.description_bodies.len(), 1);
        assert_eq!(
            scope_at_marker(
                "function f()
  print(1) $
end
"
            ),
            Scope::Root
        );
    }

    #[test]
    fn xmakever() {
        let o = outline("set_xmakever(\"2.8.5\")");
        assert_eq!(o.xmakever.map(|v| v.0), Some(Version::new(2, 8, 5)));
    }
}
