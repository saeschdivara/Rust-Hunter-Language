use std::borrow::Borrow;
use log::info;
use crate::reporter::CodeReporter;
use crate::scanner::{Token, TokenType};

pub struct EmptyExpr{}

pub struct FunctionExpr {
    name: String,
    return_type: Box<dyn Expression>,
    body: Vec<Box<dyn Expression>>,
}

pub struct PrintExpr {
    values: Vec<Box<dyn Expression>>
}

pub struct ConstExpr {
    variable: String,
    value: Box<dyn Expression>
}

pub struct StringExpr {
    value: String,
}

pub struct VariableExpr {
    name: String,
}

pub struct IntExpr {
    value: i64,
}

pub trait Expression {
    fn dump(&self) -> String;
}

impl Expression for EmptyExpr {
    fn dump(&self) -> String {
        String::from("<empty>")
    }
}

impl Expression for ConstExpr {
    fn dump(&self) -> String {
        String::from(format!("<Const> {} = {}", &self.variable, &self.value.dump()))
    }
}

impl Expression for FunctionExpr {
        fn dump(&self) -> String {
        let mut parameters_output = String::from("");

        for body_expr in &self.body {
            parameters_output += &*String::from(format!("<Body-Expr> {}\n", body_expr.dump()));
        }

        String::from(format!("<Func> {}\n{}", self.name, parameters_output))
    }
}

impl Expression for StringExpr {
    fn dump(&self) -> String {
        String::from(format!("<String> \"{}\"", self.value))
    }
}

impl Expression for IntExpr {
    fn dump(&self) -> String {
        String::from(format!("<Int> \"{}\"", self.value))
    }
}

impl Expression for VariableExpr {
    fn dump(&self) -> String {
        String::from(format!("<Variable> \"{}\"", self.name))
    }
}

impl Expression for PrintExpr {
    fn dump(&self) -> String {
        let mut parameters_output = String::from("");

        for value_expr in &self.values {
            parameters_output += &*String::from(format!("{}, ", value_expr.dump()));
        }

        String::from(format!("<Print> {}", parameters_output))
    }
}

pub struct Ast {
    pub expressions: Vec<Box<dyn Expression>>
}

impl Ast {
    pub fn dump(&self) {
        for expression in &self.expressions {
            info!("Expr: {}", expression.dump());
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current_token_index: usize,
    current_level: i32,
}

impl Parser {

    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens: tokens, current_token_index: 0, current_level: 0 }
    }

    pub fn parse_ast(&mut self) -> Ast {

        let mut reporter = CodeReporter::new();
        let mut ast = Ast{ expressions: vec![] };

        while !self.is_at_end() {
            let token = self.advance();
            let expr = self.parse_expr(&token);

            if token.token_type == TokenType::LineBreak {
                continue
            }

            match expr {
                Ok(..) => ast.expressions.push(expr.unwrap()),
                Err(..) => {
                    reporter.report_error(
                        &token.file_name,
                        token.line,
                        &String::from(expr.err().unwrap())
                    );
                }
            }
        }

        ast
    }

    fn parse_expr(&mut self, token: &Token) -> Result<Box<dyn Expression>, &str> {
        if token.token_type == TokenType::FUNCTION {
            self.parse_function()
        } else if token.token_type == TokenType::CONST {
            self.parse_const()
        } else if token.token_type == TokenType::PRINT {
            self.parse_print()
        } else if token.token_type == TokenType::STRING {
            self.parse_string(token)
        } else if token.token_type == TokenType::INT {
            self.parse_int(token)
        } else if token.token_type == TokenType::IDENTIFIER {
            self.parse_variable(token)
        } else {
            Err("Could not parse an expression")
        }
    }

