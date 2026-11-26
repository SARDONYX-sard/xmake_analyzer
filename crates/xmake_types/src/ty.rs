//! Structured types, as LuaCATS writes them and as Lua evaluates them.
//!
//! Types display in the notation of LuaCATS, as LuaLS (sumneko) shows them:
//! `string?`, `string|any[]`, `table<string, target>`, `fun(): string`. The
//! informal `array` of the manual of xmake is `any[]`.
//!
//! ```lua
//! ---@return string?        -- string|nil
//! ---@param files string|array
//! ---@return table<string, target>
//! ---@return fun(): string  -- an iterator
//! ```
//!
//! Unions follow TypeScript: a union is assignable to a type when all its
//! members are, and a type is assignable to a union when it is assignable to
//! one of its members. The operators follow Lua:
//! - only `nil` and `false` are falsy: `0` and `""` are truthy;
//! - `a or b` is `a` when truthy, else `b`: `truthy(a) | b`;
//! - `a and b` is `a` when falsy, else `b`: `falsy(a) | b`;
//! - arithmetic gives numbers, `..` strings, comparisons and `not` booleans,
//!   `#` and the bitwise operators integers.
//!
//! `integer` is the subtype of `number` whose values are integral, as in Lua
//! 5.3+ and in the annotations of LuaLS.

use std::{borrow::Cow, fmt};

use xmake_api::Stubs;

