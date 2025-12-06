#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    Identifier,

    Function, // fn
    Let,      // let
    Const,    // const
    Return,   // return
    If,       // if
    Else,     // else
    While,    // while
    True,     // true
    False,    // false

    Int64Lit(i64),
    Uint64Lit(u64),
    FloatLit(f64),

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
    LineComment,  // //
    Percent,      // %
    PercentEqual, // %=
    Bang,         // !
    BangEqual,    // !=
    Equal,        // =
    EqualEqual,   // ==
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=
    AndAnd,       // &&
    OrOr,         // ||

    EOF,
    Default,
}

impl TokenKind {
    pub fn ident_or_keyword(source: &str) -> TokenKind {
        match source {
            "fn" => TokenKind::Function,   // fn
            "let" => TokenKind::Let,       // let
            "const" => TokenKind::Const,   // const
            "return" => TokenKind::Return, // return
            "if" => TokenKind::If,         // if
            "else" => TokenKind::Else,     // else
            "while" => TokenKind::While,   // while
            "true" => TokenKind::True,     // true
            "false" => TokenKind::False,   // false
            _ => return TokenKind::Identifier,
        }
    }
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

    pub fn default() -> Self {
        Self {
            kind: TokenKind::Default,
            text: "",
            line: usize::MAX,
            byte: usize::MAX,
        }
    }
}
