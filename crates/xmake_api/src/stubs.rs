//! Typed stubs of the script APIs, written as Lua with LuaCATS annotations
//! in `stubs/*.lua`:
//!
//! ```lua
//! --- Copies files or directories.
//! ---
//! --- # Examples
//! --- ...
//! ---@param src string The files, globs allowed.
//! ---@param opt? {rootdir?: string}
//! ---@return boolean ok
//! function os.cp(src, dst, opt) end
//! ```
//!
//! Supported annotations: `@class`, `@param`, `@return`, `@overload` (the
//! signature of a callable module such as `lib.detect.find_tool`), `@see`
//! (the section of the manual), `@alias` with `---| "value" # description`
//! lines (the values xmake knows: platforms, policies, ...). The other comment
//! lines are the Markdown documentation.
//!
//! The functions xmake lists carry tags telling where they exist, from which
//! [`crate::ApiDb`] is built:
//!
//! - `@xmake.kind values`: an API of the description scope of its class
//!   (`xmake.target`, ...), of that kind;
//! - `@xmake.context description script instance import`: a global function
//!   or a module function of the description or the script context, a method
//!   of an instance (`target:name()`), a function of an importable module;
//! - `@xmake.prototype set_kind(kind: string)`: the prototype of the manual.
//!
//! The stubs are parsed once, on first use; names and types borrow the
//! embedded text. Stubs written at run time for the installed xmake are
//! leaked and [installed](Stubs::install) instead.

use std::{
    path::{Path, PathBuf},
    sync::{OnceLock, RwLock},
};

use rustc_hash::FxHashMap;
use xmake_syntax::{Dialect, SyntaxElement, SyntaxKind, SyntaxNode, TextRange, parse};

use crate::ApiKind;

include!(concat!(env!("OUT_DIR"), "/stubs_table.rs"));

/// The owner of the global functions.
pub const GLOBALS: &str = "_G";

/// A parameter of a stub function.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Param {
    pub name: &'static str,
    /// A LuaCATS type: `string`, `target?`, `{rootdir?: string}`, ...
    pub ty: &'static str,
    pub optional: bool,
    pub doc: &'static str,
}

/// A returned value of a stub function.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Return {
    pub ty: &'static str,
    pub name: &'static str,
}

/// Where a function exists, see the module documentation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Contexts {
    pub description: bool,
    pub script: bool,
    pub instance: bool,
    pub import: bool,
}

impl Contexts {
    /// Parses `description script ...`.
    pub fn parse(s: &str) -> Contexts {
        let mut out = Contexts::default();
        for word in s.split_whitespace() {
            match word {
                "description" => out.description = true,
                "script" => out.script = true,
                "instance" => out.instance = true,
                "import" => out.import = true,
                _ => {}
            }
        }
        out
    }

    /// `description script`, empty for none.
    pub fn to_tag(self) -> String {
        let names = [
            (self.description, "description"),
            (self.script, "script"),
            (self.instance, "instance"),
            (self.import, "import"),
        ];
        names.iter().filter(|(on, _)| *on).map(|(_, n)| *n).collect::<Vec<_>>().join(" ")
    }

    pub fn is_empty(self) -> bool {
        self == Contexts::default()
    }
}

/// Where a stub is written: the file of the stubs and the range of its name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Location {
    /// `os.lua`
    pub file: &'static str,
    pub range: TextRange,
}

/// A function of a module or a method of a class.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function {
    /// The class or module: `os`, `target`, `core.base.json`.
    pub owner: &'static str,
    pub name: &'static str,
    /// Called with `:`.
    pub method: bool,
    pub params: Vec<Param>,
    pub returns: Vec<Return>,
    pub doc: String,
    /// `@xmake.kind`: an API of the description scope of [`Function::owner`].
    pub kind: Option<ApiKind>,
    /// `@xmake.context`
    pub contexts: Contexts,
    /// `@xmake.prototype`
    pub prototype: Option<&'static str>,
    /// `@xmake.param name description`: the parameters of the prototype.
    pub prototype_params: Vec<(&'static str, &'static str)>,
    /// `@see`: the section of the manual.
    pub see: Option<&'static str>,
    pub location: Location,
}

