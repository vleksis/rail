use crate::{
    grammar::*,
    lexer::Lexer,
    lexer::{Token, token},
};

pub struct Parser<'s> {
    lexer: Lexer<'s>,
    arena: Arena,
    previous: Token<'s>,
    current: Token<'s>,
}

impl<'s> Parser<'s> {
    pub fn new(lexer: Lexer<'s>) -> Self {
        Self {
            lexer,
            arena: Arena::default(),
            previous: Token::default(),
            current: Token::default(),
        }
    }

    fn advance(&mut self) {
        let token = self.lexer.scan_token().unwrap();
        dbg!(&token);
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
        self.advance();
        let stmt = self.parse_statement();
        self.arena.set_root(stmt);
        Syntax { arena: self.arena }
    }

    fn parse_statement(&mut self) -> statement::Id {
        match self.previous.get_kind() {
            token::Kind::LBrace => self.parse_block(),
            token::Kind::Let => unimplemented!(),
            _ => {
                let expr = self.parse_expression();
                self.consume(token::Kind::Semicolon);
                self.arena.push_epxression_statement(expr)
            }
        }
    }

    fn parse_block(&mut self) -> statement::Id {
        unimplemented!()
    }

    fn parse_expression(&mut self) -> expression::Id {
        self.parse_bp(0)
    }

    fn parse_lhs(&mut self) -> expression::Id {
        self.advance();
        match self.previous.get_kind() {
            token::Kind::Int64Lit(i) => self.arena.make_int64(i),
            token::Kind::Uint64Lit(u) => self.arena.make_uint64(u),
            token::Kind::FloatLit(f) => self.arena.make_float64(f),
            token::Kind::True => self.arena.make_bool(true),
            token::Kind::False => self.arena.make_bool(false),
            token::Kind::LParen => {
                let exp = self.parse_bp(0);
                self.consume(token::Kind::RParen);
                exp
            }
            t => {
                if let Some(op) = operator::Prefix::get(t) {
                    let rbp = op.get_bp();
                    let exp = self.parse_bp(rbp);
                    self.arena.make_prefix(op, exp)
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
                lhs = self.arena.make_postfix(op, lhs);
                continue;
            }

            if let Some(op) = operator::Infix::get(op) {
                let (lbp, rbp) = op.get_bp();
                if lbp < bp {
                    break;
                }

                self.advance();
                let rhs = self.parse_bp(rbp);
                lhs = self.arena.make_infix(op, lhs, rhs);
                continue;
            }

            match op {
                token::Kind::EOF | token::Kind::RParen | token::Kind::Semicolon => break,
                _ => {
                    panic!("Unexpected operation: {op:?}");
                }
            }
        }

        lhs
    }
}