    fn parse_print(&mut self) -> Result<Box<dyn Expression>, &str> {
        let left_paren_token = self.advance();
        if left_paren_token.token_type != TokenType::LeftParen {
            return Err("Missing left paren after function name");
        }

        let mut values: Vec<Box<dyn Expression>> = vec![];
        let mut current_token = self.advance();
        while current_token.token_type != TokenType::RightParen && !self.is_at_end() {
            let expr_result = self.parse_expr(&current_token);
            if expr_result.is_ok() {
                values.push(expr_result.unwrap());
                current_token = self.advance();

                if current_token.token_type != TokenType::COMMA && current_token.token_type != TokenType::RightParen {
                    return Err("Comma missing after parameter")
                } else if current_token.token_type == TokenType::COMMA {
                    current_token = self.advance();
                }
            } else {
                break;
            }

        }

        Ok(Box::new(PrintExpr { values: values }))
    }

    fn parse_const(&mut self) -> Result<Box<dyn Expression>, &str> {
        let identifier_token = self.advance();
        if identifier_token.token_type != TokenType::IDENTIFIER {
            return Err("Missing identifier after function keyword");
        }

        let equal_token = self.advance();
        if equal_token.token_type != TokenType::EQUAL {
            return Err("Missing equal after identifier");
        }

        let token = self.advance();
        let value = self.parse_expr(&token);

        if value.is_err() {
            return Err(value.err().unwrap())
        }

        Ok(Box::new(ConstExpr{ variable: identifier_token.lexeme, value: value.unwrap() }))
    }

    fn parse_string(&mut self, token: &Token) -> Result<Box<dyn Expression>, &str> {
        Ok(Box::new(StringExpr{ value: token.lexeme.to_string() }))
    }

    fn parse_int(&mut self, token: &Token) -> Result<Box<dyn Expression>, &str> {
        Ok(Box::new(IntExpr{ value: token.lexeme.to_string().parse().unwrap() }))
    }

    fn parse_variable(&mut self, token: &Token) -> Result<Box<dyn Expression>, &str> {
        Ok(Box::new(VariableExpr{ name: token.lexeme.to_string() }))
    }

    fn parse_function(&mut self) -> Result<Box<dyn Expression>, &str> {

        let identifier_token = self.advance();
        if identifier_token.token_type != TokenType::IDENTIFIER {
            return Err("Missing identifier after function keyword");
        }

        let left_paren_token = self.advance();
        if left_paren_token.token_type != TokenType::LeftParen {
            return Err("Missing left paren after function name");
        }

        let right_paren_token = self.advance();
        if right_paren_token.token_type != TokenType::RightParen {
            return Err("Missing right paren after after all params");
        }

        // ignore line break
        self.advance();

        let level1 = self.advance();
        if level1.token_type != TokenType::SpaceLevel {
            return Err("Function body is missing");
        }

        let mut level_number = 1;
        let mut next_token = self.peek();

        while next_token.token_type == TokenType::SpaceLevel {
            level_number += 1;
            next_token = self.advance();
        }

        self.current_level += level_number;

        let mut body_expr_list: Vec<Box<dyn Expression>> = vec![];
        let body_expr = self.parse_expr(&next_token);
        let mut is_body_parsing = body_expr.is_ok();

        if is_body_parsing {
            body_expr_list.push(body_expr.unwrap());
            // ignore line breaks
            self.advance();
        }

        while is_body_parsing && !self.is_at_end() {
            for i in 0..level_number {
                next_token = self.advance();
            }
            let body_expr = self.parse_expr(&next_token);
            is_body_parsing = body_expr.is_ok();

            if is_body_parsing {
                body_expr_list.push(body_expr.unwrap());
                // ignore line break
                self.advance();
            }
        }

        self.current_level -= level_number;

        Ok(Box::new(FunctionExpr {
            name: identifier_token.lexeme.to_string(),
            return_type: Box::new(EmptyExpr{}),
            body: body_expr_list
        }))
    }

    fn is_at_end(&self) -> bool {
        if self.current_token_index >= self.tokens.len() { return true }
        else { self.peek().token_type == TokenType::EOF }
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.current_token_index].borrow();
        self.current_token_index += 1;
        return Token::new(token.token_type, token.lexeme.to_string(), token.file_name.to_string(), token.line);
    }

    fn peek(&self) -> Token {
        let token = self.tokens[self.current_token_index].borrow();
        return Token::new(token.token_type, token.lexeme.to_string(), token.file_name.to_string(), token.line);
    }
}