impl Function {
    /// Whether the stub types or documents the function: not only listed by
    /// xmake.
    pub fn is_typed(&self) -> bool {
        !self.doc.is_empty() || !self.params.is_empty() || !self.returns.is_empty()
    }

    /// `os.cp(src: string, dst: string) -> boolean`.
    pub fn signature(&self) -> String {
        let mut out = String::with_capacity(64);
        // Globals, and the description APIs of `xmake.*`, are called bare.
        if self.owner != GLOBALS && !self.owner.starts_with("xmake.") {
            out.push_str(self.owner.rsplit('.').next().unwrap_or(self.owner));
            out.push(if self.method { ':' } else { '.' });
        }
        out.push_str(self.name);
        push_params(&mut out, &self.params);
        push_returns(&mut out, &self.returns);
        out
    }
}

/// A class or a module table.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Class {
    pub name: &'static str,
    pub doc: String,
    /// `@overload fun(...)`: the module itself is callable.
    pub call: Option<&'static str>,
    /// `@see`: the section of the manual.
    pub see: Option<&'static str>,
    pub location: Location,
}

/// `---@alias name` with its `---| "value" # description` lines.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Alias {
    pub name: &'static str,
    pub values: Vec<(&'static str, &'static str)>,
}

fn push_params(out: &mut String, params: &[Param]) {
    out.push('(');
    for (i, p) in params.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(p.name);
        if p.optional {
            out.push('?');
        }
        if p.name != "..." || p.ty != "any" {
            out.push_str(": ");
            out.push_str(p.ty);
        }
    }
    out.push(')');
}

fn push_returns(out: &mut String, returns: &[Return]) {
    for (i, r) in returns.iter().enumerate() {
        out.push_str(if i == 0 { " -> " } else { ", " });
        out.push_str(r.ty);
    }
}

/// Every stub.
#[derive(Debug, Default)]
pub struct Stubs {
    classes: Vec<Class>,
    functions: Vec<Function>,
    aliases: Vec<Alias>,
    /// `@xmake.version` of the xmake the lists come from.
    pub version: Option<&'static str>,
    /// The files, `(name, text)`.
    files: Vec<(&'static str, &'static str)>,
    /// The directory holding [`Stubs::files`] on the disk, for editors to
    /// open them.
    dir: Option<PathBuf>,
}

/// The stubs in use: the builtin ones until others are installed.
static ACTIVE: RwLock<Option<&'static Stubs>> = RwLock::new(None);

