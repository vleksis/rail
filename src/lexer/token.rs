#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum TokenKind {
    Ident,
    Fn,     // fn
    Let,    // let
    Const,  // const
    Return, // return
    If,     // if
    Else,   // else
    While,  // while
    True,   // true
    False,  // false

    IntLit,
    FloatLit,

    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }
    Comma,     // ,
    Semicolon, // ;
    Colon,     // ,
    Dot,       // .

    Plus,         // +
    PlusEqual,    // +=
    Minus,        // -
    MinusEqual,   // -=
    Arrow,        // ->
    Star,         // *
    StarEqual,    // *=
    Slash,        // /
    SlashEqual,   // /=
    SlashSlash,   // //
    Percent,      // %
    PercentEqual, // %=
    Bang,         // !
    BangEq,       // !=
    Eq,           // =
    EqEq,         // ==
    Lt,           // <
    LtEq,         // <=
    Gt,           // >
    GtEq,         // >=
    AndAnd,       // &&
    OrOr,         // ||

    Eof,
}

impl TokenKind {
    fn keyword(source: &str) -> Option<TokenKind> {
        let res = match source {
            "fn" => TokenKind::Fn,
            "let" => TokenKind::Let,       // let
            "const" => TokenKind::Const,   // const
            "return" => TokenKind::Return, // return
            "if" => TokenKind::If,         // if
            "else" => TokenKind::Else,     // else
            "while" => TokenKind::While,   // while
            "true" => TokenKind::True,     // true
            "false" => TokenKind::False,   // false
            _ => return None,
        };

        Some(res)
    }

    pub fn ident_or_keyword(source: &str) -> TokenKind {
        Self::keyword(source).unwrap_or(TokenKind::Ident)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SourceLocation {
    line: usize,   // 1-based
    offset: usize, // 1-based
    byte: usize,   // 0-based
}

impl SourceLocation {
    pub fn new(line: usize, offset: usize, byte: usize) -> Self {
        Self { line, offset, byte }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SourceSpan {
    pub(crate) begin: SourceLocation,
    pub(crate) end: SourceLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'s> {
    pub(crate) kind: TokenKind,
    pub(crate) text: &'s str,
    pub(crate) line: usize,
    pub(crate) byte: usize,
}

impl<'s> Token<'s> {
    pub fn get_kind(&self) -> TokenKind {
        self.kind.clone()
    }
}
