//! Typed views over the untyped syntax tree.
//!
//! Each wrapper is a `Copy` handle around a [`SyntaxNode`]; accessors return
//! `None` when the (possibly malformed) source lacks a part.

use std::borrow::Cow;

use crate::{SyntaxKind, SyntaxNode, SyntaxToken};

/// A typed node.
pub trait AstNode<'a>: Sized + Copy {
    /// Wraps `node` if it has the right kind.
    fn cast(node: SyntaxNode<'a>) -> Option<Self>;
    /// The underlying node.
    fn syntax(&self) -> SyntaxNode<'a>;
}

macro_rules! ast_nodes {
    ($($(#[$doc:meta])* $name:ident = $kind:ident;)*) => {$(
        $(#[$doc])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct $name<'a>(SyntaxNode<'a>);

        impl<'a> AstNode<'a> for $name<'a> {
            fn cast(node: SyntaxNode<'a>) -> Option<Self> {
                (node.kind() == SyntaxKind::$kind).then_some($name(node))
            }

            fn syntax(&self) -> SyntaxNode<'a> {
                self.0
            }
        }
    )*};
}

ast_nodes! {
    /// The whole file.
    SourceFile = SOURCE_FILE;
    /// A sequence of statements.
    Block = BLOCK;
    /// `f(args)`
    CallExpr = CALL_EXPR;
    /// `obj:method(args)`
    MethodCallExpr = METHOD_CALL_EXPR;
    /// The arguments of a call.
    ArgList = ARG_LIST;
    /// `function (params) body end`
    FunctionExpr = FUNCTION_EXPR;
    /// `(a, b, ...)`
    ParamList = PARAM_LIST;
    /// A declared name.
    Name = NAME;
    /// A use of a name.
    NameRef = NAME_REF;
    /// `a.b`
    FieldExpr = FIELD_EXPR;
    /// A literal.
    Literal = LITERAL;
    /// `local a, b = ...`
    LocalStmt = LOCAL_STMT;
    /// `local function f() end`
    LocalFunctionStmt = LOCAL_FUNCTION_STMT;
    /// `function a.b:c() end`
    FunctionStmt = FUNCTION_STMT;
    /// `a, b = ...`
    AssignStmt = ASSIGN_STMT;
    /// `{ ... }`
    TableExpr = TABLE_EXPR;
    /// One field of a table constructor.
    TableField = TABLE_FIELD;
}

/// The first descendant token of `node` that is an identifier.
fn ident_of<'a>(node: SyntaxNode<'a>) -> Option<SyntaxToken<'a>> {
    node.child_token(SyntaxKind::IDENT)
}

impl<'a> SourceFile<'a> {
    /// The top level block.
    pub fn block(&self) -> Option<Block<'a>> {
        self.0.children().find_map(Block::cast)
    }
}

impl<'a> Block<'a> {
    /// The statements, in order.
    pub fn statements(&self) -> impl Iterator<Item = SyntaxNode<'a>> + use<'a> {
        self.0.children()
    }
}

impl<'a> CallExpr<'a> {
    /// The called expression.
    pub fn callee(&self) -> Option<SyntaxNode<'a>> {
        self.0.children().next()
    }

    /// The name of the called function when it is a plain name: `target` in
    /// `target("foo")`.
    pub fn callee_name(&self) -> Option<&'a str> {
        let callee = NameRef::cast(self.callee()?)?;
        Some(callee.text())
    }

    /// The arguments.
    pub fn arg_list(&self) -> Option<ArgList<'a>> {
        self.0.children().find_map(ArgList::cast)
    }

    /// The `index`-th argument.
    pub fn arg(&self, index: usize) -> Option<SyntaxNode<'a>> {
        self.arg_list()?.args().nth(index)
    }
}

