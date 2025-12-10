#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
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

impl Kind {
    pub fn ident_or_keyword(source: &str) -> Kind {
        match source {
            "fn" => Kind::Function,   // fn
            "let" => Kind::Let,       // let
            "const" => Kind::Const,   // const
            "return" => Kind::Return, // return
            "if" => Kind::If,         // if
            "else" => Kind::Else,     // else
            "while" => Kind::While,   // while
            "true" => Kind::True,     // true
            "false" => Kind::False,   // false
            _ => return Kind::Identifier,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'s> {
    pub(crate) kind: Kind,
    pub(crate) text: &'s str,
    pub(crate) line: usize,
    pub(crate) byte: usize,
}

impl<'s> Token<'s> {
    pub fn get_kind(&self) -> Kind {
        self.kind.clone()
    }

    pub fn default() -> Self {
        Self {
            kind: Kind::Default,
            text: "",
            line: usize::MAX,
            byte: usize::MAX,
        }
    }
}
