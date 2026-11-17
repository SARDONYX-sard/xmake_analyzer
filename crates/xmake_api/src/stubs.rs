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
//! signature of a callable module such as `lib.detect.find_tool`). The other
//! comment lines are the Markdown documentation.
//!
//! The stubs are parsed once, on first use; names and types borrow the
//! embedded text.

use std::sync::OnceLock;

use rustc_hash::FxHashMap;
use xmake_syntax::{Dialect, SyntaxElement, SyntaxKind, SyntaxNode, parse};

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
}

impl Function {
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
}

impl Stubs {
    /// The stubs embedded in the binary.
    pub fn get() -> &'static Stubs {
        static STUBS: OnceLock<Stubs> = OnceLock::new();
        STUBS.get_or_init(|| {
            let mut stubs = Stubs::default();
            for (_, text) in STUB_FILES {
                stubs.parse_file(text);
            }
            stubs
        })
    }

    /// The class or module `name`.
    /// Every class: the instances (`target`, ...) and the modules (`os`, ...).
    pub fn classes(&self) -> impl Iterator<Item = &Class> {
        self.classes.iter()
    }

    pub fn class(&self, name: &str) -> Option<&Class> {
        self.classes.iter().find(|c| c.name == name)
    }

    /// The functions and methods of `owner`.
    pub fn members<'a>(&'a self, owner: &'a str) -> impl Iterator<Item = &'a Function> + 'a {
        self.functions.iter().filter(move |f| f.owner == owner)
    }

    /// The function or method `name` of `owner`.
    pub fn member(&self, owner: &str, name: &str) -> Option<&Function> {
        self.functions.iter().find(|f| f.owner == owner && f.name == name)
    }

    fn parse_file(&mut self, text: &'static str) {
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
                    self.statement(text, node, &comments, &mut tables);
                    comments.clear();
                }
            }
        }
    }

    fn statement(
        &mut self,
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