impl<'a> MethodCallExpr<'a> {
    /// The object the method is called on.
    pub fn receiver(&self) -> Option<SyntaxNode<'a>> {
        self.0.children().next()
    }

    /// The method name.
    pub fn method(&self) -> Option<SyntaxToken<'a>> {
        ident_of(self.0)
    }

    /// The arguments.
    pub fn arg_list(&self) -> Option<ArgList<'a>> {
        self.0.children().find_map(ArgList::cast)
    }
}

impl<'a> ArgList<'a> {
    /// The argument expressions. A table or string argument without
    /// parentheses is the only argument.
    pub fn args(&self) -> impl Iterator<Item = SyntaxNode<'a>> + use<'a> {
        self.0.children().filter(|n| n.kind() != SyntaxKind::ERROR)
    }
}

impl<'a> FunctionExpr<'a> {
    /// The parameters.
    pub fn param_list(&self) -> Option<ParamList<'a>> {
        self.0.children().find_map(ParamList::cast)
    }

    /// The body.
    pub fn body(&self) -> Option<Block<'a>> {
        self.0.children().find_map(Block::cast)
    }
}

impl<'a> ParamList<'a> {
    /// The named parameters (`...` excluded).
    pub fn params(&self) -> impl Iterator<Item = Name<'a>> + use<'a> {
        self.0.children().filter_map(Name::cast)
    }
}

impl<'a> Name<'a> {
    /// The identifier token.
    pub fn ident(&self) -> Option<SyntaxToken<'a>> {
        ident_of(self.0)
    }

    /// The name, empty when missing.
    pub fn text(&self) -> &'a str {
        self.ident().map_or("", |t| t.text())
    }
}

impl<'a> NameRef<'a> {
    /// The identifier token.
    pub fn ident(&self) -> Option<SyntaxToken<'a>> {
        ident_of(self.0)
    }

    /// The name, empty when missing.
    pub fn text(&self) -> &'a str {
        self.ident().map_or("", |t| t.text())
    }
}

impl<'a> FieldExpr<'a> {
    /// The expression before the dot.
    pub fn base(&self) -> Option<SyntaxNode<'a>> {
        self.0.children().next()
    }

    /// The field name after the dot.
    pub fn field(&self) -> Option<SyntaxToken<'a>> {
        ident_of(self.0)
    }
}

impl<'a> Literal<'a> {
    /// The literal token.
    pub fn token(&self) -> Option<SyntaxToken<'a>> {
        self.0.first_token()
    }

    /// The value of a string literal, unescaped. Borrows the source unless
    /// the literal contains escape sequences.
    pub fn string_value(&self) -> Option<Cow<'a, str>> {
        let token = self.token()?;
        match token.kind() {
            SyntaxKind::STRING => Some(unescape_quoted(token.text())),
            SyntaxKind::LONG_STRING => Some(Cow::Borrowed(long_string_body(token.text()))),
            _ => None,
        }
    }
}

impl<'a> LocalStmt<'a> {
    /// The declared names.
    pub fn names(&self) -> impl Iterator<Item = Name<'a>> + use<'a> {
        let list = self.0.child_node(SyntaxKind::NAME_LIST);
        list.into_iter().flat_map(|l| l.children().filter_map(Name::cast))
    }

    /// The initializers.
    pub fn values(&self) -> impl Iterator<Item = SyntaxNode<'a>> + use<'a> {
        let list = self.0.child_node(SyntaxKind::EXPR_LIST);
        list.into_iter().flat_map(|l| l.children())
    }
}

impl<'a> LocalFunctionStmt<'a> {
    /// The function name.
    pub fn name(&self) -> Option<Name<'a>> {
        self.0.children().find_map(Name::cast)
    }

    /// The body.
    pub fn body(&self) -> Option<Block<'a>> {
        self.0.children().find_map(Block::cast)
    }
}

