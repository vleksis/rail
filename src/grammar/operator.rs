use crate::lexer::token::Kind;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) enum Infix {
    Plus,
    Minus,
    Mul,
    Div,
}

impl Infix {
    pub(crate) fn get(kind: Kind) -> Option<Infix> {
        let op = match kind {
            Kind::Plus => Infix::Plus,
            Kind::Minus => Infix::Minus,
            Kind::Star => Infix::Mul,
            Kind::Slash => Infix::Div,
            _ => return None,
        };

        Some(op)
    }

    pub(crate) fn get_bp(&self) -> (u8, u8) {
        match &self {
            Infix::Plus | Infix::Minus => (1, 2),
            Infix::Mul | Infix::Div => (3, 4),
        }
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
        5
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