/// A type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type<'a> {
    /// Not inferred: no check, no hint.
    Unknown,
    /// Declared as anything: `any`.
    Any,
    /// No value: `nil` of `nil`, the truthy part of `false`.
    Never,
    Nil,
    Boolean,
    /// `true`: the truthy part of `boolean`.
    True,
    /// `false`: the falsy part of `boolean`.
    False,
    Integer,
    Number,
    String,
    /// A function, with what follows `fun` in its LuaCATS type when known:
    /// `(a: string): integer`.
    Function(Option<&'a str>),
    /// Any table.
    Table,
    /// A table with known fields, as written: `{rootdir?: string}`.
    Shape(&'a str),
    /// A list: `string[]`.
    Array(Box<Type<'a>>),
    /// `table<K, V>`.
    Map(Box<Type<'a>>, Box<Type<'a>>),
    /// An instance of a class of the stubs: `target`, `semver`, ...
    Class(&'a str),
    /// A module table: `os`, `core.base.json`, ...
    Module(&'a str),
    /// A string literal: `"c++20"` of `---@param std "c++20"|"c++23"`.
    Literal(&'a str),
    /// A name not understood (`file`, `V`): accepted everywhere.
    Named(&'a str),
    /// Normalized: flat, without duplicates, at least two members, neither
    /// `Unknown` nor `Any` (they absorb the union).
    Union(Vec<Type<'a>>),
}

impl<'a> Type<'a> {
    /// The type of a LuaCATS annotation.
    pub fn parse(text: &'a str) -> Type<'a> {
        let text = text.trim();
        // `fun(a): b|c` returns `b|c`.
        if let Some(signature) = function_signature(text) {
            return Type::Function(signature);
        }
        let parts = split_top(text, b'|');
        if parts.len() > 1 {
            return Type::union(parts.into_iter().map(Type::parse));
        }
        if let Some(inner) = text.strip_suffix('?') {
            return Type::union([Type::parse(inner), Type::Nil]);
        }
        if let Some(inner) = text.strip_suffix("[]") {
            return Type::Array(Box::new(Type::parse(inner)));
        }
        if text.starts_with('(') && closing(text, 0) == Some(text.len() - 1) {
            return Type::parse(&text[1..text.len() - 1]);
        }
        if text.starts_with('{') {
            return Type::Shape(text);
        }
        if let Some(inner) = text.strip_prefix("table<").and_then(|t| t.strip_suffix('>'))
            && let [key, value] = split_top(inner, b',')[..]
        {
            return Type::Map(Box::new(Type::parse(key)), Box::new(Type::parse(value)));
        }
        match text {
            "" | "unknown" => Type::Unknown,
            "any" | "..." => Type::Any,
            "nil" | "void" => Type::Nil,
            "boolean" | "bool" => Type::Boolean,
            "true" => Type::True,
            "false" => Type::False,
            "integer" => Type::Integer,
            "number" => Type::Number,
            "string" => Type::String,
            "table" => Type::Table,
            "array" => Type::Array(Box::new(Type::Any)),
            _ if text.len() >= 2
                && (text.starts_with('"') && text.ends_with('"')
                    || text.starts_with('\'') && text.ends_with('\'')) =>
            {
                Type::Literal(&text[1..text.len() - 1])
            }
            _ if text.bytes().all(|b| b.is_ascii_digit()) => Type::Integer,
            _ if Stubs::get().class(text).is_some() => Type::Class(text),
            // The values of the stubs (`xmake.value.kind`): strings.
            _ if Stubs::get().alias(text).is_some() => Type::String,
            _ => Type::Named(text),
        }
    }

    /// The union of `types`, normalized.
    pub fn union(types: impl IntoIterator<Item = Type<'a>>) -> Type<'a> {
        fn add<'a>(out: &mut Vec<Type<'a>>, ty: Type<'a>) {
            match ty {
                Type::Never => {}
                Type::Union(members) => members.into_iter().for_each(|m| add(out, m)),
                ty if !out.contains(&ty) => out.push(ty),
                _ => {}
            }
        }
        let mut members = Vec::new();
        for ty in types {
            add(&mut members, ty);
        }
        for absorbing in [Type::Unknown, Type::Any] {
            if members.contains(&absorbing) {
                return absorbing;
            }
        }
        let has = |members: &[Type<'_>], ty: Type<'_>| members.contains(&ty);
        if has(&members, Type::Number) {
            members.retain(|m| *m != Type::Integer);
        }
        if has(&members, Type::True) && has(&members, Type::False) {
            members.retain(|m| !matches!(m, Type::True | Type::False));
            members.push(Type::Boolean);
        }
        // `string` holds every literal.
        if has(&members, Type::String) {
            members.retain(|m| !matches!(m, Type::Literal(_)));
        }
        if has(&members, Type::Boolean) {
            members.retain(|m| !matches!(m, Type::True | Type::False));
        }
        match members.len() {
            0 => Type::Never,
            1 => members.pop().unwrap_or(Type::Never),
            _ => Type::Union(members),
        }
    }

    /// Each member of a union, or the type itself.
    pub fn members(&self) -> &[Type<'a>] {
        match self {
            Type::Union(members) => members,
            Type::Never => &[],
            other => std::slice::from_ref(other),
        }
    }

    fn map(&self, f: impl Fn(&Type<'a>) -> Type<'a>) -> Type<'a> {
        match self {
            Type::Union(members) => Type::union(members.iter().map(f)),
            other => f(other),
        }
    }

    /// The values that are truthy: `string` of `string?`, `true` of
    /// `boolean`.
    pub fn truthy(&self) -> Type<'a> {
        self.map(|ty| match ty {
            Type::Nil | Type::False => Type::Never,
            Type::Boolean => Type::True,
            other => other.clone(),
        })
    }

    /// The values that are falsy: `nil` of `string?`, `false` of `boolean`.
    pub fn falsy(&self) -> Type<'a> {
        self.map(|ty| match ty {
            Type::Unknown | Type::Any | Type::Named(_) => ty.clone(),
            Type::Nil => Type::Nil,
            Type::False | Type::Boolean => Type::False,
            _ => Type::Never,
        })
    }

    /// `self or other`: `self` when truthy, `other` otherwise.
    pub fn or(&self, other: Type<'a>) -> Type<'a> {
        match self.falsy() {
            Type::Never => self.clone(),
            _ => Type::union([self.truthy(), other]),
        }
    }

    /// `self and other`: `self` when falsy, `other` otherwise.
    pub fn and(&self, other: Type<'a>) -> Type<'a> {
        match self.truthy() {
            Type::Never => self.clone(),
            _ => Type::union([self.falsy(), other]),
        }
    }

    /// The type without `nil`: `target` of `target?`.
    pub fn non_nil(&self) -> Type<'a> {
        self.map(|ty| if *ty == Type::Nil { Type::Never } else { ty.clone() })
    }

    /// Whether the type tells nothing: no hint, no check.
    pub fn is_unknown(&self) -> bool {
        matches!(self, Type::Unknown | Type::Any | Type::Named(_))
    }

    /// Whether a value of `self` can be given where `to` is expected.
    pub fn is_assignable_to(&self, to: &Type<'_>) -> bool {
        use Type as T;
        match (self, to) {
            (T::Unknown | T::Any | T::Named(_) | T::Never, _)
            | (_, T::Unknown | T::Any | T::Named(_)) => true,
            (T::Union(members), _) => members.iter().all(|m| m.is_assignable_to(to)),
            (_, T::Union(members)) => members.iter().any(|m| self.is_assignable_to(m)),
            (T::True | T::False, T::Boolean) | (T::Integer, T::Number) => true,
            (T::Literal(_), T::String) => true,
            (T::Literal(a), T::Literal(b)) => a == b,
            // A string whose value is not known: no complaint, unlike
            // TypeScript; only literals are compared.
            (T::String, T::Literal(_)) => true,
            // `on_run("main")`: xmake takes the name of a module for a script.
            (T::String | T::Literal(_) | T::Function(_), T::Function(_)) => true,
            (T::Array(from), T::Array(to)) => from.is_assignable_to(to),
            (T::Table | T::Shape(_), T::Array(_) | T::Map(..)) => true,
            (T::Array(element), T::Map(key, value)) => {
                T::Integer.is_assignable_to(key) && element.is_assignable_to(value)
            }
            (T::Map(from_key, from_value), T::Map(key, value)) => {
                from_key.is_assignable_to(key) && from_value.is_assignable_to(value)
            }
            (
                T::Table | T::Shape(_) | T::Array(_) | T::Map(..) | T::Class(_) | T::Module(_),
                T::Table | T::Shape(_),
            ) => true,
            (T::Class(a), T::Class(b)) | (T::Module(a), T::Module(b)) => a == b,
            (from, to) => from.same_kind(to),
        }
    }

    fn same_kind(&self, other: &Type<'_>) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
            && matches!(
                self,
                Type::Nil
                    | Type::Boolean
                    | Type::True
                    | Type::False
                    | Type::Integer
                    | Type::Number
                    | Type::String
            )
    }

    /// The owner of the members in the stubs: `string` for strings, the class
    /// of `target?`, ...
    pub fn owner(&self) -> Option<&'a str> {
        match self.non_nil() {
            Type::String | Type::Literal(_) => Some("string"),
            Type::Class(name) | Type::Module(name) => Some(name),
            _ => None,
        }
    }

    /// The class of an instance, `nil` aside.
    pub fn class(&self) -> Option<&'a str> {
        match self.non_nil() {
            Type::Class(name) => Some(name),
            _ => None,
        }
    }

    /// The module of a module table.
    pub fn module(&self) -> Option<&'a str> {
        match self {
            Type::Module(name) => Some(name),
            _ => None,
        }
    }

    /// The element of a list: `string` of `string[]`.
    pub fn element(&self) -> Type<'a> {
        match self.non_nil() {
            Type::Array(element) => *element,
            Type::Map(_, value) => *value,
            _ => Type::Unknown,
        }
    }

    /// The values a call of a function gives: `string` of `fun(): string`.
    pub fn returns(&self) -> Vec<Type<'a>> {
        let Type::Function(Some(signature)) = self else {
            return Vec::new();
        };
        let Some(close) = closing(signature, 0) else {
            return Vec::new();
        };
        let Some(returns) = signature[close + 1..].trim_start().strip_prefix(':') else {
            return Vec::new();
        };
        split_top(returns, b',').into_iter().map(Type::parse).collect()
    }

    /// Literal booleans widened: a variable holding `true` is a `boolean`.
    pub fn widen(&self) -> Type<'a> {
        self.map(|ty| match ty {
            Type::True | Type::False => Type::Boolean,
            Type::Literal(_) => Type::String,
            other => other.clone(),
        })
    }

    /// The type as text, borrowed when possible.
    pub fn text(&self) -> Cow<'a, str> {
        Cow::Borrowed(match self {
            Type::Unknown => "unknown",
            Type::Any => "any",
            // LuaCATS has no empty type: nothing but `nil` comes out.
            Type::Never | Type::Nil => "nil",
            Type::Boolean => "boolean",
            Type::True => "true",
            Type::False => "false",
            Type::Integer => "integer",
            Type::Number => "number",
            Type::String => "string",
            Type::Function(None) => "function",
            Type::Table => "table",
            // Modules are classes in the stubs: `---@class os`.
            Type::Shape(text) | Type::Class(text) | Type::Module(text) | Type::Named(text) => text,
            _ => return Cow::Owned(self.to_string()),
        })
    }
}

impl fmt::Display for Type<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Function(Some(signature)) => write!(f, "fun{signature}"),
            Type::Literal(value) => write!(f, "\"{value}\""),
            Type::Array(element) => match **element {
                Type::Union(_) | Type::Function(Some(_)) => write!(f, "({element})[]"),
                _ => write!(f, "{element}[]"),
            },
            Type::Map(key, value) => write!(f, "table<{key}, {value}>"),
            Type::Union(members) => {
                // `T?` for `T|nil`.
                let others: Vec<_> = members.iter().filter(|m| **m != Type::Nil).collect();
                if let [single] = others[..]
                    && others.len() < members.len()
                    && !matches!(single, Type::Function(Some(_)))
                {
                    return write!(f, "{single}?");
                }
                for (i, member) in members.iter().enumerate() {
                    let separator = if i == 0 { "" } else { "|" };
                    match member {
                        Type::Function(Some(_)) => write!(f, "{separator}({member})")?,
                        _ => write!(f, "{separator}{member}")?,
                    }
                }
                Ok(())
            }
            other => f.write_str(&other.text()),
        }
    }
}

