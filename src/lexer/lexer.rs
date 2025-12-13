use super::*;
use token::Kind;
use token::Token;

#[derive(Debug)]
pub struct Lexer<'s> {
    source: &'s str,
    line: usize,
    start: usize,
    current_byte: usize,
}

#[derive(Debug)]
enum NumberPrefix {
    Bin,  // "0b"
    Oct,  // "0o"
    Hex,  // "0x"
    None, // ""
}

impl NumberPrefix {
    pub fn to_num(self) -> u32 {
        match self {
            Self::Bin => 2,
            Self::Oct => 8,
            Self::Hex => 16,
            Self::None => 10,
        }
    }
}

#[derive(Debug)]
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
            return Ok(self.make_token(Kind::EOF));
        }

        let ch = self.advance().expect("not empty is checked");

        let result = match ch {
            '(' => self.make_token(Kind::LParen),
            ')' => self.make_token(Kind::RParen),
            '{' => self.make_token(Kind::LBrace),
            '}' => self.make_token(Kind::RBrace),
            ';' => self.make_token(Kind::Semicolon),
            ',' => self.make_token(Kind::Comma),
            ':' => self.make_token(Kind::Colon),
            '.' => self.make_token(Kind::Dot),

            '+' => {
                if self.match_token('=') {
                    self.make_token(Kind::PlusEqual)
                } else {
                    self.make_token(Kind::Plus)
                }
            }
            '-' => {
                if self.match_token('=') {
                    self.make_token(Kind::MinusEqual)
                } else if self.match_token('>') {
                    self.make_token(Kind::Arrow)
                } else {
                    self.make_token(Kind::Minus)
                }
            }
            '*' => {
                if self.match_token('=') {
                    self.make_token(Kind::StarEqual)
                } else {
                    self.make_token(Kind::Star)
                }
            }
            '/' => {
                if self.match_token('=') {
                    self.make_token(Kind::SlashEqual)
                } else if self.match_token('/') {
                    self.scan_comment()
                } else {
                    self.make_token(Kind::Slash)
                }
            }
            '%' => {
                if self.match_token('=') {
                    self.make_token(Kind::PercentEqual)
                } else {
                    self.make_token(Kind::Percent)
                }
            }
            '!' => {
                if self.match_token('=') {
                    self.make_token(Kind::BangEqual)
                } else {
                    self.make_token(Kind::Bang)
                }
            }
            '=' => {
                if self.match_token('=') {
                    self.make_token(Kind::EqualEqual)
                } else {
                    self.make_token(Kind::Equal)
                }
            }
            '<' => {
                if self.match_token('=') {
                    self.make_token(Kind::LessEqual)
                } else {
                    self.make_token(Kind::Less)
                }
            }
            '>' => {
                if self.match_token('=') {
                    self.make_token(Kind::GreaterEqual)
                } else {
                    self.make_token(Kind::Greater)
                }
            }
            '&' => {
                if self.match_token('&') {
                    self.make_token(Kind::AndAnd)
                } else {
                    return Err(self.make_error("Impossible &"));
                }
            }
            '|' => {
                if self.match_token('|') {
                    self.make_token(Kind::OrOr)
                } else {
                    return Err(self.make_error("Impossible |"));
                }
            }

            c => {
                if c.is_ascii_digit() {
                    self.current_byte -= 1;
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
        self.make_token(Kind::LineComment)
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
        let radix = dbg!(self.scan_number_prefix()).to_num();
        let has_dot = dbg!(self.scan_number_core());
        let mut num = dbg!(self.get_text());
        if radix != 10 {
            num = &num[2..];
        }
        let post = dbg!(self.scan_number_postfix());
        let kind = match post {
            NumberPostfix::Int64 => Kind::Int64Lit(i64::from_str_radix(num, radix).unwrap()),
            NumberPostfix::Uint64 => Kind::Uint64Lit(u64::from_str_radix(num, radix).unwrap()),
            NumberPostfix::Float64 => Kind::FloatLit(num.parse().unwrap()),
            NumberPostfix::None => match has_dot {
                true => Kind::FloatLit(num.parse().unwrap()),
                false => Kind::Int64Lit(i64::from_str_radix(num, radix).unwrap()),
            },
        };

        self.make_token(kind)
    }

    fn scan_ident_or_keyword(&mut self) -> Token<'s> {
        while self.peek().unwrap().is_alphanumeric() {
            self.advance();
        }
        let text = self.get_text();
        self.make_token(Kind::ident_or_keyword(text))
    }

    fn get_text(&self) -> &'s str {
        &self.source[self.start..self.current_byte]
    }

    fn make_token(&self, kind: Kind) -> Token<'s> {
        Token {
            kind,
            text: self.get_text(),
            line: self.line,
            byte: self.start,
        }
    }

    fn make_error(&self, message: &str) -> Error {
        eprintln!("{message}");
        Error {}
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

        assert_eq!(lex.scan_token()?.kind, Kind::LParen);
        assert_eq!(lex.scan_token()?.kind, Kind::RParen);
        assert_eq!(lex.scan_token()?.kind, Kind::LBrace);
        assert_eq!(lex.scan_token()?.kind, Kind::RBrace);
        assert_eq!(lex.scan_token()?.kind, Kind::Comma);
        assert_eq!(lex.scan_token()?.kind, Kind::Colon);
        assert_eq!(lex.scan_token()?.kind, Kind::Semicolon);
        assert_eq!(lex.scan_token()?.kind, Kind::Dot);
        assert_eq!(lex.scan_token()?.kind, Kind::Plus);
        assert_eq!(lex.scan_token()?.kind, Kind::PlusEqual);
        assert_eq!(lex.scan_token()?.kind, Kind::Minus);
        assert_eq!(lex.scan_token()?.kind, Kind::MinusEqual);
        assert_eq!(lex.scan_token()?.kind, Kind::Star);
        assert_eq!(lex.scan_token()?.kind, Kind::StarEqual);
        assert_eq!(lex.scan_token()?.kind, Kind::Slash);
        assert_eq!(lex.scan_token()?.kind, Kind::SlashEqual);
        assert_eq!(lex.scan_token()?.kind, Kind::Percent);
        assert_eq!(lex.scan_token()?.kind, Kind::PercentEqual);
        assert_eq!(lex.scan_token()?.kind, Kind::Bang);
        assert_eq!(lex.scan_token()?.kind, Kind::BangEqual);
        assert_eq!(lex.scan_token()?.kind, Kind::Equal);
        assert_eq!(lex.scan_token()?.kind, Kind::EqualEqual);
        assert_eq!(lex.scan_token()?.kind, Kind::Less);
        assert_eq!(lex.scan_token()?.kind, Kind::LessEqual);
        assert_eq!(lex.scan_token()?.kind, Kind::Greater);
        assert_eq!(lex.scan_token()?.kind, Kind::GreaterEqual);
        assert_eq!(lex.scan_token()?.kind, Kind::AndAnd);
        assert_eq!(lex.scan_token()?.kind, Kind::OrOr);
        assert_eq!(lex.scan_token()?.kind, Kind::EOF);

        Ok(())
    }
}
