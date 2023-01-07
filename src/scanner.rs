use crate::error;
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

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.token()
        }

        self.add_token(Token::Eof);
        self.tokens.clone()
    }

    fn token(&mut self) {
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
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(Token::Slash)
                }
            }
            ' ' | '\r' | '\t' => (),
            '"' => self.string(),
            '\n' => self.line += 1,
            c if c.is_alphanumeric() || c == '_' => self.identifier(),
            c if c.is_digit(10) => self.number(),
            x => unreachable!("Unexpected character. '{x} "),
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            error(self.line as u32, "Unterminated string.");
            return;
        }

        self.advance();

        let value = String::from_iter(self.source[self.start + 1..self.current - 1].iter());
        self.add_token(Token::String { value });
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let name = String::from_iter(self.source[self.start..self.current].iter());
        match keyword(&name) {
            Some(token) => self.add_token(token),
            None => self.add_token(Token::Identifier { name })
        }
    }

    fn add_token(&mut self, token: Token) {
        &self.tokens.push(token);
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
            _ => false
        }
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            *self.source.get(self.current).unwrap()
        }
    }
    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let val = String::from_iter(self.source[self.start..self.current].iter()).parse::<f32>().unwrap();
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
        _ => None
    }
}