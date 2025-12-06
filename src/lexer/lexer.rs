use crate::lexer::error::{LexError, Result};
use crate::lexer::token::{Token, TokenKind};

pub struct Lexer<'s> {
    source: &'s str,
    line: usize,
    start: usize,
    current_byte: usize,
}

enum NumberPrefix {
    Bin,  // "0b"
    Oct,  // "0o"
    Hex,  // "0x"
    None, // ""
}

enum NumberPostfix {
    Int64,   // "i64"
    Uint64,  // "u64"
    Float64, // "f64"
    None,    // ""
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

    fn multi_match(&mut self, expected: &'static str) -> bool {
        let byte = self.current_byte;

        for ch in expected.chars() {
            if ch == self.peek().unwrap() {
                self.advance();
            } else {
                self.current_byte = byte;
                return false;
            }
        }

        true
    }

    pub fn scan_token(&mut self) -> Result<Token<'s>> {
        self.skip_whitespace();
        self.start = self.current_byte;

        if self.is_at_end() {
            return Ok(self.make_token(TokenKind::EOF));
        }

        let ch = self.advance().expect("not empty is checked");

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

            c => {
                if c.is_ascii_digit() {
                    self.scan_numeric()
                } else {
                    self.scan_ident_or_keyword()
                }
            }
        };

        Ok(result)
    }

    fn scan_comment(&mut self) -> Token<'s> {
        let line = self.line;
        while line == self.line {
            self.advance();
        }
        self.make_token(TokenKind::LineComment)
    }

    fn scan_number_prefix(&mut self) -> NumberPrefix {
        if self.multi_match("0b") {
            NumberPrefix::Bin
        } else if self.multi_match("0o") {
            NumberPrefix::Oct
        } else if self.multi_match("0x") {
            NumberPrefix::Hex
        } else {
            NumberPrefix::None
        }
    }

    /// Return true if number has dot
    fn scan_number_core(&mut self) -> bool {
        while self.peek().unwrap().is_numeric() {
            self.advance();
        }

        if self.match_token('.') {
            while self.peek().unwrap().is_numeric() {
                self.advance();
            }

            true
        } else {
            false
        }
    }

    fn scan_number_postfix(&mut self) -> NumberPostfix {
        if self.multi_match("f64") {
            NumberPostfix::Float64
        } else if self.multi_match("i64") {
            NumberPostfix::Int64
        } else if self.multi_match("u64") {
            NumberPostfix::Uint64
        } else {
            NumberPostfix::None
        }
    }

    fn scan_numeric(&mut self) -> Token<'s> {
        let pref = self.scan_number_prefix();
        let has_dot = self.scan_number_core();
        let num = self.get_text();
        let post = self.scan_number_postfix();
        let kind = match post {
            NumberPostfix::Int64 => TokenKind::Int64Lit(num.parse().unwrap()),
            NumberPostfix::Uint64 => TokenKind::Uint64Lit(num.parse().unwrap()),
            NumberPostfix::Float64 => TokenKind::FloatLit(num.parse().unwrap()),
            NumberPostfix::None => match has_dot {
                true => TokenKind::FloatLit(num.parse().unwrap()),
                false => TokenKind::Int64Lit(num.parse().unwrap()),
            },
        };

        self.make_token(kind)
    }

    fn scan_ident_or_keyword(&mut self) -> Token<'s> {
        while self.peek().unwrap().is_alphanumeric() {
            self.advance();
        }
        let text = self.get_text();
        self.make_token(TokenKind::ident_or_keyword(text))
    }

    fn get_text(&self) -> &'s str {
        &self.source[self.start..self.current_byte]
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
