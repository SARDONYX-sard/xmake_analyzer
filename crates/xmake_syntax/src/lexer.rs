//! Zero-copy Lua lexer.
//!
//! The lexer never allocates: a [`Token`] is a kind plus a byte range of the
//! source, and the tokens cover the input losslessly. Malformed input still
//! yields tokens, with a [`LexError`] attached.

use crate::{Features, SyntaxKind};

/// A lexical error attached to a token.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LexError {
    /// A quoted string reached the end of the line without its closing quote.
    UnterminatedString,
    /// A `[[` string reached the end of the file without its closing `]]`.
    UnterminatedLongString,
    /// A `--[[` comment reached the end of the file without its closing `]]`.
    UnterminatedLongComment,
    /// A number such as `1e` or `0x`.
    MalformedNumber,
    /// A character that cannot start any token.
    UnknownChar,
}

impl LexError {
    /// Human readable message.
    pub fn message(self) -> &'static str {
        match self {
            LexError::UnterminatedString => "unterminated string",
            LexError::UnterminatedLongString => "unterminated long string, expected `]]`",
            LexError::UnterminatedLongComment => "unterminated long comment, expected `]]`",
            LexError::MalformedNumber => "malformed number",
            LexError::UnknownChar => "unexpected character",
        }
    }
}

/// A token borrowing nothing: its text is `source[offset..offset + len]`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: SyntaxKind,
    pub offset: u32,
    pub len: u32,
    pub error: Option<LexError>,
}

/// Iterator over the tokens of a source text.
pub struct Lexer<'a> {
    text: &'a str,
    pos: usize,
    features: Features,
}

impl<'a> Lexer<'a> {
    /// Lexes `text` with the keywords of `features` (`goto`, `global`).
    pub fn new(text: &'a str, features: Features) -> Lexer<'a> {
        Lexer { text, pos: 0, features }
    }

    fn bytes(&self) -> &'a [u8] {
        self.text.as_bytes()
    }

    fn peek(&self, ahead: usize) -> u8 {
        self.bytes().get(self.pos + ahead).copied().unwrap_or(0)
    }

    fn bump_while(&mut self, pred: impl Fn(u8) -> bool) {
        while self.pos < self.text.len() && pred(self.bytes()[self.pos]) {
            self.pos += 1;
        }
    }

    /// Lexes one token starting at `self.pos`, returning its kind and error.
    fn lex(&mut self) -> (SyntaxKind, Option<LexError>) {
        use SyntaxKind::*;
        let c = self.peek(0);

        if self.pos == 0 && c == b'#' && self.peek(1) == b'!' {
            self.bump_while(|b| b != b'\n');
            return (SHEBANG, None);
        }
        if c.is_ascii_whitespace() {
            self.bump_while(|b| b.is_ascii_whitespace());
            return (WHITESPACE, None);
        }
        if c == b'-' && self.peek(1) == b'-' {
            return self.comment();
        }
        if c == b'_' || c.is_ascii_alphabetic() {
            let start = self.pos;
            self.bump_while(|b| b == b'_' || b.is_ascii_alphanumeric());
            let word = &self.text[start..self.pos];
            return (SyntaxKind::keyword(word, self.features).unwrap_or(IDENT), None);
        }
        if c.is_ascii_digit() || (c == b'.' && self.peek(1).is_ascii_digit()) {
            return self.number();
        }
        if c == b'"' || c == b'\'' {
            return self.quoted_string(c);
        }
        if c == b'['
            && let Some(level) = self.long_bracket_level()
        {
            let closed = self.long_bracket_body(level);
            return (LONG_STRING, (!closed).then_some(LexError::UnterminatedLongString));
        }

        let (kind, len) = self.punct(c);
        if kind == UNKNOWN {
            // Skip a whole UTF-8 character.
            let ch_len = self.text[self.pos..].chars().next().map_or(1, char::len_utf8);
            self.pos += ch_len;
            return (UNKNOWN, Some(LexError::UnknownChar));
        }
        self.pos += len;
        (kind, None)
    }

