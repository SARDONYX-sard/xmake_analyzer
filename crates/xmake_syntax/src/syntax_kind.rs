//! The kinds of tokens and nodes of the syntax tree.

/// The kind of a token or a node.
///
/// Tokens come first, so that [`SyntaxKind::is_token`] is a single comparison.
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SyntaxKind {
    // === Trivia ===
    /// Spaces, tabs and line breaks.
    WHITESPACE,
    /// `-- comment` up to the end of the line.
    LINE_COMMENT,
    /// `--[[ comment ]]`, `--[==[ comment ]==]`.
    BLOCK_COMMENT,
    /// `#!/usr/bin/env lua` on the first line.
    SHEBANG,

    // === Literals and names ===
    IDENT,
    /// `42`, `0xff`, `1ULL` (LuaJIT).
    INT,
    /// `1.5`, `2e10`, `0x1p4`.
    FLOAT,
    /// `"text"` or `'text'`.
    STRING,
    /// `[[text]]`, `[==[text]==]`.
    LONG_STRING,

    // === Punctuation ===
    L_PAREN,
    R_PAREN,
    L_BRACE,
    R_BRACE,
    L_BRACK,
    R_BRACK,
    COMMA,
    SEMICOLON,
    /// `:`
    COLON,
    /// `::`
    COLON2,
    DOT,
    /// `..`
    DOT2,
    /// `...`
    DOT3,
    EQ,
    /// `==`
    EQ2,
    /// `~=`
    NEQ,
    LT,
    /// `<=`
    LTEQ,
    GT,
    /// `>=`
    GTEQ,
    /// `<<`
    SHL,
    /// `>>`
    SHR,
    PLUS,
    MINUS,
    STAR,
    SLASH,
    /// `//`
    SLASH2,
    PERCENT,
    CARET,
    /// `#`
    HASH,
    AMP,
    PIPE,
    TILDE,

    // === Keywords ===
    AND_KW,
    BREAK_KW,
    DO_KW,
    ELSE_KW,
    ELSEIF_KW,
    END_KW,
    FALSE_KW,
    FOR_KW,
    FUNCTION_KW,
    /// Lua 5.5 only; an identifier elsewhere.
    GLOBAL_KW,
    /// Lua 5.2+ and LuaJIT; an identifier in Lua 5.1.
    GOTO_KW,
    IF_KW,
    IN_KW,
    LOCAL_KW,
    NIL_KW,
    NOT_KW,
    OR_KW,
    REPEAT_KW,
    RETURN_KW,
    THEN_KW,
    TRUE_KW,
    UNTIL_KW,
    WHILE_KW,

    /// A character that cannot start any token.
    UNKNOWN,
    /// The end of the input: never part of the tree.
    EOF,

    // === Nodes ===
    SOURCE_FILE,
    BLOCK,

    LOCAL_STMT,
    LOCAL_FUNCTION_STMT,
    FUNCTION_STMT,
    GLOBAL_STMT,
    ASSIGN_STMT,
    CALL_STMT,
    DO_STMT,
    WHILE_STMT,
    REPEAT_STMT,
    IF_STMT,
    ELSEIF_CLAUSE,
    ELSE_CLAUSE,
    NUMERIC_FOR_STMT,
    GENERIC_FOR_STMT,
    RETURN_STMT,
    BREAK_STMT,
    GOTO_STMT,
    LABEL_STMT,

    /// A name being declared: local, parameter, loop variable.
    NAME,
    /// `<const>` / `<close>` after a local name.
    ATTRIB,
    NAME_LIST,
    EXPR_LIST,
    PARAM_LIST,
    /// `a.b.c:d` in `function a.b.c:d() end`.
    FUNC_NAME,

    /// `nil`, `true`, `false`, numbers, strings, `...`.
    LITERAL,
    /// A use of a name.
    NAME_REF,
    PAREN_EXPR,
    /// `a.b`
    FIELD_EXPR,
    /// `a[b]`
    INDEX_EXPR,
    /// `f(args)`, `f "str"`, `f {table}`
    CALL_EXPR,
    /// `obj:method(args)`
    METHOD_CALL_EXPR,
    ARG_LIST,
    FUNCTION_EXPR,
    TABLE_EXPR,
    /// `value`, `name = value` or `[key] = value`.
    TABLE_FIELD,
    BIN_EXPR,
    UNARY_EXPR,

    ERROR,
}

impl SyntaxKind {
    /// Whether this is a token rather than a node.
    pub fn is_token(self) -> bool {
        self <= SyntaxKind::EOF
    }

    /// Whether the parser skips this token.
    pub fn is_trivia(self) -> bool {
        matches!(
            self,
            SyntaxKind::WHITESPACE
                | SyntaxKind::LINE_COMMENT
                | SyntaxKind::BLOCK_COMMENT
                | SyntaxKind::SHEBANG
        )
    }

    /// Whether this is a comment token.
    pub fn is_comment(self) -> bool {
        matches!(self, SyntaxKind::LINE_COMMENT | SyntaxKind::BLOCK_COMMENT)
    }

    /// Whether this is a reserved word.
    pub fn is_keyword(self) -> bool {
        (SyntaxKind::AND_KW..=SyntaxKind::WHILE_KW).contains(&self)
    }

    /// Whether this is a string literal token.
    pub fn is_string(self) -> bool {
        matches!(self, SyntaxKind::STRING | SyntaxKind::LONG_STRING)
    }

    /// Describes a token kind in "expected ..." messages.
    pub fn describe(self) -> &'static str {
        use SyntaxKind::*;
        match self {
            IDENT => "a name",
            INT | FLOAT => "a number",
            STRING | LONG_STRING => "a string",
            L_PAREN => "`(`",
            R_PAREN => "`)`",
            L_BRACE => "`{`",
            R_BRACE => "`}`",
            L_BRACK => "`[`",
            R_BRACK => "`]`",
            COMMA => "`,`",
            COLON2 => "`::`",
            EQ => "`=`",
            GT => "`>`",
            DO_KW => "`do`",
            END_KW => "`end`",
            IN_KW => "`in`",
            THEN_KW => "`then`",
            UNTIL_KW => "`until`",
            FUNCTION_KW => "`function`",
            EOF => "end of file",
            _ => "a token",
        }
    }

    /// The keyword spelled `text`, honoring the version-dependent keywords.
    pub(crate) fn keyword(text: &str, features: crate::Features) -> Option<SyntaxKind> {
        use SyntaxKind::*;
        let kind = match text {
            "and" => AND_KW,
            "break" => BREAK_KW,
            "do" => DO_KW,
            "else" => ELSE_KW,
            "elseif" => ELSEIF_KW,
            "end" => END_KW,
            "false" => FALSE_KW,
            "for" => FOR_KW,
            "function" => FUNCTION_KW,
            "global" if features.has(crate::Feature::GlobalDeclarations) => GLOBAL_KW,
            "goto" if features.has(crate::Feature::Goto) => GOTO_KW,
            "if" => IF_KW,
            "in" => IN_KW,
            "local" => LOCAL_KW,
            "nil" => NIL_KW,
            "not" => NOT_KW,
            "or" => OR_KW,
            "repeat" => REPEAT_KW,
            "return" => RETURN_KW,
            "then" => THEN_KW,
            "true" => TRUE_KW,
            "until" => UNTIL_KW,
            "while" => WHILE_KW,
            _ => return None,
        };
        Some(kind)
    }
}