impl<'a> FunctionStmt<'a> {
    /// The tokens of `a.b:c`.
    pub fn name_tokens(&self) -> impl Iterator<Item = SyntaxToken<'a>> + use<'a> {
        let name = self.0.child_node(SyntaxKind::FUNC_NAME);
        name.into_iter().flat_map(|n| n.child_tokens().filter(|t| t.kind() == SyntaxKind::IDENT))
    }

    /// The parameters.
    pub fn param_list(&self) -> Option<ParamList<'a>> {
        self.0.children().find_map(ParamList::cast)
    }

    /// The body.
    pub fn body(&self) -> Option<Block<'a>> {
        self.0.children().find_map(Block::cast)
    }
}

impl<'a> TableExpr<'a> {
    /// The fields.
    pub fn fields(&self) -> impl Iterator<Item = TableField<'a>> + use<'a> {
        self.0.children().filter_map(TableField::cast)
    }
}

impl<'a> TableField<'a> {
    /// `name` in `name = value`.
    pub fn key_name(&self) -> Option<SyntaxToken<'a>> {
        let has_eq = self.0.child_token(SyntaxKind::EQ).is_some();
        let named = self.0.child_token(SyntaxKind::L_BRACK).is_none();
        if has_eq && named { ident_of(self.0) } else { None }
    }

    /// The value expression.
    pub fn value(&self) -> Option<SyntaxNode<'a>> {
        self.0.children().last()
    }
}

/// The body of `[==[body]==]`, without the first line break, as Lua does.
fn long_string_body(text: &str) -> &str {
    let level = text[1..].bytes().take_while(|&b| b == b'=').count();
    let open = level + 2;
    let close = if text.len() >= open * 2 && text.ends_with(']') { open } else { 0 };
    let body = text.get(open..text.len() - close).unwrap_or("");
    body.strip_prefix("\r\n").or_else(|| body.strip_prefix('\n')).unwrap_or(body)
}

/// Unescapes a quoted string literal, quotes included in `text`.
///
/// Borrows when there is nothing to unescape, which is the common case.
pub fn unescape_quoted(text: &str) -> Cow<'_, str> {
    let quote = text.chars().next().unwrap_or('"');
    let inner = text.strip_prefix(quote).unwrap_or(text);
    let inner = inner.strip_suffix(quote).unwrap_or(inner);
    if !inner.contains('\\') {
        return Cow::Borrowed(inner);
    }

    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('r') => out.push('\r'),
            Some('a') => out.push('\x07'),
            Some('b') => out.push('\x08'),
            Some('f') => out.push('\x0c'),
            Some('v') => out.push('\x0b'),
            Some('z') => while chars.next_if(|c| c.is_whitespace()).is_some() {},
            Some('x') => {
                let hex: String =
                    (0..2).filter_map(|_| chars.next_if(char::is_ascii_hexdigit)).collect();
                out.extend(u8::from_str_radix(&hex, 16).ok().map(char::from));
            }
            Some(d) if d.is_ascii_digit() => {
                let mut value = d.to_digit(10).unwrap_or(0);
                for _ in 0..2 {
                    match chars.next_if(char::is_ascii_digit) {
                        Some(d) => value = value * 10 + d.to_digit(10).unwrap_or(0),
                        None => break,
                    }
                }
                out.extend(char::from_u32(value));
            }
            Some(other) => out.push(other),
            None => {}
        }
    }
    Cow::Owned(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unescape_borrows_when_possible() {
        assert!(matches!(unescape_quoted("\"abc\""), Cow::Borrowed("abc")));
        assert_eq!(unescape_quoted(r#""a\tb\"c\65""#), "a\tb\"cA");
        assert_eq!(unescape_quoted("'unterminated"), "unterminated");
    }

    #[test]
    fn long_strings() {
        assert_eq!(long_string_body("[[\nabc]]"), "abc");
        assert_eq!(long_string_body("[==[a]]b]==]"), "a]]b");
        assert_eq!(long_string_body("[[open"), "open");
    }
}