    fn punct(&self, c: u8) -> (SyntaxKind, usize) {
        use SyntaxKind::*;
        let next = self.peek(1);
        match (c, next) {
            (b'.', b'.') if self.peek(2) == b'.' => (DOT3, 3),
            (b'.', b'.') => (DOT2, 2),
            (b':', b':') => (COLON2, 2),
            (b'=', b'=') => (EQ2, 2),
            (b'~', b'=') => (NEQ, 2),
            (b'<', b'=') => (LTEQ, 2),
            (b'>', b'=') => (GTEQ, 2),
            (b'<', b'<') => (SHL, 2),
            (b'>', b'>') => (SHR, 2),
            (b'/', b'/') => (SLASH2, 2),
            (b'(', _) => (L_PAREN, 1),
            (b')', _) => (R_PAREN, 1),
            (b'{', _) => (L_BRACE, 1),
            (b'}', _) => (R_BRACE, 1),
            (b'[', _) => (L_BRACK, 1),
            (b']', _) => (R_BRACK, 1),
            (b',', _) => (COMMA, 1),
            (b';', _) => (SEMICOLON, 1),
            (b':', _) => (COLON, 1),
            (b'.', _) => (DOT, 1),
            (b'=', _) => (EQ, 1),
            (b'<', _) => (LT, 1),
            (b'>', _) => (GT, 1),
            (b'+', _) => (PLUS, 1),
            (b'-', _) => (MINUS, 1),
            (b'*', _) => (STAR, 1),
            (b'/', _) => (SLASH, 1),
            (b'%', _) => (PERCENT, 1),
            (b'^', _) => (CARET, 1),
            (b'#', _) => (HASH, 1),
            (b'&', _) => (AMP, 1),
            (b'|', _) => (PIPE, 1),
            (b'~', _) => (TILDE, 1),
            _ => (UNKNOWN, 1),
        }
    }

    fn comment(&mut self) -> (SyntaxKind, Option<LexError>) {
        self.pos += 2;
        if self.peek(0) == b'['
            && let Some(level) = self.long_bracket_level()
        {
            let closed = self.long_bracket_body(level);
            let error = (!closed).then_some(LexError::UnterminatedLongComment);
            return (SyntaxKind::BLOCK_COMMENT, error);
        }
        self.bump_while(|b| b != b'\n' && b != b'\r');
        (SyntaxKind::LINE_COMMENT, None)
    }

    /// At `[`, returns the level of a long bracket `[==[` without consuming
    /// anything, or `None` for a plain `[`.
    fn long_bracket_level(&self) -> Option<usize> {
        let rest = &self.bytes()[self.pos + 1..];
        let level = rest.iter().take_while(|&&b| b == b'=').count();
        (rest.get(level) == Some(&b'[')).then_some(level)
    }

    /// Consumes `[==[ ... ]==]`. Returns whether the closing bracket was found.
    fn long_bracket_body(&mut self, level: usize) -> bool {
        self.pos += level + 2;
        let bytes = self.bytes();
        while self.pos < bytes.len() {
            if bytes[self.pos] == b']' {
                let after = &bytes[self.pos + 1..];
                let equals = after.iter().take_while(|&&b| b == b'=').count();
                if equals == level && after.get(level) == Some(&b']') {
                    self.pos += level + 2;
                    return true;
                }
            }
            self.pos += 1;
        }
        false
    }

    fn quoted_string(&mut self, quote: u8) -> (SyntaxKind, Option<LexError>) {
        self.pos += 1;
        let bytes = self.bytes();
        while self.pos < bytes.len() {
            match bytes[self.pos] {
                b if b == quote => {
                    self.pos += 1;
                    return (SyntaxKind::STRING, None);
                }
                b'\\' => {
                    // An escaped line break continues the string; `\z` skips
                    // the following blanks, line breaks included.
                    self.pos += 1;
                    match self.peek(0) {
                        b'\r' if self.peek(1) == b'\n' => self.pos += 2,
                        b'z' => {
                            self.pos += 1;
                            self.bump_while(|b| b.is_ascii_whitespace());
                        }
                        0 => {}
                        _ => self.pos += 1,
                    }
                }
                b'\n' | b'\r' => break,
                _ => self.pos += 1,
            }
        }
        (SyntaxKind::STRING, Some(LexError::UnterminatedString))
    }

