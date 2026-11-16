//! Documentation comments: the comment lines right above a declaration.
//!
//! ```lua
//! --- The version of the protocol.
//! ---
//! --- # Notes
//! --- Bump it with the server.
//! local VERSION <const> = 3
//! ```
//!
//! `--`, `---` and `--[[ ]]` comments all count, Markdown included; a blank
//! line ends the block. Annotation lines (`---@param`, ...) are left out of
//! the text: they describe types.

use std::borrow::Cow;

use xmake_syntax::{SyntaxKind, SyntaxNode, SyntaxToken};

/// The documentation comment above `node`, as Markdown.
pub fn doc_comment<'a>(node: SyntaxNode<'a>) -> Option<Cow<'a, str>> {
    let first = node.first_token()?;
    let comments = comments_above(first);
    if comments.is_empty() {
        return None;
    }
    let mut lines: Vec<&'a str> = Vec::new();
    for comment in comments.iter().rev() {
        let text = comment.text();
        if comment.kind() == SyntaxKind::BLOCK_COMMENT {
            let level = text[3..].bytes().take_while(|&b| b == b'=').count();
            let body = text.get(4 + level..text.len().saturating_sub(2 + level)).unwrap_or("");
            lines.extend(body.lines());
            continue;
        }
        let line = text.trim_start_matches('-');
        let line = line.strip_prefix(' ').unwrap_or(line).trim_end();
        if !line.starts_with('@') {
            lines.push(line);
        }
    }
    // Drop blank lines around the text.
    while lines.first().is_some_and(|l| l.trim().is_empty()) {
        lines.remove(0);
    }
    while lines.last().is_some_and(|l| l.trim().is_empty()) {
        lines.pop();
    }
    match lines.as_slice() {
        [] => None,
        [single] => Some(Cow::Borrowed(single)),
        _ => Some(Cow::Owned(lines.join("\n"))),
    }
}

/// The comment tokens right above `token`, nearest first, stopping at a
/// blank line or code. A comment after code on its line does not count.
pub(crate) fn comments_above(token: SyntaxToken<'_>) -> Vec<SyntaxToken<'_>> {
    let mut out = Vec::new();
    let mut cur = token.prev_token();
    while let Some(prev) = cur {
        match prev.kind() {
            SyntaxKind::WHITESPACE => {
                if prev.text().bytes().filter(|&b| b == b'\n').count() > 1 {
                    break;
                }
            }
            SyntaxKind::LINE_COMMENT | SyntaxKind::BLOCK_COMMENT => {
                // `x = 1 -- comment` documents `x`, not the next line.
                let mut before = prev.prev_token();
                if let Some(blank) = before
                    .filter(|b| b.kind() == SyntaxKind::WHITESPACE && !b.text().contains('\n'))
                {
                    before = blank.prev_token();
                }
                if before.is_some_and(|b| !b.kind().is_trivia()) {
                    break;
                }
                out.push(prev);
            }
            _ => break,
        }
        cur = prev.prev_token();
    }
    out
}

#[cfg(test)]
mod tests {
    use xmake_syntax::{Dialect, parse};

    use super::*;

    fn doc_of_last_statement(text: &str) -> Option<String> {
        let parse = parse(text, Dialect::XMAKE_DEFAULT);
        let root = parse.root(text);
        let block = root.child_node(SyntaxKind::BLOCK)?;
        let last = block.children().last()?;
        doc_comment(last).map(Cow::into_owned)
    }

    #[test]
    fn line_comments() {
        let text = "x = 1 -- not this\n\n--- The answer.\n---\n--- # Notes\n--- Really.\n---@type integer\nlocal y = 42\n";
        assert_eq!(doc_of_last_statement(text).as_deref(), Some("The answer.\n\n# Notes\nReally."));
    }

    #[test]
    fn stops_at_blank_lines_and_code() {
        assert_eq!(doc_of_last_statement("-- far\n\nlocal y = 1\n"), None);
        assert_eq!(doc_of_last_statement("x = 1 -- trailing\nlocal y = 1\n"), None);
        assert_eq!(
            doc_of_last_statement("--[[ Block\n  doc ]]\ntarget('app')\n").as_deref(),
            Some(" Block\n  doc ")
        );
    }
}
