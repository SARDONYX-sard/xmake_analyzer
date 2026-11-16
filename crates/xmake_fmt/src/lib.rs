//! Formatter for xmake scripts.
//!
//! The formatter re-emits the tokens of the file, so comments are never lost:
//!
//! - line breaks are kept where the author put them, runs of blank lines are
//!   shortened to [`FormatOptions::max_blank_lines`];
//! - every line is re-indented from the syntax tree; in description files
//!   the contents of `target(...)`, `option(...)`, ... are indented one level,
//!   as in the xmake manual;
//! - spaces between tokens on a line are normalized (`a = b + 1`, `f(a, b)`,
//!   `function (target)`, `{x = 1}`);
//! - trailing whitespace is removed and the file ends with one line break.
//!
//! Multi-line tokens (long strings and comments) are copied verbatim. As a
//! safety net, the result is lexed again and must have the same tokens as the
//! input; otherwise [`format`] gives up.

mod editorconfig;
mod indent;
mod spacing;

#[cfg(test)]
mod tests;

use std::path::Path;

use xmake_syntax::{Dialect, SyntaxKind, SyntaxToken, lexer::Lexer, parse};

pub use crate::editorconfig::EditorConfig;

/// One indentation level.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Indent {
    /// This many spaces.
    Spaces(u8),
    /// This many tabs.
    Tabs(u8),
}

/// The line break written.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Newline {
    /// The first line break of the input (LF when there is none).
    #[default]
    Auto,
    Lf,
    CrLf,
}

/// How to format.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormatOptions {
    pub indent: Indent,
    pub newline: Newline,
    /// End a non-empty file with a line break.
    pub final_newline: bool,
    /// Indent the contents of scopes (`target("x")` ... `target_end()`).
    /// Only meaningful for description files.
    pub indent_scopes: bool,
    /// Longest run of blank lines kept.
    pub max_blank_lines: usize,
}

impl Default for FormatOptions {
    fn default() -> FormatOptions {
        FormatOptions {
            indent: Indent::Spaces(4),
            newline: Newline::Auto,
            final_newline: true,
            indent_scopes: true,
            max_blank_lines: 1,
        }
    }
}

impl FormatOptions {
    /// The options of the file at `path`: the defaults, then `base` (e.g.
    /// the editor settings), then `.editorconfig`.
    pub fn for_file(path: &Path, base: impl FnOnce(&mut FormatOptions)) -> FormatOptions {
        let mut options = FormatOptions::default();
        base(&mut options);
        EditorConfig::for_file(path).apply(&mut options);
        options
    }
}

/// Why a file was not formatted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FormatError {
    /// The file has syntax errors: its tree cannot be trusted.
    SyntaxErrors,
    /// The output would not have the same tokens as the input (a bug).
    Unsafe,
}

impl std::fmt::Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            FormatError::SyntaxErrors => "the file has syntax errors",
            FormatError::Unsafe => "formatting would change the meaning of the file",
        })
    }
}

impl std::error::Error for FormatError {}

/// Formats `text`.
pub fn format(text: &str, dialect: Dialect, options: FormatOptions) -> Result<String, FormatError> {
    let parse = parse(text, dialect);
    if !parse.errors().is_empty() {
        return Err(FormatError::SyntaxErrors);
    }
    let root = parse.root(text);
    let mut levels = indent::Levels::new(root, text, options.indent_scopes);
    let newline = match options.newline {
        Newline::Lf => "\n",
        Newline::CrLf => "\r\n",
        Newline::Auto => match text.find('\n') {
            Some(i) if text[..i].ends_with('\r') => "\r\n",
            _ => "\n",
        },
    };

    let mut out = String::with_capacity(text.len() + text.len() / 8);
    let mut prev: Option<SyntaxToken<'_>> = None;
    let mut pending_ws: &str = "";
    let tokens: Vec<SyntaxToken<'_>> = root.descendant_tokens().collect();
    for (i, &token) in tokens.iter().enumerate() {
        if token.kind() == SyntaxKind::WHITESPACE {
            pending_ws = token.text();
            continue;
        }
        let breaks = pending_ws.bytes().filter(|&b| b == b'\n').count();
        match prev {
            None => {
                let level = levels.level_of(token, &tokens[i + 1..]);
                push_indent(&mut out, options.indent, level);
            }
            Some(_) if breaks > 0 => {
                trim_trailing_blanks(&mut out);
                let kept = breaks.min(options.max_blank_lines + 1);
                for _ in 0..kept {
                    out.push_str(newline);
                }
                let level = levels.level_of(token, &tokens[i + 1..]);
                push_indent(&mut out, options.indent, level);
            }
            Some(prev) => {
                if spacing::space_between(prev, token, !pending_ws.is_empty()) {
                    out.push(' ');
                }
            }
        }
        out.push_str(token.text());
        prev = Some(token);
        pending_ws = "";
    }
    trim_trailing_blanks(&mut out);
    if options.final_newline && !out.is_empty() {
        out.push_str(newline);
    }

    if !same_tokens(text, &out, dialect) {
        return Err(FormatError::Unsafe);
    }
    Ok(out)
}

fn push_indent(out: &mut String, indent: Indent, level: u32) {
    for _ in 0..level {
        match indent {
            Indent::Tabs(n) => out.extend(std::iter::repeat_n('\t', n as usize)),
            Indent::Spaces(n) => out.extend(std::iter::repeat_n(' ', n as usize)),
        }
    }
}

/// Removes spaces and tabs at the end of `out`.
fn trim_trailing_blanks(out: &mut String) {
    let trimmed = out.trim_end_matches([' ', '\t']).len();
    out.truncate(trimmed);
}

/// Whether `a` and `b` have the same tokens, ignoring whitespace and the
/// trailing blanks of line comments.
fn same_tokens(a: &str, b: &str, dialect: Dialect) -> bool {
    significant_tokens(a, dialect).eq(significant_tokens(b, dialect))
}

fn significant_tokens(text: &str, dialect: Dialect) -> impl Iterator<Item = (SyntaxKind, &str)> {
    Lexer::new(text, dialect.lua.features()).filter(|t| t.kind != SyntaxKind::WHITESPACE).map(
        move |t| {
            let s = &text[t.offset as usize..(t.offset + t.len) as usize];
            (t.kind, s.trim_end())
        },
    )
}
