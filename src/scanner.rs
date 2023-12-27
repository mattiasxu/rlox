use crate::token::{Token, TokenType, Literal};
use crate::errors::error;

struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new(source: String) -> Scanner {
        let source_chars = source.chars().collect();
        Scanner { source: source_chars, tokens: Vec::new(), start: 0, current: 0, line: 1 }
    }

    fn scan_tokens(&self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::EOF, String::from(""), None, self.line));
        return self.tokens.clone();
    }

    fn scan_token(&self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => self.add_token(
                if self.check('=') { TokenType::BangEqual } else { TokenType::Bang }
            ),
            '=' => self.add_token(
                if self.check('=') { TokenType::EqualEqual } else { TokenType::Equal }
            ),
            '<' => self.add_token(
                if self.check('=') { TokenType::LessEqual } else { TokenType::Less }
            ),
            '>' => self.add_token(
                if self.check('=') { TokenType::GreaterEqual } else { TokenType::Greater }
            ),
            '/' => if self.check('/') {
                while self.peek() != '\n' && !self.is_at_end() { self.advance(); }
            } else {
                self.add_token(TokenType::Slash);
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => if c.is_digit(10) {
                self.number();
            } else if c.is_alphanumeric() {
                self.identifier();
            } else {
                error(self.line, "Unexpected character");
            },
        }
    }

    fn identifier(&self) {
        while self.peek().is_alphanumeric() { self.advance(); }

        let text = self.source[self.start..self.current].iter().collect::<String>();
        let token_type = keyword_to_token_type(&text);

        self.add_token(token_type);
    }

    fn number(&self) {
        while self.peek().is_digit(10) { self.advance(); }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) { self.advance(); }
        }

        match self.source[self.start..self.current].iter().collect::<String>().parse() {
            Ok(num) => self.add_token_with_literal(TokenType::Number, Some(Literal::Float(num))),
            Err(_) => error(self.line, "Failed to parse number"),
        }
    }

    fn string(&self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {self.line += 1}
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "Unterminated string");
        }

        self.advance();
        let value = self.source[self.start + 1..self.current - 1].iter().collect();
        self.add_token_with_literal(TokenType::String, Some(Literal::Str(value)));
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn advance(&self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        return c;
    }

    fn add_token(&self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None);
    }

    fn add_token_with_literal(&self, token_type: TokenType, literal: Option<Literal>) {
        let text: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token { token_type, lexeme: text, literal, line: 0 });
    }

    fn check(&self, expected: char) -> bool {
        if self.is_at_end() { return false }
        if self.source[self.current] != expected { return false }

        self.current += 1;
        return true;
    }

    fn peek(&self) -> char {
        if self.is_at_end() { return '\0' }
        return self.source[self.current];
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() { return '\0'; }
        return self.source[self.current + 1];
    }
}

fn keyword_to_token_type(keyword: &str) -> TokenType {
    match keyword {
        "and" => TokenType::And,
        "class" => TokenType::Class,
        "else" => TokenType::Else,
        "false" => TokenType::False,
        "for" => TokenType::For,
        "fun" => TokenType::Fun,
        "if" => TokenType::If,
        "nil" => TokenType::Nil,
        "or" => TokenType::Or,
        "print" => TokenType::Print,
        "return" => TokenType::Return,
        "super" => TokenType::Super,
        "this" => TokenType::This,
        "true" => TokenType::True,
        "var" => TokenType::Var,
        "while" => TokenType::While,
        _ => TokenType::Identifier, // Return None if the keyword is not found
    }
}