    fn number(&mut self) -> (SyntaxKind, Option<LexError>) {
        let start = self.pos;
        let is_hex = self.peek(0) == b'0' && matches!(self.peek(1), b'x' | b'X');
        if is_hex {
            self.pos += 2;
        }
        let exponent = if is_hex { *b"pP" } else { *b"eE" };
        let mut is_float = false;
        let mut malformed = false;
        loop {
            let c = self.peek(0);
            if exponent.contains(&c) {
                is_float = true;
                self.pos += 1;
                if matches!(self.peek(0), b'+' | b'-') {
                    self.pos += 1;
                }
                malformed |= !self.peek(0).is_ascii_digit();
            } else if c == b'.' && self.peek(1) != b'.' {
                is_float = true;
                self.pos += 1;
            } else if c.is_ascii_alphanumeric() || c == b'_' {
                self.pos += 1;
            } else {
                break;
            }
        }
        let text = &self.text[start..self.pos];
        // LuaJIT suffixes: `1LL`, `1ULL`, `1i`.
        let digits = text.trim_end_matches(['L', 'l', 'U', 'u', 'i', 'I']);
        let body = if is_hex { &digits[2..] } else { digits };
        let valid_digit = |b: u8| {
            let digit = if is_hex { b.is_ascii_hexdigit() } else { b.is_ascii_digit() };
            digit || matches!(b, b'.' | b'+' | b'-') || exponent.contains(&b)
        };
        malformed |= body.is_empty() || !body.bytes().all(valid_digit);
        let kind = if is_float { SyntaxKind::FLOAT } else { SyntaxKind::INT };
        (kind, malformed.then_some(LexError::MalformedNumber))
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.pos >= self.text.len() {
            return None;
        }
        let start = self.pos;
        let (kind, error) = self.lex();
        debug_assert!(self.pos > start, "the lexer made no progress");
        Some(Token { kind, offset: start as u32, len: (self.pos - start) as u32, error })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LuaVersion;

    fn kinds(text: &str, lua: LuaVersion) -> Vec<(SyntaxKind, &str)> {
        Lexer::new(text, lua.features())
            .filter(|t| t.kind != SyntaxKind::WHITESPACE)
            .map(|t| (t.kind, &text[t.offset as usize..(t.offset + t.len) as usize]))
            .collect()
    }

    #[test]
    fn lossless() {
        let text = "local x = [==[a]]b]==] --[[c]] -- d\nprint(x .. 'e\\'f', 0x1p4, ...)";
        let total: u32 = Lexer::new(text, LuaVersion::Lua54.features()).map(|t| t.len).sum();
        assert_eq!(total as usize, text.len());
    }

    #[test]
    fn long_brackets() {
        use SyntaxKind::*;
        let toks = kinds("[==[a]]b]==] --[[c\n]] x[1]", LuaVersion::Lua54);
        assert_eq!(
            toks,
            [
                (LONG_STRING, "[==[a]]b]==]"),
                (BLOCK_COMMENT, "--[[c\n]]"),
                (IDENT, "x"),
                (L_BRACK, "["),
                (INT, "1"),
                (R_BRACK, "]"),
            ]
        );
    }

    #[test]
    fn numbers() {
        use SyntaxKind::*;
        let toks = kinds("3 3.0 .5 1e-3 0xff 0x1p4 1ULL 1e", LuaVersion::Lua54);
        let ks: Vec<_> = toks.iter().map(|t| t.0).collect();
        assert_eq!(ks, [INT, FLOAT, FLOAT, FLOAT, INT, FLOAT, INT, FLOAT]);
        let errors: Vec<_> =
            Lexer::new("1e 0x", LuaVersion::Lua54.features()).filter_map(|t| t.error).collect();
        assert_eq!(errors, [LexError::MalformedNumber, LexError::MalformedNumber]);
    }

    #[test]
    fn version_dependent_keywords() {
        assert_eq!(kinds("goto", LuaVersion::Lua51), [(SyntaxKind::IDENT, "goto")]);
        assert_eq!(kinds("goto", LuaVersion::LuaJit), [(SyntaxKind::GOTO_KW, "goto")]);
        assert_eq!(kinds("global", LuaVersion::Lua54), [(SyntaxKind::IDENT, "global")]);
        assert_eq!(kinds("global", LuaVersion::Lua55), [(SyntaxKind::GLOBAL_KW, "global")]);
    }

    #[test]
    fn unterminated_string_stops_at_line_end() {
        let toks: Vec<_> = Lexer::new("'abc\nx", LuaVersion::Lua54.features()).collect();
        assert_eq!(toks[0].error, Some(LexError::UnterminatedString));
        assert_eq!(toks[0].len, 4);
    }
}
