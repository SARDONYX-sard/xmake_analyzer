//! The annotations of LuaCATS, as LuaLS reads them:
//!
//! ```lua
//! ---@param path string
//! ---@param opt? {force?: boolean}
//! ---@return boolean ok
//! local function copy(path, opt) end
//!
//! ---@type string[]
//! local names = {}
//! ```
//!
//! An annotation is a line comment starting with `---@`; a block of them
//! documents the statement right below.

use xmake_syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken, TextRange, TextSize};

/// The tags of LuaCATS.
pub const TAGS: &[&str] = &[
    "alias",
    "as",
    "async",
    "cast",
    "class",
    "deprecated",
    "diagnostic",
    "enum",
    "field",
    "generic",
    "meta",
    "module",
    "nodiscard",
    "operator",
    "overload",
    "package",
    "param",
    "private",
    "protected",
    "return",
    "see",
    "source",
    "type",
    "vararg",
    "version",
];

/// One annotation: `---@param path string`.
#[derive(Clone, Copy, Debug)]
pub struct Annotation<'a> {
    pub comment: SyntaxToken<'a>,
    /// `param`
    pub tag: &'a str,
    pub tag_range: TextRange,
    /// What follows the tag, trimmed: `path string`.
    pub args: &'a str,
    /// Where `args` starts in the file.
    pub args_start: TextSize,
}

impl<'a> Annotation<'a> {
    /// The annotation of a line comment, when it is one.
    pub fn parse(comment: SyntaxToken<'a>) -> Option<Annotation<'a>> {
        if comment.kind() != SyntaxKind::LINE_COMMENT {
            return None;
        }
        let text = comment.text();
        let body = text.strip_prefix("---")?.trim_start();
        let after_at = body.strip_prefix('@')?;
        let tag_len = after_at
            .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '-'))
            .unwrap_or(after_at.len());
        let tag = &after_at[..tag_len];
        let rest = &after_at[tag_len..];
        let args = rest.trim();
        let start = comment.text_range().start();
        let offset_of = |part: &str| {
            start + TextSize::new((part.as_ptr() as usize - text.as_ptr() as usize) as u32)
        };
        let tag_start = offset_of(tag);
        Some(Annotation {
            comment,
            tag,
            tag_range: TextRange::at(tag_start, TextSize::new(tag.len() as u32)),
            args,
            args_start: offset_of(rest)
                + TextSize::new((rest.len() - rest.trim_start().len()) as u32),
        })
    }

    /// The first word of the arguments and the rest: `("path", "string")`
    /// for `---@param path string`, with the range of the word.
    pub fn name_and_rest(&self) -> Option<(&'a str, TextRange, &'a str)> {
        let end = self.args.find(char::is_whitespace).unwrap_or(self.args.len());
        let name = &self.args[..end];
        (!name.is_empty()).then(|| {
            let range = TextRange::at(self.args_start, TextSize::new(end as u32));
            (name, range, self.args[end..].trim_start())
        })
    }

    /// Where `part`, a slice of `args`, is in the file.
    pub fn range_of(&self, part: &str) -> TextRange {
        let offset = part.as_ptr() as usize - self.args.as_ptr() as usize;
        TextRange::at(
            self.args_start + TextSize::new(offset as u32),
            TextSize::new(part.len() as u32),
        )
    }
}

