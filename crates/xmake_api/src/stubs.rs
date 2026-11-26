//! The xmake APIs, written as Lua with LuaCATS annotations: the stubs
//! shipped with the server under `stubs/<xmake version>/`, read from the disk
//! so that editors open them on go-to-definition.
//!
//! ```lua
//! --- Copies files or directories.
//! ---
//! --- # Examples
//! --- ...
//! ---@param src string The files, globs allowed.
//! ---@param opt? {rootdir?: string}
//! ---@return boolean ok
//! ---@see https://xmake.io/api/scripts/builtin-modules/os.html#os-cp
//! function os.cp(src, dst, opt) end
//! ```
//!
//! Only standard annotations are used: `@class`, `@field`, `@param`,
//! `@return`, `@overload` (a callable module, `lib.detect.find_tool`), `@see`
//! (the manual), `@alias` with `---| "value" # description` lines. Where a
//! function exists follows from its file:
//!
//! | File | Functions |
//! | ---- | --------- |
//! | `description/scopes/<scope>.lua` | the APIs of a scope, by kind: `xmake.target.paths` |
//! | `description/globals.lua`, `description/<module>.lua` | the functions and modules of `xmake.lua` |
//! | `script/globals.lua`, `script/<module>.lua` | those of the scripts |
//! | `script/instances/<instance>.lua` | the methods of `target`, `package`, ... |
//! | `modules/<a>/<b>.lua` | the importable module `a.b` |
//! | `types/*.lua`, `values.lua` | types and values only |

use std::{
    path::{Path, PathBuf},
    sync::{OnceLock, RwLock},
};

use rustc_hash::FxHashMap;
use xmake_syntax::{Dialect, SyntaxElement, SyntaxKind, SyntaxNode, TextRange, parse};

use crate::{ApiKind, Instance, Scope};

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
    /// The string literals of the type: `binary` of `"binary"|"static"`,
    /// `(value, description)`.
    pub literals: Vec<(&'static str, &'static str)>,
}

impl Param {
    /// The parts of the type, `[]` removed: `xmake.value.kind` of
    /// `xmake.value.kind|xmake.value.kind[]`.
    pub fn type_names(&self) -> impl Iterator<Item = &'static str> {
        let ty: &'static str = self.ty;
        split_union(ty).into_iter().map(|p| p.trim().trim_end_matches("[]"))
    }
}

/// A returned value of a stub function.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Return {
    pub ty: &'static str,
    pub name: &'static str,
}

/// Where a function exists, from its file (see the module documentation).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Contexts {
    pub description: bool,
    pub script: bool,
    pub instance: bool,
    pub import: bool,
}

impl Contexts {
    pub fn is_empty(self) -> bool {
        self == Contexts::default()
    }
}

/// Where a stub is written: its file, relative to the stubs (`script/os.lua`),
/// and the range of its name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Location {
    pub file: &'static str,
    pub range: TextRange,
}

/// A function of a module or a method of a class.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function {
    /// The class or module: `os`, `target`, `core.base.json`,
    /// `xmake.target.values`.
    pub owner: &'static str,
    pub name: &'static str,
    /// Called with `:`.
    pub method: bool,
    pub params: Vec<Param>,
    pub returns: Vec<Return>,
    pub doc: String,
    /// `@see`: the section of the manual.
    pub see: Option<&'static str>,
    /// An API of a description scope, of this kind: the class
    /// `xmake.<scope>.<kind>`.
    pub scope: Option<(Scope, ApiKind)>,
    pub contexts: Contexts,
    /// The prototype in the notation of the manual, see [`Stubs::prototype`].
    pub prototype: Option<&'static str>,
    /// The other annotations, kept as written: `generic V`, `deprecated`.
    pub extra: Vec<&'static str>,
    pub location: Location,
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

    /// The parameter the `index`-th argument fills: the last one when it is
    /// `...`.
    pub fn param_at(&self, index: usize) -> Option<&Param> {
        self.params.get(index).or_else(|| self.params.last().filter(|p| p.name == "..."))
    }

    /// Whether the stub types or documents the function: not only listed by
    /// xmake.
    pub fn is_typed(&self) -> bool {
        !self.doc.is_empty() || !self.params.is_empty() || !self.returns.is_empty()
    }
}

