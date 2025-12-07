use crate::lexer::token::TokenKind;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) enum InfixOperator {
    Plus,
    Minus,
    Mul,
    Div,
}

impl InfixOperator {
    pub(crate) fn get(kind: TokenKind) -> Option<InfixOperator> {
        let op = match kind {
            TokenKind::Plus => InfixOperator::Plus,
            TokenKind::Minus => InfixOperator::Minus,
            TokenKind::Star => InfixOperator::Mul,
            TokenKind::Slash => InfixOperator::Div,
            _ => return None,
        };

        Some(op)
    }

    pub(crate) fn get_bp(&self) -> (u8, u8) {
        match &self {
            InfixOperator::Plus | InfixOperator::Minus => (1, 2),
            InfixOperator::Mul | InfixOperator::Div => (3, 4),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) enum PrefixOperator {
    Plus,
    Minus,
    Negate,
}

impl PrefixOperator {
    pub(crate) fn get(kind: TokenKind) -> Option<PrefixOperator> {
        let op = match kind {
            TokenKind::Plus => PrefixOperator::Plus,
            TokenKind::Minus => PrefixOperator::Minus,
            TokenKind::Bang => PrefixOperator::Negate,
            _ => return None,
        };

        Some(op)
    }

    pub(crate) fn get_bp(&self) -> u8 {
        5
    }
}

/// There is no PostfixOperator right now
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub(crate) enum PostfixOperator {
    Nothing,
}

impl PostfixOperator {
    pub(crate) fn get(kind: TokenKind) -> Option<PostfixOperator> {
        None
    }

    pub(crate) fn get_bp(&self) -> u8 {
        u8::MAX
    }
}