/// Every annotation of the tree under `root`.
pub fn all(root: SyntaxNode<'_>) -> impl Iterator<Item = Annotation<'_>> {
    root.descendants_with_tokens().filter_map(|e| match e {
        SyntaxElement::Token(token) => Annotation::parse(token),
        SyntaxElement::Node(_) => None,
    })
}

/// The annotations right above `node`, in order.
pub fn above(node: SyntaxNode<'_>) -> Vec<Annotation<'_>> {
    let Some(first) = node.first_token() else {
        return Vec::new();
    };
    let mut out: Vec<_> = crate::doc_comment::comments_above(first)
        .into_iter()
        .filter_map(Annotation::parse)
        .collect();
    out.reverse();
    out
}

/// The statement the comment block of `comment` documents: the first
/// statement below it, when nothing but comments separate them.
pub fn documented_statement(comment: SyntaxToken<'_>) -> Option<SyntaxNode<'_>> {
    let mut next = comment.next_token();
    while let Some(token) = next.filter(|t| t.kind().is_trivia()) {
        // A blank line ends the block.
        if token.kind() == SyntaxKind::WHITESPACE && token.text().matches('\n').count() > 1 {
            return None;
        }
        next = token.next_token();
    }
    let first = next?;
    let statement = first.ancestors().find(|n| {
        matches!(
            n.kind(),
            SyntaxKind::FUNCTION_STMT
                | SyntaxKind::LOCAL_FUNCTION_STMT
                | SyntaxKind::LOCAL_STMT
                | SyntaxKind::ASSIGN_STMT
                | SyntaxKind::RETURN_STMT
                | SyntaxKind::CALL_STMT
        )
    })?;
    (statement.trimmed_range().start() == first.text_range().start()).then_some(statement)
}

/// The parameters of the function `statement` declares: `function f(a)`,
/// `local function f(a)`, `local f = function (a) end`.
pub fn function_params(statement: SyntaxNode<'_>) -> Option<SyntaxNode<'_>> {
    let function = match statement.kind() {
        SyntaxKind::FUNCTION_STMT | SyntaxKind::LOCAL_FUNCTION_STMT | SyntaxKind::FUNCTION_EXPR => {
            statement
        }
        SyntaxKind::LOCAL_STMT | SyntaxKind::ASSIGN_STMT => statement
            .child_node(SyntaxKind::EXPR_LIST)?
            .children()
            .find(|n| n.kind() == SyntaxKind::FUNCTION_EXPR)?,
        _ => return None,
    };
    function.child_node(SyntaxKind::PARAM_LIST)
}

/// The names of the parameters in `params`, `...` included.
pub fn param_names(params: SyntaxNode<'_>) -> impl Iterator<Item = SyntaxToken<'_>> {
    let names = params.children().filter_map(|n| n.first_token());
    names.chain(params.child_token(SyntaxKind::DOT3))
}

/// The statement declaring the function whose parameter list is `params`.
pub fn statement_of_params(params: SyntaxNode<'_>) -> Option<SyntaxNode<'_>> {
    let function = params.parent()?;
    match function.kind() {
        SyntaxKind::FUNCTION_STMT | SyntaxKind::LOCAL_FUNCTION_STMT => Some(function),
        // `local f = function (a) end`
        SyntaxKind::FUNCTION_EXPR => {
            let statement = function.parent()?.parent()?;
            matches!(statement.kind(), SyntaxKind::LOCAL_STMT | SyntaxKind::ASSIGN_STMT)
                .then_some(statement)
        }
        _ => None,
    }
}

/// The type given to the parameter `name` by the `---@param` annotations
/// above `statement`, with whether it is optional (`name?`).
pub fn param_type<'a>(statement: SyntaxNode<'a>, name: &str) -> Option<(&'a str, bool)> {
    above(statement).into_iter().filter(|a| a.tag == "param").find_map(|a| {
        let (declared, _, ty) = a.name_and_rest()?;
        let optional = declared.ends_with('?');
        (declared.trim_end_matches('?') == name && !ty.is_empty())
            .then(|| (type_part(ty), optional))
    })
}

/// The types of the `---@return` annotations above `statement`, in order.
pub fn return_types(statement: SyntaxNode<'_>) -> Vec<&str> {
    above(statement)
        .into_iter()
        .filter(|a| a.tag == "return" && !a.args.is_empty())
        .flat_map(|a| split_top(type_part(a.args), b','))
        .collect()
}

/// The type of a `---@type` annotation above `statement`, for its `index`-th
/// name: `---@type string, integer`.
pub fn declared_type(statement: SyntaxNode<'_>, index: usize) -> Option<&str> {
    let annotation = above(statement).into_iter().find(|a| a.tag == "type")?;
    split_top(type_part(annotation.args), b',').into_iter().nth(index)
}

/// The type at the start of `text`, without the name or the description
/// that may follow: `string` of `string the path`, `fun(a: x): y` whole.
pub fn type_part(text: &str) -> &str {
    let mut depth = 0i32;
    let bytes = text.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'(' | b'{' | b'<' | b'[' => depth += 1,
            b')' | b'}' | b'>' | b']' => depth -= 1,
            // `fun(): a, b` and `a | b`: the type goes on.
            b' ' | b'\t' if depth == 0 => {
                let before = text[..i].trim_end();
                let after = text[i..].trim_start();
                let joined = before.ends_with([':', '|', ',']) || after.starts_with(['|', ':']);
                if !joined {
                    return before;
                }
            }
            _ => {}
        }
    }
    text.trim_end()
}

/// `text` split at `separator` outside brackets, trimmed.
pub fn split_top(text: &str, separator: u8) -> Vec<&str> {
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

    #[test]
    fn type_parts() {
        assert_eq!(type_part("string the path"), "string");
        assert_eq!(type_part("string | nil the path"), "string | nil");
        assert_eq!(
            type_part("fun(a: string): integer, boolean ok"),
            "fun(a: string): integer, boolean"
        );
        assert_eq!(type_part("table<string, target> by name"), "table<string, target>");
        assert_eq!(type_part("{a: integer, b?: string}"), "{a: integer, b?: string}");
    }
}