/// `---@field name type description` of a class.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub name: &'static str,
    pub ty: &'static str,
    pub optional: bool,
    pub doc: &'static str,
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
    pub fields: Vec<Field>,
    /// `(name, description)` of [`Class::fields`].
    pub field_docs: Vec<(&'static str, &'static str)>,
    /// The other annotations, kept as written.
    pub extra: Vec<&'static str>,
    pub location: Location,
}

/// `---@alias name` with its `---| "value" # description` lines.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Alias {
    pub name: &'static str,
    /// The name in `---@alias name`.
    pub location: Location,
    /// The type of a one line alias: `---@alias xmake.value.target string`.
    pub ty: &'static str,
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
    /// The xmake version: the name of the directory.
    pub version: Option<String>,
    /// The files, `(path relative to the stubs, text)`.
    files: Vec<(&'static str, &'static str)>,
    /// The directory of the stubs.
    dir: Option<PathBuf>,
}

/// The stubs in use: the builtin ones until others are installed.
static ACTIVE: RwLock<Option<&'static Stubs>> = RwLock::new(None);

impl Stubs {
    /// The stubs shipped with the server (see [`Stubs::locate`]); none when
    /// they are not found.
    pub fn builtin() -> &'static Stubs {
        static STUBS: OnceLock<Stubs> = OnceLock::new();
        STUBS.get_or_init(|| match Stubs::locate().map(|dir| Stubs::read_dir(&dir)) {
            Some(Ok(stubs)) => stubs,
            _ => Stubs::default(),
        })
    }

    /// The directory of the newest stubs shipped with the server: under
    /// `$XMAKE_ANALYZER_STUBS`, `stubs/` next to the executable or the stubs
    /// of the sources (a build of the repository).
    pub fn locate() -> Option<PathBuf> {
        let roots = [
            std::env::var_os("XMAKE_ANALYZER_STUBS").map(PathBuf::from),
            std::env::current_exe().ok().and_then(|e| Some(e.parent()?.join("stubs"))),
            Some(PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/stubs"))),
        ];
        roots.into_iter().flatten().find_map(|root| newest_version(&root))
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

    /// Uses the builtin stubs again.
    pub fn reset() {
        if let Ok(mut active) = ACTIVE.write() {
            *active = None;
        }
    }

    /// Parses `(path, text)` files: `path` relative to the stubs, with `/`.
    pub fn parse_files(files: Vec<(&'static str, &'static str)>) -> Stubs {
        let mut stubs = Stubs::default();
        for &(path, text) in &files {
            stubs.parse_file(path, text);
        }
        stubs.files = files;
        // Once per load: the stubs live until the end of the process.
        let prototypes: Vec<Option<&'static str>> =
            stubs.functions.iter().map(|f| stubs.prototype(f).map(|p| &*String::leak(p))).collect();
        for (f, prototype) in stubs.functions.iter_mut().zip(prototypes) {
            f.prototype = prototype;
        }
        stubs
    }

    /// Reads the `*.lua` files under `dir`, whose name is the xmake version;
    /// the texts are leaked, see [`Stubs::install`].
    pub fn read_dir(dir: &Path) -> std::io::Result<Stubs> {
        let mut files = Vec::new();
        read_lua_files(dir, dir, &mut files)?;
        files.sort_unstable();
        let mut stubs = Stubs::parse_files(files);
        stubs.version = dir.file_name().map(|n| n.to_string_lossy().into_owned());
        stubs.dir = Some(dir.to_owned());
        Ok(stubs)
    }

    /// The files, `(path, text)`, sorted by path.
    pub fn files(&self) -> &[(&'static str, &'static str)] {
        &self.files
    }

    /// The directory of the stubs.
    pub fn dir(&self) -> Option<&Path> {
        self.dir.as_deref()
    }

    /// The path of `location` on the disk.
    pub fn path_of(&self, location: Location) -> Option<PathBuf> {
        Some(self.dir.as_ref()?.join(location.file))
    }

    /// Every function and method.
    pub fn functions(&self) -> &[Function] {
        &self.functions
    }

    /// Every alias.
    pub fn aliases(&self) -> &[Alias] {
        &self.aliases
    }

    /// The alias `name`: `xmake.value.platform`, ...
    pub fn alias(&self, name: &str) -> Option<&Alias> {
        self.aliases.iter().find(|a| a.name == name)
    }

    /// Every class: the instances (`target`, ...) and the modules (`os`, ...).
    pub fn classes(&self) -> impl Iterator<Item = &Class> {
        self.classes.iter()
    }

    /// The class or module `name`.
    pub fn class(&self, name: &str) -> Option<&Class> {
        self.classes.iter().find(|c| c.name == name)
    }

    /// The typed functions and methods of `owner`, see [`Function::is_typed`].
    pub fn members<'a>(&'a self, owner: &'a str) -> impl Iterator<Item = &'a Function> + 'a {
        let mut seen = rustc_hash::FxHashSet::default();
        self.functions
            .iter()
            .filter(move |f| f.owner == owner && f.is_typed() && seen.insert(f.name))
    }

    /// Every function and method of `owner`, typed or only listed.
    pub fn all_members<'a>(&'a self, owner: &'a str) -> impl Iterator<Item = &'a Function> + 'a {
        let mut seen = rustc_hash::FxHashSet::default();
        self.functions.iter().filter(move |f| f.owner == owner && seen.insert(f.name))
    }

    /// The typed function or method `name` of `owner`.
    pub fn member(&self, owner: &str, name: &str) -> Option<&Function> {
        let mut found = self.functions.iter().filter(|f| f.owner == owner && f.name == name);
        found.find(|f| f.is_typed())
    }

    /// The function or method `name` of `owner`, typed or only listed: the
    /// typed one first.
    pub fn find(&self, owner: &str, name: &str) -> Option<&Function> {
        self.member(owner, name)
            .or_else(|| self.functions.iter().find(|f| f.owner == owner && f.name == name))
    }

    /// The API `name` of `scope`, whatever its kind.
    pub fn scope_api(&self, scope: Scope, name: &str) -> Option<&Function> {
        self.functions.iter().find(|f| f.name == name && f.scope.is_some_and(|(s, _)| s == scope))
    }

    fn parse_file(&mut self, file: &'static str, text: &'static str) {
        self.parse_aliases(file, text);
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

    /// `---@alias name [type]` and its `---| "value" # description` lines.
    fn parse_aliases(&mut self, file: &'static str, text: &'static str) {
        let mut current: Option<Alias> = None;
        for raw in text.lines() {
            let line = raw.trim();
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
            if let Some(rest) = line.strip_prefix("---@alias") {
                let rest = rest.trim();
                let (name, ty) = rest.split_once(char::is_whitespace).unwrap_or((rest, ""));
                // `raw` and `name` are slices of `text`.
                let start = name.as_ptr() as usize - text.as_ptr() as usize;
                let range = TextRange::at((start as u32).into(), (name.len() as u32).into());
                let location = Location { file, range };
                current = Some(Alias { name, location, ty: ty.trim(), values: Vec::new() });
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
                let field_docs = annotations.fields.iter().map(|f| (f.name, f.doc)).collect();
                self.classes.push(Class {
                    name: class,
                    doc: annotations.doc,
                    call: annotations.overload,
                    see: annotations.see,
                    fields: annotations.fields,
                    field_docs,
                    extra: annotations.extra,
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
                let (scope, contexts) = place(file, owner, method);
                self.functions.push(Function {
                    owner,
                    name: slice(text, function.text_range()),
                    method,
                    params: annotations.params,
                    returns: annotations.returns,
                    doc: annotations.doc,
                    see: annotations.see,
                    scope,
                    contexts,
                    prototype: None,
                    extra: annotations.extra,
                    location: Location { file, range: function.text_range() },
                });
            }
            _ => {}
        }
    }
}

/// The newest `<version>/` under `root`.
fn newest_version(root: &Path) -> Option<PathBuf> {
    let key = |dir: &Path| -> Option<Vec<u64>> {
        dir.file_name()?.to_str()?.split('.').map(|p| p.parse().ok()).collect()
    };
    std::fs::read_dir(root)
        .ok()?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .filter_map(|p| Some((key(&p)?, p)))
        .max()
        .map(|(_, p)| p)
}

fn read_lua_files(
    root: &Path,
    dir: &Path,
    out: &mut Vec<(&'static str, &'static str)>,
) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            read_lua_files(root, &path, out)?;
            continue;
        }
        if path.extension().is_none_or(|e| e != "lua") {
            continue;
        }
        let relative = path.strip_prefix(root).unwrap_or(&path);
        let name = relative.to_string_lossy().replace('\\', "/");
        let text = std::fs::read_to_string(&path)?;
        out.push((&*String::leak(name), &*String::leak(text)));
    }
    Ok(())
}

