//! Conversions between protocol types and IDE types.

use std::{
    borrow::Cow,
    path::{Path, PathBuf},
    str::FromStr,
};

use lsp_types::{Position, Range, Uri};
use xmake_ide::{AnalyzedFile, LineCol, TextRange, TextSize};

/// Converts a `file://` URI into a path.
pub(crate) fn uri_to_path(uri: &Uri) -> Option<PathBuf> {
    let rest = uri.as_str().strip_prefix("file://")?;
    // Skip the authority (usually empty).
    let path = &rest[rest.find('/')?..];
    let decoded = percent_decode(path)?;
    // `/c:/foo` on Windows.
    let bytes = decoded.as_bytes();
    let is_drive =
        bytes.len() >= 3 && bytes[0] == b'/' && bytes[1].is_ascii_alphabetic() && bytes[2] == b':';
    if !is_drive {
        return Some(PathBuf::from(decoded.into_owned()));
    }
    // Normalize the drive letter: editors send both `c:` and `C:`.
    let mut path = decoded[1..].replace('/', "\\");
    path[..1].make_ascii_uppercase();
    Some(PathBuf::from(path))
}

/// Converts an absolute path into a `file://` URI.
pub(crate) fn path_to_uri(path: &Path) -> Uri {
    let s = path.to_string_lossy();
    let mut out = String::with_capacity(s.len() + 8);
    out.push_str("file://");
    if !s.starts_with(['/', '\\']) {
        out.push('/');
    }
    for b in s.bytes() {
        match b {
            b'\\' => out.push('/'),
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' | b'/' => {
                out.push(b as char);
            }
            _ => {
                use std::fmt::Write as _;
                let _ = write!(out, "%{b:02X}");
            }
        }
    }
    Uri::from_str(&out).unwrap_or_else(|_| Uri::from_str("file:///").expect("valid uri"))
}

/// Decodes `%XX` escapes, borrowing when there is none.
fn percent_decode(s: &str) -> Option<Cow<'_, str>> {
    if !s.contains('%') {
        return Some(Cow::Borrowed(s));
    }
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            out.push(u8::from_str_radix(s.get(i + 1..i + 3)?, 16).ok()?);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8(out).ok().map(Cow::Owned)
}

/// A protocol position in `file` as a byte offset.
pub(crate) fn offset(file: &AnalyzedFile, pos: Position) -> TextSize {
    file.line_index.offset(&file.text, LineCol { line: pos.line, col: pos.character })
}

/// A byte offset of `file` as a protocol position.
pub(crate) fn position(file: &AnalyzedFile, offset: TextSize) -> Position {
    let lc = file.line_index.line_col(&file.text, offset);
    Position { line: lc.line, character: lc.col }
}

/// A byte range of `file` as a protocol range.
pub(crate) fn range(file: &AnalyzedFile, range: TextRange) -> Range {
    Range { start: position(file, range.start()), end: position(file, range.end()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uri_roundtrip() {
        let path = if cfg!(windows) {
            PathBuf::from(r"C:\a b\xmake.lua")
        } else {
            PathBuf::from("/a b/xmake.lua")
        };
        let uri = path_to_uri(&path);
        assert!(uri.as_str().contains("a%20b"), "{}", uri.as_str());
        assert_eq!(uri_to_path(&uri), Some(path));
    }

    #[test]
    fn drive_letter_is_normalized() {
        let uri = Uri::from_str("file:///c%3A/x/xmake.lua").expect("uri");
        let path = uri_to_path(&uri).expect("path");
        if cfg!(windows) {
            assert_eq!(path, PathBuf::from(r"C:\x\xmake.lua"));
        }
    }
}
