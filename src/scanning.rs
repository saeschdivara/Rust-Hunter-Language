use std::fmt::{Debug, Formatter};
use log::info;
use crate::reporting::CodeReporter;

#[derive(PartialEq)]
pub enum TokenType {
    INVALID,

    // Special token types
    COMMENT, LineBreak, SPACE, SpaceLevel,

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
        let mut format_value: String;
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
            TokenType::IDENTIFIER => {
                format_value = format!("Identifier (\"{}\")", &self.lexeme);
                format_value.as_str()
            },
            TokenType::STRING => {
                format_value = format!("String (\"{}\")", &self.lexeme);
                format_value.as_str()
            },
            TokenType::INT => "",
            TokenType::FLOAT => "",
            TokenType::CONST => "keyword: const",
            TokenType::FUNCTION => "keyword: fun",
            TokenType::PRINT => "keyword: print",
            TokenType::SpaceLevel => ">",
            TokenType::SPACE => "<SPACE>",
            TokenType::EOF => "",
            _ => "",
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
            self.start = self.current;
            let token = self.scan_token();

            match token.token_type {
                TokenType::COMMENT => {}
                TokenType::SPACE => {}
                TokenType::LineBreak => { self.line += 1; }
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
            '\r' => TokenType::SPACE,
            '\n' => TokenType::LineBreak,
            '\t' => TokenType::SpaceLevel,
            ' ' => {
                if self.matches_character(' ') { TokenType::SpaceLevel } else { TokenType::SPACE }
            },
            '"' => self.scan_string_token(),
            _   => {

                if character.is_alphabetic() {
                    self.scan_identifier()
                } else {
                    TokenType::INVALID
                }

            },
        };

        return self.create_token(token_type)
    }

    fn scan_identifier(&mut self) -> TokenType {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let token_str = self.get_current_token_string();

        match token_str.as_str() {
            "const" => TokenType::CONST,
            "fun" => TokenType::FUNCTION,
            "print" => TokenType::PRINT,
            _ => TokenType::IDENTIFIER
        }
    }

    fn scan_string_token(&mut self) -> TokenType {
        while self.peek() != '"' && !self.is_at_end() {

            if self.peek() == '\n' {
                self.reporter.report_error(&self.file_name, self.line, &String::from("Broken string"));
                return TokenType::INVALID
            }

            self.advance();
        }

        if self.is_at_end() {
            self.reporter.report_error(&self.file_name, self.line, &String::from("Broken string"));
            return TokenType::INVALID
        }

        // The closing "
        self.advance();

        return TokenType::STRING
    }

    fn create_token(&self, token_type: TokenType) -> Token {
        let mut range_beginning = self.start;
        let mut range_ending = self.current;

        if token_type == TokenType::STRING {
            range_beginning = range_beginning + 1;
            range_ending = range_ending - 1;
        }

        Token::new(token_type,self.source[range_beginning..range_ending].to_string(), self.line)
    }

    fn get_current_token_string(&self) -> String {
        return self.source[self.start..self.current].to_string()
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

    fn look_back(&mut self) -> char {
        if self.is_at_end() { return '\0' }
        if self.current == 0 { return '\0' }
        let previous_index = self.current - 1;
        return self.source.chars().nth(previous_index).unwrap();
    }

    fn matches_character(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false }
        let character = self.source.chars().nth(self.current as usize).unwrap();
        if character != expected { return false }

        self.current += 1;

        return true
    }
}