/// The scope API and the contexts of a function of `owner` in `file`.
fn place(file: &str, owner: &str, method: bool) -> (Option<(Scope, ApiKind)>, Contexts) {
    let mut contexts = Contexts::default();
    let stem = file.rsplit('/').next().unwrap_or(file).trim_end_matches(".lua");
    if file.starts_with("description/scopes/") {
        let scope_api = owner.strip_prefix("xmake.").and_then(|rest| {
            let (scope, kind) = rest.split_once('.')?;
            let scope =
                std::iter::once(Scope::Root).chain(Scope::NAMED).find(|s| s.name() == scope)?;
            Some((scope, ApiKind::parse(kind)?))
        });
        return (scope_api, contexts);
    }
    if let Some(module) = file.strip_prefix("modules/") {
        let module = module.trim_end_matches(".lua").replace('/', ".");
        contexts.import = owner == module;
        return (None, contexts);
    }
    let (description, script) = (file.starts_with("description/"), file.starts_with("script/"));
    if file.starts_with("script/instances/") {
        contexts.instance = method && owner == stem && Instance::parse(stem).is_some();
    } else if (description || script) && (owner == stem || (stem == "globals" && owner == GLOBALS))
    {
        contexts.description = description;
        contexts.script = script;
    }
    (None, contexts)
}

