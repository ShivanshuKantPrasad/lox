use crate::error::LoxError;
use crate::token::Token;

#[derive()]
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: Vec<char>) -> Scanner {
        Scanner {
            source,
            tokens: Default::default(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        let mut had_error: Option<LoxError> = None;

        while !self.is_at_end() {
            self.start = self.current;
            match self.token() {
                Ok(_) => {}
                Err(e) => {
                    e.report("".to_string());
                    had_error = Some(e);
                }
            }
        }

        self.add_token(Token::Eof);

        if let Some(e) = had_error {
            Err(e)
        } else {
            Ok(&self.tokens)
        }
    }

    fn token(&mut self) -> Result<(), LoxError> {
        match self.advance() {
            '(' => self.add_token(Token::LeftParen),
            ')' => self.add_token(Token::RightParen),
            '{' => self.add_token(Token::LeftBrace),
            '}' => self.add_token(Token::RightBrace),
            ',' => self.add_token(Token::Comma),
            '.' => self.add_token(Token::Dot),
            '+' => self.add_token(Token::Plus),
            '-' => self.add_token(Token::Minus),
            '*' => self.add_token(Token::Star),
            ';' => self.add_token(Token::SemiColon),
            '!' => {
                if self.is_match('=') {
                    self.add_token(Token::BangEqual)
                } else {
                    self.add_token(Token::Bang)
                }
            }
            '=' => {
                if self.is_match('=') {
                    self.add_token(Token::Equals)
                } else {
                    self.add_token(Token::Assign)
                }
            }
            '>' => {
                if self.is_match('=') {
                    self.add_token(Token::GreaterEqual)
                } else {
                    self.add_token(Token::Greater)
                }
            }
            '<' => {
                if self.is_match('=') {
                    self.add_token(Token::LessEqual)
                } else {
                    self.add_token(Token::Less)
                }
            }
            '/' => {
                if self.is_match('/') {
                    while let Some(ch) = self.peek() {
                        if ch != '\n' {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                } else {
                    self.add_token(Token::Slash)
                }
            }
            ' ' | '\r' | '\t' => (),
            '"' => self.string()?,
            '\n' => self.line += 1,
            c if c.is_alphanumeric() || c == '_' => self.identifier(),
            c if c.is_digit(10) => self.number(),
            x => {
                return Err(LoxError::error(
                    self.line,
                    format!("Unexpected character. '{x} "),
                ));
            }
        }

        Ok(())
    }

    fn string(&mut self) -> Result<(), LoxError> {
        while let Some(ch) = self.peek() {
            match ch {
                '"' => break,
                '\n' => self.line += 1,
                _ => {}
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(LoxError::error(self.line, "Unterminated string.".to_string()));
        }

        self.advance();

        let value = String::from_iter(self.source[self.start + 1..self.current - 1].iter());
        self.add_token(Token::String { value });
        Ok(())
    }

    fn identifier(&mut self) {
        while is_alphanumeric(self.peek()) {
            self.advance();
        }

        let name = String::from_iter(self.source[self.start..self.current].iter());
        match keyword(&name) {
            Some(token) => self.add_token(token),
            None => self.add_token(Token::Identifier { name }),
        }
    }

    fn add_token(&mut self, token: Token) {
        let _ = &self.tokens.push(token);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    fn advance(&mut self) -> char {
        let result = *self.source.get(self.current).unwrap();
        self.current += 1;
        result
    }
    fn is_match(&mut self, expected: char) -> bool {
        match self.source.get(self.current) {
            Some(ch) if *ch == expected => {
                self.current += 1;
                true
            }
            _ => false,
        }
    }
    fn peek(&self) -> Option<char> {
        self.source.get(self.current).copied()
    }

    fn number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == Some('.') && self.peek_next().is_digit(10) {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }

        let val = String::from_iter(self.source[self.start..self.current].iter())
            .parse::<f32>()
            .unwrap();
        self.add_token(Token::Number { val })
    }
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            *self.source.get(self.current + 1).unwrap()
        }
    }
}

fn keyword(text: &String) -> Option<Token> {
    match text.as_str() {
        "and" => Some(Token::And),
        "class" => Some(Token::Class),
        "else" => Some(Token::Else),
        "false" => Some(Token::False),
        "for" => Some(Token::For),
        "fun" => Some(Token::Fun),
        "if" => Some(Token::If),
        "nil" => Some(Token::Nil),
        "or" => Some(Token::Or),
        "print" => Some(Token::Print),
        "return" => Some(Token::Return),
        "super" => Some(Token::Super),
        "this" => Some(Token::This),
        "true" => Some(Token::True),
        "var" => Some(Token::Var),
        "while" => Some(Token::While),
        _ => None,
    }
}

fn is_digit(ch: Option<char>) -> bool {
    if let Some(ch) = ch {
        ch.is_digit(10)
    } else {
        false
    }
}

fn is_alphanumeric(ch: Option<char>) -> bool {
    if let Some(ch) = ch {
        ch.is_alphanumeric()
    } else {
        false
    }
}
