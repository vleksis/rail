use crate::{
    grammar::{expression::ASTBuilder, *},
    lexer::Lexer,
    lexer::{Token, token},
};

pub struct Parser<'s> {
    lexer: Lexer<'s>,
    builder: ASTBuilder,
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

    fn consume(&mut self, kind: token::Kind) {
        dbg!(kind);
        if self.current.get_kind() == kind {
            self.advance();
        } else {
            panic!("not consumed: {kind:?}");
        }
    }

    pub fn parse(mut self) -> Syntax {
        self.parse_expression();

        Syntax {
            arena: self.builder.arena,
        }
    }

    fn parse_expression(&mut self) {
        self.advance();
        let exp = self.parse_bp(0);
        self.builder.arena.set_root(exp);
    }

    fn parse_lhs(&mut self) -> expression::Id {
        self.advance();
        match self.previous.get_kind() {
            token::Kind::Int64Lit(i) => self.builder.make_int64(i),
            token::Kind::Uint64Lit(u) => self.builder.make_uint64(u),
            token::Kind::FloatLit(f) => self.builder.make_float64(f),
            token::Kind::LParen => {
                let exp = self.parse_bp(0);
                self.consume(token::Kind::RParen);
                exp
            }
            t => {
                if let Some(op) = operator::Prefix::get(t) {
                    let rbp = op.get_bp();
                    let exp = self.parse_bp(rbp);
                    self.builder.make_prefix(op, exp)
                } else {
                    panic!("Unexpected TokenKind: {t:?}");
                }
            }
        }
    }

    fn parse_bp(&mut self, bp: u8) -> expression::Id {
        let mut lhs = self.parse_lhs();

        loop {
            let op = self.current.get_kind();

            if let Some(op) = operator::Postfix::get(op) {
                let lbp = op.get_bp();
                if lbp < bp {
                    break;
                }

                self.advance();
                lhs = self.builder.make_postfix(op, lhs);
                continue;
            }

            if let Some(op) = operator::Infix::get(op) {
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
                token::Kind::EOF | token::Kind::RParen => break,
                _ => {
                    panic!("Unexpected operation: {op:?}");
                }
            }
        }

        lhs
    }
}
