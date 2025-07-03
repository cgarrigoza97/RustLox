#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
    Eof,
}

pub struct Token {
    pub token_type: TokenType,
    pub start: usize,
    pub length: usize,
    pub line: i32,
    pub error_message: Option<u8>,
}

pub struct Scanner {
    pub source: String,
    pub start: usize,
    pub current: usize,
    pub line: u32,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            source: String::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn set_source(&mut self, source: String) {
        self.source = source;
    }

    pub fn get_token_value(&self, token: &Token) -> String {
        self.source
            .chars()
            .skip(token.start)
            .take(token.length)
            .collect()
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        let c = self.advance();
        if self.is_alpha(c.unwrap()) {
            return self.identifier();
        }

        if c.unwrap().is_ascii_digit() {
            return self.number();
        }

        match c {
            Some('(') => self.make_token(TokenType::LeftParen),
            Some(')') => self.make_token(TokenType::RightParen),
            Some('{') => self.make_token(TokenType::LeftBrace),
            Some('}') => self.make_token(TokenType::RightBrace),
            Some(';') => self.make_token(TokenType::Semicolon),
            Some(',') => self.make_token(TokenType::Comma),
            Some('.') => self.make_token(TokenType::Dot),
            Some('-') => self.make_token(TokenType::Minus),
            Some('+') => self.make_token(TokenType::Plus),
            Some('/') => self.make_token(TokenType::Slash),
            Some('*') => self.make_token(TokenType::Star),
            Some('!') => {
                let token_type = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };

                return self.make_token(token_type);
            }
            Some('=') => {
                let token_type = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };

                return self.make_token(token_type);
            }
            Some('<') => {
                let token_type = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };

                return self.make_token(token_type);
            }
            Some('>') => {
                let token_type = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };

                return self.make_token(token_type);
            }
            Some('"') => self.string(),
            _ => return self.error_token("Unexpected character."),
        };

        self.error_token("Unexpected character.")
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn make_token(&mut self, token_type: TokenType) -> Token {
        Token {
            token_type,
            start: self.start,
            length: self.current - self.start,
            line: self.line as i32,
            error_message: None,
        }
    }

    fn error_token(&self, message: &str) -> Token {
        Token {
            token_type: TokenType::Error,
            start: 0,
            length: message.len(),
            line: self.line as i32,
            error_message: Some(message.as_ptr() as u8),
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();

            match c {
                Some(' ') | Some('\r') | Some('\t') => {
                    self.advance();
                }
                Some('\n') => {
                    self.line += 1;
                    self.advance();
                }
                Some('/') => {
                    if self.peek_next().unwrap() == '/' {
                        while self.peek().unwrap() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => break,
            }
        }
    }

    fn check_keyword(
        &self,
        start: usize,
        length: usize,
        rest: &str,
        token_type: TokenType,
    ) -> TokenType {
        let string_to_compare: String = self
            .source
            .chars()
            .skip(self.start + start)
            .take(length)
            .collect();
        if self.current - self.start == start + length && string_to_compare == rest {
            return token_type;
        }

        TokenType::Identifier
    }

    fn identifier_type(&self) -> TokenType {
        match self.source.chars().nth(self.start) {
            Some('a') => return self.check_keyword(1, 2, "nd", TokenType::And),
            Some('c') => return self.check_keyword(1, 4, "lass", TokenType::Class),
            Some('e') => return self.check_keyword(1, 3, "lse", TokenType::Else),
            Some('f') => {
                if self.current - self.start > 1 {
                    match self.source.chars().nth(self.start + 1) {
                        Some('a') => return self.check_keyword(2, 3, "lse", TokenType::False),
                        Some('o') => return self.check_keyword(2, 1, "r", TokenType::For),
                        Some('u') => return self.check_keyword(2, 1, "n", TokenType::Fun),
                        _ => {}
                    }
                }
            }
            Some('i') => return self.check_keyword(1, 1, "f", TokenType::If),
            Some('n') => return self.check_keyword(1, 2, "il", TokenType::Nil),
            Some('o') => return self.check_keyword(1, 1, "r", TokenType::Or),
            Some('p') => return self.check_keyword(1, 4, "rint", TokenType::Print),
            Some('r') => return self.check_keyword(1, 5, "eturn", TokenType::Return),
            Some('s') => return self.check_keyword(1, 4, "uper", TokenType::Super),
            Some('t') => {
                if self.current - self.start > 1 {
                    match self.source.chars().nth(self.start + 1) {
                        Some('h') => return self.check_keyword(2, 2, "is", TokenType::This),
                        Some('r') => return self.check_keyword(2, 2, "ue", TokenType::True),
                        _ => {}
                    }
                }
            }
            Some('v') => return self.check_keyword(1, 2, "ar", TokenType::Var),
            Some('w') => return self.check_keyword(1, 5, "hile", TokenType::While),
            _ => {}
        };
        TokenType::Identifier
    }

    fn identifier(&mut self) -> Token {
        while self.is_alpha(self.peek().unwrap()) || self.peek().unwrap().is_ascii_digit() {
            self.advance();
        }

        self.make_token(self.identifier_type())
    }

    fn number(&mut self) -> Token {
        while self.peek().unwrap().is_ascii_digit() {
            self.advance();
        }

        if self.peek().unwrap() == '.' && self.peek_next().unwrap().is_ascii_digit() {
            self.advance();

            while self.peek().unwrap().is_ascii_digit() {
                self.advance();
            }
        }

        return self.make_token(TokenType::Number);
    }

    fn string(&mut self) -> Token {
        while self.peek().unwrap() != '"' && !self.is_at_end() {
            if self.peek().unwrap() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }

        // Closing quote reached.
        self.advance();

        return self.make_token(TokenType::String);
    }

    fn is_at_end(&self) -> bool {
        self.source.len() == self.current
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current - 1)
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn peek_next(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }

        return self.source.chars().nth(1);
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;

        return true;
    }
}