impl Stubs {
    /// The stubs embedded in the binary.
    pub fn builtin() -> &'static Stubs {
        static STUBS: OnceLock<Stubs> = OnceLock::new();
        STUBS.get_or_init(|| Stubs::parse_files(STUB_FILES.to_vec()))
    }

    /// The stubs in use, see [`Stubs::install`].
    pub fn get() -> &'static Stubs {
        let active = ACTIVE.read().map(|a| *a).unwrap_or(None);
        active.unwrap_or_else(Stubs::builtin)
    }

    /// Uses `stubs` from now on: those written for the installed xmake. They
    /// live until the end of the process (an install per xmake probed).
    pub fn install(stubs: Stubs) -> &'static Stubs {
        let stubs: &'static Stubs = Box::leak(Box::new(stubs));
        if let Ok(mut active) = ACTIVE.write() {
            *active = Some(stubs);
        }
        stubs
    }

    /// Parses `(name, text)` files.
    pub fn parse_files(files: Vec<(&'static str, &'static str)>) -> Stubs {
        let mut stubs = Stubs::default();
        for &(name, text) in &files {
            stubs.parse_file(name, text);
        }
        stubs.files = files;
        stubs
    }

    /// Reads the `*.lua` files of `dir`; the texts are leaked, see
    /// [`Stubs::install`].
    pub fn read_dir(dir: &Path) -> std::io::Result<Stubs> {
        let mut files = Vec::new();
        for entry in std::fs::read_dir(dir)? {
            let path = entry?.path();
            if path.extension().is_none_or(|e| e != "lua") {
                continue;
            }
            let name =
                path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
            let text = std::fs::read_to_string(&path)?;
            files.push((&*String::leak(name), &*String::leak(text)));
        }
        files.sort_unstable();
        let mut stubs = Stubs::parse_files(files);
        stubs.dir = Some(dir.to_owned());
        Ok(stubs)
    }

    /// The files, `(name, text)`, sorted by name.
    pub fn files(&self) -> &[(&'static str, &'static str)] {
        &self.files
    }

    /// The directory of the files on the disk, when known.
    pub fn dir(&self) -> Option<&Path> {
        self.dir.as_deref()
    }

    /// Sets [`Stubs::dir`]: the files are there.
    pub fn set_dir(&mut self, dir: PathBuf) {
        self.dir = Some(dir);
    }

    /// The path of `location` on the disk, when the files are there.
    pub fn path_of(&self, location: Location) -> Option<PathBuf> {
        Some(self.dir.as_ref()?.join(location.file))
    }

    /// Every function and method.
    pub fn functions(&self) -> &[Function] {
        &self.functions
    }

    /// The alias `name`: `xmake.platform`, ...
    pub fn alias(&self, name: &str) -> Option<&Alias> {
        self.aliases.iter().find(|a| a.name == name)
    }

    /// The class or module `name`.
    /// Every class: the instances (`target`, ...) and the modules (`os`, ...).
    pub fn classes(&self) -> impl Iterator<Item = &Class> {
        self.classes.iter()
    }

    pub fn class(&self, name: &str) -> Option<&Class> {
        self.classes.iter().find(|c| c.name == name)
    }

    /// The typed functions and methods of `owner`, see [`Function::is_typed`].
    pub fn members<'a>(&'a self, owner: &'a str) -> impl Iterator<Item = &'a Function> + 'a {
        self.functions.iter().filter(move |f| f.owner == owner && f.is_typed())
    }

    /// The typed function or method `name` of `owner`.
    pub fn member(&self, owner: &str, name: &str) -> Option<&Function> {
        self.find(owner, name).filter(|f| f.is_typed())
    }

    /// The function or method `name` of `owner`, typed or only listed.
    pub fn find(&self, owner: &str, name: &str) -> Option<&Function> {
        self.functions.iter().find(|f| f.owner == owner && f.name == name)
    }

    fn parse_file(&mut self, file: &'static str, text: &'static str) {
        self.parse_aliases(text);
        if self.version.is_none() {
            self.version = text
                .lines()
                .find_map(|l| l.trim().strip_prefix("---@xmake.version"))
                .map(str::trim);
        }
        let parse = parse(text, Dialect::XMAKE_DEFAULT);
        let root = parse.root(text);
        // Local names of the class tables: `local target = {}`.
        let mut tables: FxHashMap<&'static str, &'static str> = FxHashMap::default();
        let mut comments: Vec<&'static str> = Vec::new();
        // Leading comments belong to the root, the others to the block.
        let elements = root.children_with_tokens().flat_map(|e| match e {
            SyntaxElement::Node(n) if n.kind() == SyntaxKind::BLOCK => {
                n.children_with_tokens().collect()
            }
            other => vec![other],
        });
        for element in elements {
            match element {
                SyntaxElement::Token(token) => match token.kind() {
                    SyntaxKind::LINE_COMMENT => comments.push(slice(text, token.text_range())),
                    SyntaxKind::WHITESPACE if token.text().matches('\n').count() > 1 => {
                        comments.clear()
                    }
                    _ => {}
                },
                SyntaxElement::Node(node) => {
                    self.statement(file, text, node, &comments, &mut tables);
                    comments.clear();
                }
            }
        }
    }

    /// `---@alias name` and its `---| "value" # description` lines.
    fn parse_aliases(&mut self, text: &'static str) {
        let mut current: Option<Alias> = None;
        for line in text.lines() {
            let line = line.trim();
            if let Some(value) = line.strip_prefix("---|") {
                let Some(alias) = current.as_mut() else {
                    continue;
                };
                let (value, description) = value.split_once(" #").unwrap_or((value, ""));
                let value = value.trim().trim_matches('"');
                alias.values.push((value, description.trim()));
                continue;
            }
            self.aliases.extend(current.take());
            if let Some(name) = line.strip_prefix("---@alias") {
                let name = name.split_whitespace().next().unwrap_or("");
                current = Some(Alias { name, values: Vec::new() });
            }
        }
        self.aliases.extend(current);
    }

    fn statement(
        &mut self,
        file: &'static str,
        text: &'static str,
        node: SyntaxNode<'_>,
        comments: &[&'static str],
        tables: &mut FxHashMap<&'static str, &'static str>,
    ) {
        let annotations = Annotations::parse(comments);
        match node.kind() {
            SyntaxKind::LOCAL_STMT | SyntaxKind::ASSIGN_STMT => {
                let Some(class) = annotations.class else {
                    return;
                };
                let var = node
                    .descendant_tokens()
                    .find(|t| t.kind() == SyntaxKind::IDENT)
                    .map(|t| slice(text, t.text_range()));
                if let Some(var) = var {
                    tables.insert(var, class);
                }
                self.classes.push(Class {
                    name: class,
                    doc: annotations.doc,
                    call: annotations.overload,
                    see: annotations.see,
                    location: Location { file, range: node.trimmed_range() },
                });
            }
            SyntaxKind::FUNCTION_STMT => {
                let Some(name) = node.child_node(SyntaxKind::FUNC_NAME) else {
                    return;
                };
                let parts: Vec<_> = name.child_tokens().filter(|t| !t.kind().is_trivia()).collect();
                // `function import(...)`: a global, owned by `_G`.
                let (owner, function, method) = match parts.as_slice() {
                    [function] => (GLOBALS, function, false),
                    [table, separator, function] => {
                        let table = slice(text, table.text_range());
                        (
                            tables.get(table).copied().unwrap_or(table),
                            function,
                            separator.kind() == SyntaxKind::COLON,
                        )
                    }
                    _ => return,
                };
                self.functions.push(Function {
                    owner,
                    name: slice(text, function.text_range()),
                    method,
                    params: annotations.params,
                    returns: annotations.returns,
                    doc: annotations.doc,
                    kind: annotations.kind,
                    contexts: annotations.contexts,
                    prototype: annotations.prototype,
                    prototype_params: annotations.prototype_params,
                    see: annotations.see,
                    location: Location { file, range: function.text_range() },
                });
            }
            _ => {}
        }
    }
}

