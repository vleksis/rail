use crate::lexer::error::{LexError, Result};
use crate::lexer::token::{Token, TokenKind};

pub struct Lexer<'s> {
    source: &'s str,
    line: usize,
    start: usize,
    current_byte: usize,
}

impl<'s> Lexer<'s> {
    pub fn new(source: &'s str) -> Self {
        Self {
            source,
            line: 0,
            start: 0,
            current_byte: 0,
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }

        let ch = self.peek().expect("should be more chars");

        if ch == '\n' {
            self.line += 1;
        }

        self.current_byte += ch.len_utf8();

        Some(ch)
    }

    fn peek(&self) -> Option<char> {
        self.source[self.current_byte..].chars().next()
    }

    fn peek_next(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        let cur = self.peek().unwrap();
        let next_byte = self.current_byte + cur.len_utf8();
        self.source[next_byte..].chars().next()
    }

    fn is_at_end(&self) -> bool {
        self.current_byte >= self.source.len()
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            match ch {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }

                '\n' => {
                    self.line += 1;
                    self.advance();
                }

                _ => break,
            }
        }
    }

    fn match_token(&mut self, expected: char) -> bool {
        if let Some(cur) = self.peek() {
            if cur == expected {
                self.advance();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn scan_token(&mut self) -> Result<Token<'s>> {
        self.skip_whitespace();
        self.start = self.current_byte;

        if self.is_at_end() {
            return Ok(self.make_token(TokenKind::EOF));
        }

        let ch = self.advance().expect("not empty is checked");
        dbg!(ch);

        let result = match ch {
            '(' => self.make_token(TokenKind::LParen),
            ')' => self.make_token(TokenKind::RParen),
            '{' => self.make_token(TokenKind::LBrace),
            '}' => self.make_token(TokenKind::RBrace),
            ';' => self.make_token(TokenKind::Semicolon),
            ',' => self.make_token(TokenKind::Comma),
            ':' => self.make_token(TokenKind::Colon),
            '.' => self.make_token(TokenKind::Dot),

            '+' => {
                if self.match_token('=') {
                    self.make_token(TokenKind::PlusEqual)
                } else {
                    self.make_token(TokenKind::Plus)
                }
            }
            '-' => {
                if self.match_token('=') {
                    self.make_token(TokenKind::MinusEqual)
                } else if self.match_token('>') {
                    self.make_token(TokenKind::Arrow)
                } else {
                    self.make_token(TokenKind::Minus)
                }
            }
            '*' => {
                if self.match_token('=') {
                    self.make_token(TokenKind::StarEqual)
                } else {
                    self.make_token(TokenKind::Star)
                }
            }
            '/' => {
                if self.match_token('=') {
                    self.make_token(TokenKind::SlashEqual)
                } else if self.match_token('/') {
                    self.scan_comment()
                } else {
                    self.make_token(TokenKind::Slash)
                }
            }
            '%' => {
                if self.match_token('=') {
                    self.make_token(TokenKind::PercentEqual)
                } else {
                    self.make_token(TokenKind::Percent)
                }
            }
            '!' => {
                if self.match_token('=') {
                    self.make_token(TokenKind::BangEqual)
                } else {
                    self.make_token(TokenKind::Bang)
                }
            }
            '=' => {
                if self.match_token('=') {
                    self.make_token(TokenKind::EqualEqual)
                } else {
                    self.make_token(TokenKind::Equal)
                }
            }
            '<' => {
                if self.match_token('=') {
                    self.make_token(TokenKind::LessEqual)
                } else {
                    self.make_token(TokenKind::Less)
                }
            }
            '>' => {
                if self.match_token('=') {
                    self.make_token(TokenKind::GreaterEqual)
                } else {
                    self.make_token(TokenKind::Greater)
                }
            }
            '&' => {
                if self.match_token('&') {
                    self.make_token(TokenKind::AndAnd)
                } else {
                    return Err(self.make_error("Impossible &"));
                }
            }
            '|' => {
                if self.match_token('|') {
                    self.make_token(TokenKind::OrOr)
                } else {
                    return Err(self.make_error("Impossible |"));
                }
            }
            // TODO(vleksis): add number and types parsing
            c => {
                if c.is_numeric() {
                    loop {
                        let cur = self.peek().unwrap();
                        if cur.is_numeric() {
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    let number = &self.source[self.start..self.current_byte];
                    let number: i64 = number.parse().unwrap();

                    self.make_token(TokenKind::IntLit(number))
                } else {
                    loop {
                        let cur = self.peek().unwrap();
                        if cur.is_alphanumeric() {
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    self.make_token(TokenKind::ident_or_keyword(
                        &self.source[self.start..self.current_byte],
                    ))
                }
            }
        };

        dbg!(&result);

        Ok(result)
    }

    fn scan_comment(&mut self) -> Token<'s> {
        let line = self.line;
        while line == self.line {
            self.advance();
        }
        self.make_token(TokenKind::LineComment)
    }

    fn make_token(&self, kind: TokenKind) -> Token<'s> {
        let text = &self.source[self.start..self.current_byte];
        Token {
            kind,
            text,
            line: self.line,
            byte: self.start,
        }
    }

    fn make_error(&self, message: &str) -> LexError {
        eprintln!("{message}");
        LexError {}
    }
}

mod test {
    use super::*;

    #[test]
    fn simple_tokens() -> Result<()> {
        let mut lex = Lexer::new(
            "()     {}, :;.
            + += - -= *
            *= / /= % %=
        ! != = == < <= > >=
                    && ||",
        );

        assert_eq!(lex.scan_token()?.kind, TokenKind::LParen);
        assert_eq!(lex.scan_token()?.kind, TokenKind::RParen);
        assert_eq!(lex.scan_token()?.kind, TokenKind::LBrace);
        assert_eq!(lex.scan_token()?.kind, TokenKind::RBrace);
        assert_eq!(lex.scan_token()?.kind, TokenKind::Comma);
        assert_eq!(lex.scan_token()?.kind, TokenKind::Colon);
        assert_eq!(lex.scan_token()?.kind, TokenKind::Semicolon);
        assert_eq!(lex.scan_token()?.kind, TokenKind::Dot);
        assert_eq!(lex.scan_token()?.kind, TokenKind::Plus);
        assert_eq!(lex.scan_token()?.kind, TokenKind::PlusEqual);
        assert_eq!(lex.scan_token()?.kind, TokenKind::Minus);
        assert_eq!(lex.scan_token()?.kind, TokenKind::MinusEqual);
        assert_eq!(lex.scan_token()?.kind, TokenKind::Star);
        assert_eq!(lex.scan_token()?.kind, TokenKind::StarEqual);
        assert_eq!(lex.scan_token()?.kind, TokenKind::Slash);
        assert_eq!(lex.scan_token()?.kind, TokenKind::SlashEqual);
        assert_eq!(lex.scan_token()?.kind, TokenKind::Percent);
        assert_eq!(lex.scan_token()?.kind, TokenKind::PercentEqual);
        assert_eq!(lex.scan_token()?.kind, TokenKind::Bang);
        assert_eq!(lex.scan_token()?.kind, TokenKind::BangEqual);
        assert_eq!(lex.scan_token()?.kind, TokenKind::Equal);
        assert_eq!(lex.scan_token()?.kind, TokenKind::EqualEqual);
        assert_eq!(lex.scan_token()?.kind, TokenKind::Less);
        assert_eq!(lex.scan_token()?.kind, TokenKind::LessEqual);
        assert_eq!(lex.scan_token()?.kind, TokenKind::Greater);
        assert_eq!(lex.scan_token()?.kind, TokenKind::GreaterEqual);
        assert_eq!(lex.scan_token()?.kind, TokenKind::AndAnd);
        assert_eq!(lex.scan_token()?.kind, TokenKind::OrOr);
        assert_eq!(lex.scan_token()?.kind, TokenKind::EOF);

        Ok(())
    }
}
