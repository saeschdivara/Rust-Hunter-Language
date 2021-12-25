use std::fmt::{Debug, Formatter};
use log::info;
use crate::reporting::CodeReporter;

pub enum TokenType {
    INVALID,

    // Special token types
    COMMENT, LineBreak,

    // Single-character tokens.
    COMMA, DOT, LeftParen, RightParen,
    MINUS, PLUS,

    // One or two character tokens.
    EQUAL, BANG, BangEqual, EqualEqual,

    // Literals.
    IDENTIFIER, STRING, INT, FLOAT,

    // Keywords.
    CONST, FUNCTION,
    PRINT,

    EOF,
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Token { token_type, lexeme, line }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let output= match self.token_type {
            TokenType::INVALID => "Invalid token",
            TokenType::COMMA => ",",
            TokenType::DOT => ".",
            TokenType::LeftParen => "(",
            TokenType::RightParen => ")",
            TokenType::MINUS => "",
            TokenType::PLUS => "",
            TokenType::EQUAL => "=",
            TokenType::BANG => "!",
            TokenType::BangEqual => "!=",
            TokenType::EqualEqual => "==",
            TokenType::IDENTIFIER => "",
            TokenType::STRING => "",
            TokenType::INT => "",
            TokenType::FLOAT => "",
            TokenType::CONST => "",
            TokenType::FUNCTION => "",
            TokenType::PRINT => "",
            TokenType::EOF => "",
        };

        f.write_str(output)
    }
}

pub struct Scanner {
    reporter: CodeReporter,

    source: String,
    file_name: String,

    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(reporter: CodeReporter) -> Self {
        Scanner {
            reporter: reporter,

            source: String::new(),
            file_name: String::new(),

            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_string(&mut self, file_name: String, input: String) -> Vec<Token> {

        self.file_name = file_name;
        self.source = input;

        let mut tokens = vec![];

        while !self.is_at_end() {
            let token = self.scan_token();

            match token.token_type {
                TokenType::COMMENT => {}
                _ => tokens.push(token)
            }

        }

        return tokens;
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len() as usize
    }

    fn scan_token(&mut self) -> Token {
        self.start = self.current;
        let character = self.advance();

        let token_type = match character {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            ',' => TokenType::COMMA,
            '.' => TokenType::DOT,
            '!' => if self.matches_character('=') { TokenType::BangEqual } else { TokenType::BANG },
            '=' => if self.matches_character('=') { TokenType::EqualEqual } else { TokenType::EQUAL },
            '#' => { while self.peek() != '\n' { self.advance(); } TokenType::COMMENT },
            _   => TokenType::INVALID,
        };

        return self.create_token(token_type)
    }

    fn create_token(&self, token_type: TokenType) -> Token {
        Token::new(token_type,self.source[self.start..self.current].to_string(), self.line)
    }

    fn advance(&mut self) -> char {
        let character = self.source.chars().nth(self.current as usize).unwrap();
        self.current += 1;
        return character;
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() { return '\0' }
        return self.source.chars().nth(self.current as usize).unwrap();
    }

    fn matches_character(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false }
        let character = self.source.chars().nth(self.current as usize).unwrap();
        if character != expected { return false }

        self.current += 1;

        return true
    }
}