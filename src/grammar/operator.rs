use std::fmt::Display;

use crate::lexer::token::Kind;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) enum Infix {
    Plus,
    Minus,
    Mul,
    Div,

    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

impl Infix {
    pub(crate) fn get(kind: Kind) -> Option<Infix> {
        let op = match kind {
            Kind::Plus => Infix::Plus,
            Kind::Minus => Infix::Minus,
            Kind::Star => Infix::Mul,
            Kind::Slash => Infix::Div,

            Kind::EqualEqual => Infix::Equal,
            Kind::BangEqual => Infix::NotEqual,
            Kind::Less => Infix::Less,
            Kind::LessEqual => Infix::LessEqual,
            Kind::Greater => Infix::Greater,
            Kind::GreaterEqual => Infix::GreaterEqual,

            _ => return None,
        };

        Some(op)
    }

    pub(crate) fn get_bp(&self) -> (u8, u8) {
        use Infix::*;

        match &self {
            Equal | NotEqual | Less | LessEqual | Greater | GreaterEqual => (1, 2),
            Plus | Minus => (3, 4),
            Mul | Div => (5, 6),
        }
    }
}

impl Display for Infix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Infix::*;

        let label = match self {
            Plus => "+ Addition",
            Minus => "- Subtraction",
            Mul => "* Multiplication",
            Div => "/ Division",
            Equal => "== Equal Comparison",
            NotEqual => "!= NotEqual Comparison",
            Less => "< Less Comparison",
            LessEqual => "<= LessEqual Comparison",
            Greater => "> Greater Comparison",
            GreaterEqual => ">= GreaterEqual Comparison",
        };

        f.write_str(label)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) enum Prefix {
    Plus,
    Minus,
    Negate,
}

impl Prefix {
    pub(crate) fn get(kind: Kind) -> Option<Prefix> {
        let op = match kind {
            Kind::Plus => Prefix::Plus,
            Kind::Minus => Prefix::Minus,
            Kind::Bang => Prefix::Negate,
            _ => return None,
        };

        Some(op)
    }

    pub(crate) fn get_bp(&self) -> u8 {
        7
    }
}

impl Display for Prefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Prefix::*;

        let label = match self {
            Plus => "+ Unary Plus",
            Minus => "- Unary Minus",
            Negate => "! Negation",
        };

        f.write_str(label)
    }
}

/// There is no Postfix right now
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) enum Postfix {
    _Nothing,
}

impl Postfix {
    pub(crate) fn get(_kind: Kind) -> Option<Postfix> {
        // unimplemented
        None
    }

    pub(crate) fn get_bp(&self) -> u8 {
        u8::MAX
    }
}