/// A parameter of a prototype of the manual in the notation of LuaCATS:
/// `files: string|array` is `files: string|any[]`, and the scripts
/// `script: function (target)` are `script: fun(target: target)|string`. `...`
/// and the tables of options (`{defines, ...}`) stay as written.
pub fn luacats_param(param: &str) -> Cow<'_, str> {
    let Some((name, ty)) = param.split_once(':').filter(|_| !param.starts_with('{')) else {
        return Cow::Borrowed(param);
    };
    if let Some(script) = script_type(ty) {
        return Cow::Owned(format!("{}: {script}", name.trim()));
    }
    let normalized = Type::parse(ty).to_string();
    if normalized == ty.trim() {
        return Cow::Borrowed(param);
    }
    Cow::Owned(format!("{}: {normalized}", name.trim()))
}

/// `function (target, sourcefile, opt)` of the manual, a script of xmake, as
/// LuaCATS: `fun(target: target, sourcefile: string, opt: table)|string`. The
/// manual names the parameters only; their types follow from the names. A
/// script may also be given by name: xmake loads the module (`on_run("main")`).
fn script_type(ty: &str) -> Option<String> {
    let params = ty.trim().strip_prefix("function")?.trim_start();
    let params = params.strip_prefix('(')?.strip_suffix(')')?;
    let mut out = String::from("fun(");
    for (i, name) in params.split(',').map(str::trim).filter(|n| !n.is_empty()).enumerate() {
        if name.contains(':') {
            // Already typed.
            return None;
        }
        let ty = match name {
            "sourcefile" => "string",
            "sourcebatch" | "opt" => "table",
            _ if Stubs::get().class(name).is_some() => name,
            _ => "any",
        };
        let separator = if i == 0 { "" } else { ", " };
        out.push_str(&format!("{separator}{name}: {ty}"));
    }
    out.push_str(")|string");
    Some(out)
}