/// `text[range]` with the static lifetime of the stub text.
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
    fields: Vec<Field>,
    see: Option<&'static str>,
    extra: Vec<&'static str>,
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
            if line.starts_with('|') {
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
            "param" | "field" => {
                // `---@field public public? any`: a visibility, then a field
                // named like one.
                let rest = match rest.split_once(char::is_whitespace) {
                    Some((visibility, after))
                        if tag == "field"
                            && matches!(
                                visibility,
                                "public" | "protected" | "private" | "package"
                            )
                            && after.trim().contains(char::is_whitespace) =>
                    {
                        after.trim()
                    }
                    _ => rest,
                };
                let (name, rest) = rest.split_once(char::is_whitespace).unwrap_or((rest, "any"));
                let (ty, doc) = split_type(rest.trim());
                let optional = name.ends_with('?');
                let name = name.trim_end_matches('?');
                let doc = doc.trim();
                if tag == "param" {
                    let literals = split_union(ty)
                        .into_iter()
                        .map(|p| p.trim().trim_end_matches("[]"))
                        .filter(|p| p.len() >= 2 && p.starts_with('"') && p.ends_with('"'))
                        .map(|p| (&p[1..p.len() - 1], ""))
                        .collect();
                    self.params.push(Param { name, ty, optional, doc, literals });
                } else {
                    self.fields.push(Field { name, ty, optional, doc });
                }
            }
            "return" => {
                let (ty, rest) = split_type(rest);
                let name = rest.split_whitespace().next().unwrap_or("");
                self.returns.push(Return { ty, name });
            }
            "see" => self.see = Some(rest).filter(|s| !s.is_empty()),
            // Of the file, not of a stub.
            "meta" => {}
            _ => self.extra.push(annotation.trim_end()),
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

/// A LuaCATS type in the notation of the manual, which the prototypes use:
/// `string|string[]` is `string|array`, `fun(target: target)` is
/// `function (target)`. The aliases of values (`xmake.value.kind`) and the
/// literals (`"binary"`) are kept: they tell the values.
pub fn manual_type(ty: &str) -> String {
    let mut parts: Vec<String> = Vec::new();
    // `fun(target: target)|string`: a script, or its name; the manual says
    // `function (target)`.
    let script = split_union(ty).iter().any(|p| p.trim().starts_with("fun("));
    for part in split_union(ty) {
        let part = part.trim();
        if script && part == "string" {
            continue;
        }
        // An options table is no value: see [`Stubs::prototype`].
        if part.starts_with('{') || part.starts_with("xmake.options.") {
            continue;
        }
        let manual = if part.ends_with("[]") {
            "array".to_owned()
        } else if let Some(params) = part.strip_prefix("fun(") {
            let inner = params.split(')').next().unwrap_or("");
            let names: Vec<&str> = inner
                .split(',')
                .map(|p| p.split(':').next().unwrap_or("").trim())
                .filter(|p| !p.is_empty())
                .collect();
            if names.is_empty() {
                "function".to_owned()
            } else {
                format!("function ({})", names.join(", "))
            }
        } else {
            part.to_owned()
        };
        if !parts.contains(&manual) {
            parts.push(manual);
        }
    }
    parts.join("|")
}

/// The parts of a union, `|` inside brackets kept.
pub(crate) fn split_union(ty: &str) -> Vec<&str> {
    let (mut out, mut depth, mut start) = (Vec::new(), 0i32, 0);
    for (i, c) in ty.char_indices() {
        match c {
            '(' | '{' | '[' | '<' => depth += 1,
            ')' | '}' | ']' | '>' => depth -= 1,
            '|' if depth == 0 => {
                out.push(&ty[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    out.push(&ty[start..]);
    out
}

impl Stubs {
    /// The prototype of `function` in the notation of the manual:
    /// `add_files(files: string|array, ..., {defines, languages})`; `None`
    /// without parameters.
    pub fn prototype(&self, function: &Function) -> Option<String> {
        if function.params.is_empty() {
            return None;
        }
        let mut params: Vec<String> = Vec::new();
        for p in &function.params {
            // `...: string|string[]|{defines?: any}`: values, then options.
            if p.name == "..." {
                params.push("...".to_owned());
                let options = split_union(p.ty).into_iter().find_map(|part| self.table_keys(part));
                if let Some(keys) = options {
                    params.push(format!("{{{}}}", keys.join(", ")));
                }
                continue;
            }
            let param = match self.table_keys(p.ty) {
                Some(keys) => format!("{{{}}}", keys.join(", ")),
                None => {
                    let optional = if p.optional { "?" } else { "" };
                    format!("{}{optional}: {}", p.name, manual_type(p.ty))
                }
            };
            params.push(param);
        }
        Some(format!("{}({})", function.name, params.join(", ")))
    }

    /// The keys of an options table type: a class (`xmake.options.flags`)
    /// or a table literal (`{defines?: string, force?: boolean}`).
    fn table_keys(&self, ty: &str) -> Option<Vec<String>> {
        let ty = ty.trim().trim_end_matches('?');
        if let Some(class) = self.class(ty).filter(|_| ty.starts_with("xmake.options.")) {
            return Some(class.fields.iter().map(|f| f.name.to_owned()).collect());
        }
        let inner = ty.strip_prefix('{')?.strip_suffix('}')?;
        let mut keys = Vec::new();
        let (mut depth, mut start) = (0i32, 0);
        let mut push = |part: &str| {
            let key = part.split(':').next().unwrap_or("").trim().trim_end_matches('?');
            if !key.is_empty() {
                keys.push(key.to_owned());
            }
        };
        for (i, c) in inner.char_indices() {
            match c {
                '(' | '{' | '[' | '<' => depth += 1,
                ')' | '}' | ']' | '>' => depth -= 1,
                ',' if depth == 0 => {
                    push(&inner[start..i]);
                    start = i + 1;
                }
                _ => {}
            }
        }
        push(&inner[start..]);
        Some(keys)
    }
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
        assert_eq!(manual_type("string|string[]"), "string|array");
        assert_eq!(manual_type("fun(target: target)|string"), "function (target)");
        assert_eq!(manual_type("xmake.value.kind"), "xmake.value.kind");
    }
}
