use thiserror::Error;

use crate::{
    ast::node::{
        AdditionNode, BooleanNegationNode, DivisionNode, ExpressionNode, IntegralNegationNode,
        MultiplicationNode, SubtractionNode,
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
    previous: Token<'s>,
    current: Token<'s>,
}

impl<'s> Parser<'s> {
    pub fn new(lexer: Lexer<'s>) -> Self {
        Self {
            lexer,
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

    pub fn parse_expression(&mut self) -> Box<ExpressionNode> {
        self.advance();
        self.parse_bp(0)
    }

    fn parse_lhs(&mut self) -> Box<ExpressionNode> {
        self.advance();
        match self.previous.get_kind() {
            TokenKind::IntLit(i) => Box::new(ExpressionNode::Int64(i)),
            TokenKind::LParen => {
                let exp = self.parse_bp(0);
                self.consume(TokenKind::RParen);
                exp
            }
            t => {
                if let Some(op) = PrefixOperator::get(t) {
                    let rbp = op.get_bp();
                    let exp = self.parse_bp(rbp);
                    op.make_expression(exp)
                } else {
                    panic!("Unexpected TokenKind: {t:?}");
                }
            }
        }
    }

    fn parse_bp(&mut self, bp: u8) -> Box<ExpressionNode> {
        let mut lhs = self.parse_lhs();

        loop {
            let op = self.current.get_kind();

            if let Some(op) = PostfixOperator::get(op) {
                let lbp = op.get_bp();
                if lbp < bp {
                    break;
                }

                self.advance();
                lhs = op.make_expression(lhs);
                continue;
            }

            if let Some(op) = InfixOperator::get(op) {
                let (lbp, rbp) = op.get_bp();
                if lbp < bp {
                    break;
                }

                self.advance();
                let rhs = self.parse_bp(rbp);
                lhs = op.make_expression(lhs, rhs);
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

enum InfixOperator {
    Plus,
    Minus,
    Mul,
    Div,
}

impl InfixOperator {
    fn get(kind: TokenKind) -> Option<InfixOperator> {
        let op = match kind {
            TokenKind::Plus => InfixOperator::Plus,
            TokenKind::Minus => InfixOperator::Minus,
            TokenKind::Star => InfixOperator::Mul,
            TokenKind::Slash => InfixOperator::Div,
            _ => return None,
        };

        Some(op)
    }

    fn make_expression(
        &self,
        lhs: Box<ExpressionNode>,
        rhs: Box<ExpressionNode>,
    ) -> Box<ExpressionNode> {
        let exp = match self {
            InfixOperator::Plus => ExpressionNode::Addition(AdditionNode { lhs, rhs }),
            InfixOperator::Minus => ExpressionNode::Subtraction(SubtractionNode { lhs, rhs }),
            InfixOperator::Mul => ExpressionNode::Multiplication(MultiplicationNode { lhs, rhs }),
            InfixOperator::Div => ExpressionNode::Division(DivisionNode { lhs, rhs }),
        };

        Box::new(exp)
    }

    fn get_bp(&self) -> (u8, u8) {
        match &self {
            InfixOperator::Plus | InfixOperator::Minus => (1, 2),
            InfixOperator::Mul | InfixOperator::Div => (3, 4),
        }
    }
}

enum PrefixOperator {
    Plus,
    Minus,
    Negate,
}

impl PrefixOperator {
    fn get(kind: TokenKind) -> Option<PrefixOperator> {
        let op = match kind {
            TokenKind::Plus => PrefixOperator::Plus,
            TokenKind::Minus => PrefixOperator::Minus,
            TokenKind::Bang => PrefixOperator::Negate,
            _ => return None,
        };

        Some(op)
    }

    fn make_expression(&self, exp: Box<ExpressionNode>) -> Box<ExpressionNode> {
        let exp = match self {
            PrefixOperator::Plus => return exp,
            PrefixOperator::Minus => ExpressionNode::IntegralNegation(IntegralNegationNode { exp }),
            PrefixOperator::Negate => ExpressionNode::BooleanNegation(BooleanNegationNode { exp }),
        };

        Box::new(exp)
    }

    fn get_bp(&self) -> u8 {
        5
    }
}

/// There is no PostfixOperator right now
enum PostfixOperator {}

impl PostfixOperator {
    fn get(kind: TokenKind) -> Option<PostfixOperator> {
        None
    }

    fn make_expression(&self, exp: Box<ExpressionNode>) -> Box<ExpressionNode> {
        unimplemented!()
    }

    fn get_bp(&self) -> u8 {
        u8::MAX
    }
}