/// A prototype of the manual in the notation of LuaCATS:
/// `add_files(files: string|any[], ...)`.
pub fn luacats_prototype(prototype: &str) -> Cow<'_, str> {
    let params = xmake_api::prototype_params(prototype);
    if params.iter().all(|p| matches!(luacats_param(p), Cow::Borrowed(_))) {
        return Cow::Borrowed(prototype);
    }
    let name = prototype.split('(').next().unwrap_or(prototype);
    let params: Vec<_> = params.into_iter().map(luacats_param).collect();
    Cow::Owned(format!("{name}({})", params.join(", ")))
}

/// What follows `fun` / `function` in a function type: `Some("(a): b")`;
/// `Some(None)` for a bare `function`.
fn function_signature(text: &str) -> Option<Option<&str>> {
    if text == "function" {
        return Some(None);
    }
    let rest = text.strip_prefix("function").or_else(|| text.strip_prefix("fun"))?;
    let rest = rest.trim_start();
    rest.starts_with('(').then_some(Some(rest))
}

/// The index of the bracket closing the one at `open`.
fn closing(text: &str, open: usize) -> Option<usize> {
    let mut depth = 0i32;
    for (i, b) in text.bytes().enumerate().skip(open) {
        match b {
            b'(' | b'{' | b'<' | b'[' => depth += 1,
            b')' | b'}' | b'>' | b']' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

/// `text` split at `separator` outside brackets.
fn split_top(text: &str, separator: u8) -> Vec<&str> {
    let mut parts = Vec::new();
    let (mut depth, mut start) = (0i32, 0);
    for (i, b) in text.bytes().enumerate() {
        match b {
            b'(' | b'{' | b'<' | b'[' => depth += 1,
            b')' | b'}' | b'>' | b']' => depth -= 1,
            _ if b == separator && depth == 0 => {
                parts.push(text[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }
    parts.push(text[start..].trim());
    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    fn show(text: &str) -> String {
        Type::parse(text).to_string()
    }

    fn assignable(from: &str, to: &str) -> bool {
        Type::parse(from).is_assignable_to(&Type::parse(to))
    }

    #[test]
    fn parsing() {
        assert_eq!(show("string|nil"), "string?");
        assert_eq!(show("target?"), "target?");
        assert_eq!(show("string|string[]"), "string|string[]");
        assert_eq!(show("(string|integer)[]"), "(string|integer)[]");
        assert_eq!(show("table<string, target>"), "table<string, target>");
        assert_eq!(show("integer|number"), "number");
        assert_eq!(show("true|false"), "boolean");
        assert_eq!(show("string|any"), "any");
        assert_eq!(show("fun(): string"), "fun(): string");
        assert_eq!(show("function (target)"), "fun(target)");
        assert_eq!(show("{rootdir?: string}"), "{rootdir?: string}");
        assert_eq!(Type::parse("fun(): integer, V").returns(), [Type::Integer, Type::Named("V")]);
        assert_eq!(Type::parse("string[]?").element(), Type::String);
        assert_eq!(
            luacats_prototype("add_files(files: string|array, ..., {defines, rules})"),
            "add_files(files: string|any[], ..., {defines, rules})"
        );
        assert!(matches!(luacats_prototype("set_default(default: boolean)"), Cow::Borrowed(_)));
        assert_eq!(
            luacats_prototype("on_build(script: function (target))"),
            "on_build(script: fun(target: target)|string)"
        );
        assert_eq!(
            luacats_param("script: function (target, batchcmds, sourcefile, opt)"),
            "script: fun(target: target, batchcmds: any, sourcefile: string, opt: table)|string"
        );
    }

    #[test]
    fn lua_truthiness() {
        let ty = Type::parse("string|boolean|nil");
        assert_eq!(ty.truthy().to_string(), "string|true");
        assert_eq!(ty.falsy().to_string(), "false?");
        // `0` and `""` are truthy in Lua.
        assert_eq!(Type::Integer.falsy(), Type::Never);
        assert_eq!(Type::String.truthy(), Type::String);
    }

    #[test]
    fn assignability() {
        assert!(assignable("string[]", "string|array"));
        assert!(assignable("integer", "number"));
        assert!(!assignable("number", "integer"));
        assert!(assignable("function", "function (target)"));
        assert!(assignable("string", "function (target)"), "a module name");
        assert!(assignable("{}", "{rootdir?: string}"));
        assert!(assignable("target", "table"));
        assert!(!assignable("target", "package"));
        assert!(!assignable("target?", "target"));
        assert!(assignable("target?", "target|nil"));
        assert!(assignable("string|integer", "string|number"));
        assert!(!assignable("string|boolean", "string"));
        assert!(assignable("string", "weird_type"));
        assert!(assignable("integer[]", "table<integer, number>"));
        assert!(assignable("\"c++20\"", "\"c++20\"|\"c++23\""));
        assert!(!assignable("\"c++17\"", "\"c++20\"|\"c++23\""));
        assert!(assignable("\"c++17\"", "string"));
        assert_eq!(show("string|\"a\""), "string");
        assert_eq!(show("'a'|\"b\""), "\"a\"|\"b\"");
        assert!(!assignable("string[]", "integer[]"));
    }
}
