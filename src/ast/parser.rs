use thiserror::Error;

use crate::{
    ast::{
        node::{ASTBuilder, ExpressionId, ExpressionNode},
        op::{InfixOperator, PostfixOperator, PrefixOperator},
    },
    lexer::{
        lexer::Lexer,
        token::{Token, TokenKind},
    },
};

#[derive(Debug, Error)]
#[error("Parsing error")]
pub struct ParseError {}

pub struct Parser<'s> {
    lexer: Lexer<'s>,
    pub builder: ASTBuilder,
    previous: Token<'s>,
    current: Token<'s>,
}

impl<'s> Parser<'s> {
    pub fn new(lexer: Lexer<'s>) -> Self {
        Self {
            lexer,
            builder: ASTBuilder::default(),
            previous: Token::default(),
            current: Token::default(),
        }
    }

    fn advance(&mut self) {
        let token = self.lexer.scan_token().unwrap();
        self.previous = std::mem::replace(&mut self.current, token);
    }

    fn consume(&mut self, kind: TokenKind) {
        dbg!(kind);
        if self.current.get_kind() == kind {
            self.advance();
        } else {
            panic!("not consumed: {kind:?}");
        }
    }

    pub fn parse_expression(&mut self) -> ExpressionId {
        self.advance();
        self.parse_bp(0)
    }

    fn parse_lhs(&mut self) -> ExpressionId {
        self.advance();
        match self.previous.get_kind() {
            TokenKind::Int64Lit(i) => self.builder.make_int64(i),
            TokenKind::Uint64Lit(u) => self.builder.make_uint64(u),
            TokenKind::FloatLit(f) => self.builder.make_float64(f),
            TokenKind::LParen => {
                let exp = self.parse_bp(0);
                self.consume(TokenKind::RParen);
                exp
            }
            t => {
                if let Some(op) = PrefixOperator::get(t) {
                    let rbp = op.get_bp();
                    let exp = self.parse_bp(rbp);
                    self.builder.make_prefix(op, exp)
                } else {
                    panic!("Unexpected TokenKind: {t:?}");
                }
            }
        }
    }

    fn parse_bp(&mut self, bp: u8) -> ExpressionId {
        let mut lhs = self.parse_lhs();

        loop {
            let op = self.current.get_kind();

            if let Some(op) = PostfixOperator::get(op) {
                let lbp = op.get_bp();
                if lbp < bp {
                    break;
                }

                self.advance();
                lhs = self.builder.make_postfix(op, lhs);
                continue;
            }

            if let Some(op) = InfixOperator::get(op) {
                let (lbp, rbp) = op.get_bp();
                if lbp < bp {
                    break;
                }

                self.advance();
                let rhs = self.parse_bp(rbp);
                lhs = self.builder.make_infix(op, lhs, rhs);
                continue;
            }

            match op {
                TokenKind::EOF | TokenKind::RParen => break,
                _ => {
                    panic!("Unexpected operation: {op:?}");
                }
            }
        }

        lhs
    }
}