/// `text[range]` with the static lifetime of the embedded stub.
fn slice(text: &'static str, range: xmake_syntax::TextRange) -> &'static str {
    &text[range]
}

/// The annotations and documentation of a comment block.
#[derive(Default)]
struct Annotations {
    doc: String,
    class: Option<&'static str>,
    overload: Option<&'static str>,
    params: Vec<Param>,
    returns: Vec<Return>,
    kind: Option<ApiKind>,
    contexts: Contexts,
    prototype: Option<&'static str>,
    prototype_params: Vec<(&'static str, &'static str)>,
    see: Option<&'static str>,
}

impl Annotations {
    fn parse(comments: &[&'static str]) -> Annotations {
        let mut out = Annotations::default();
        let mut doc_lines: Vec<&str> = Vec::new();
        for comment in comments {
            // `--- -- a Lua comment in an example` keeps its dashes.
            let line = comment
                .strip_prefix("---")
                .or_else(|| comment.strip_prefix("--"))
                .unwrap_or(comment);
            if let Some(annotation) = line.strip_prefix('@') {
                out.annotation(annotation);
                continue;
            }
            doc_lines.push(line.strip_prefix(' ').unwrap_or(line).trim_end());
        }
        while doc_lines.last().is_some_and(|l| l.is_empty()) {
            doc_lines.pop();
        }
        out.doc = doc_lines.join("\n");
        out
    }

    fn annotation(&mut self, annotation: &'static str) {
        let (tag, rest) = annotation.split_once(char::is_whitespace).unwrap_or((annotation, ""));
        let rest = rest.trim();
        match tag {
            "class" => self.class = rest.split([' ', ':']).next().filter(|c| !c.is_empty()),
            "overload" => self.overload = Some(rest),
            "param" => {
                let (name, rest) = rest.split_once(char::is_whitespace).unwrap_or((rest, "any"));
                let (ty, doc) = split_type(rest.trim());
                let optional = name.ends_with('?');
                let name = name.trim_end_matches('?');
                self.params.push(Param { name, ty, optional, doc: doc.trim() });
            }
            "return" => {
                let (ty, rest) = split_type(rest);
                let name = rest.split_whitespace().next().unwrap_or("");
                self.returns.push(Return { ty, name });
            }
            "see" => self.see = Some(rest).filter(|s| !s.is_empty()),
            "xmake.kind" => self.kind = ApiKind::parse(rest),
            "xmake.context" => self.contexts = Contexts::parse(rest),
            "xmake.prototype" => self.prototype = Some(rest).filter(|s| !s.is_empty()),
            "xmake.param" => {
                let (name, description) = rest.split_once(' ').unwrap_or((rest, ""));
                self.prototype_params.push((name, description.trim()));
            }
            _ => {}
        }
    }
}

/// Splits a type off the start of `s`: up to the first blank outside of
/// brackets.
fn split_type(s: &str) -> (&str, &str) {
    let mut depth = 0i32;
    for (i, c) in s.char_indices() {
        match c {
            '(' | '{' | '[' | '<' => depth += 1,
            ')' | '}' | ']' | '>' => depth -= 1,
            // `fun(): string` continues after `: `.
            c if c.is_whitespace()
                && depth <= 0
                && !s[..i].ends_with(':')
                && !s[..i].ends_with(',') =>
            {
                return (&s[..i], &s[i..]);
            }
            _ => {}
        }
    }
    (s, "")
}

/// The class a type refers to: `target?` and `target|nil` are `target`;
/// lists, maps and functions refer to none.
pub fn class_of(ty: &str) -> Option<&str> {
    let first = ty.split('|').map(str::trim).find(|t| *t != "nil")?;
    let name = first.trim_end_matches('?');
    let simple = name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'.');
    (simple && !name.is_empty()).then_some(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_stubs() {
        let stubs = Stubs::get();
        let cp = stubs.member("os", "cp").expect("os.cp");
        assert!(!cp.method);
        assert_eq!(cp.params.len(), 3);
        assert_eq!(cp.params[2].ty, "{rootdir?: string, symlink?: boolean}");
        assert!(cp.params[2].optional);
        assert!(cp.doc.contains("# Examples"), "{}", cp.doc);
        assert_eq!(
            cp.signature(),
            "os.cp(src: string, dst: string, opt?: {rootdir?: string, symlink?: boolean})"
        );

        let dep = stubs.member("target", "dep").expect("target:dep");
        assert!(dep.method);
        assert_eq!(dep.returns[0].ty, "target?");
        assert_eq!(class_of(dep.returns[0].ty), Some("target"));

        let iorun = stubs.member("os", "iorun").expect("os.iorun");
        assert_eq!(iorun.returns.len(), 2);
        assert_eq!(iorun.returns[1].name, "errdata");

        assert!(stubs.member("core.base.json", "decode").is_some());
        let find_tool = stubs.class("lib.detect.find_tool").expect("find_tool");
        assert!(
            find_tool.call.is_some_and(|c| c.starts_with("fun(name: string")),
            "{:?}",
            find_tool.call
        );
        assert!(stubs.member("string", "split").is_some());
    }

    #[test]
    fn types() {
        assert_eq!(split_type("{a?: string, b: c} The doc"), ("{a?: string, b: c}", " The doc"));
        assert_eq!(split_type("fun(a: x): y rest"), ("fun(a: x): y", " rest"));
        assert_eq!(class_of("string[]"), None);
        assert_eq!(class_of("nil|package"), Some("package"));
        assert_eq!(class_of("table<string, target>"), None);
    }
}
