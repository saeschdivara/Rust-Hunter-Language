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

// pub trait ExpressionVisitor {
//     fn accept_empty(&self, expr: &EmptyExpr);
//     fn accept_const(&self, expr: &ConstExpr);
//     fn accept_func(&self, expr: &FunctionExpr);
// }

pub trait Expression {
    // fn accept(&self, visitor: &dyn ExpressionVisitor);
    fn dump(&self) -> String;
}

impl Expression for EmptyExpr {
    // fn accept(&self, visitor: &dyn ExpressionVisitor) {
    //     visitor.accept_empty(self)
    // }

    fn dump(&self) -> String {
        String::from("<empty>")
    }
}

impl Expression for ConstExpr {
    // fn accept(&self, visitor: &dyn ExpressionVisitor) {
    //     visitor.accept_const(self)
    // }

    fn dump(&self) -> String {
        String::from("<empty>")
    }
}

impl Expression for FunctionExpr {
    // fn accept(&self, visitor: &dyn ExpressionVisitor) {
    //     visitor.accept_func(self)
    // }

    fn dump(&self) -> String {
        String::from(format!("<Func> {}", self.name))
    }
}

impl Expression for StringExpr {
    fn dump(&self) -> String {
        String::from(format!("<String> {}", self.value))
    }
}

impl Expression for PrintExpr {
    fn dump(&self) -> String {
        String::from(format!("<Print>"))
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
        } else if token.token_type == TokenType::PRINT {
            self.parse_print()
        } else if token.token_type == TokenType::STRING {
            self.parse_string(token)
        } else {
            Err("Could not parse an expression")
        }
    }

    fn parse_print(&mut self) -> Result<Box<dyn Expression>, &str> {
        let left_paren_token = self.advance();
        if left_paren_token.token_type != TokenType::LeftParen {
            return Err("Missing left paren after function name");
        }

        let right_paren_token = self.advance();
        if right_paren_token.token_type != TokenType::RightParen {
            return Err("Missing right paren after after all params");
        }

        Ok(Box::new(EmptyExpr{}))
    }

    fn parse_string(&mut self, token: &Token) -> Result<Box<dyn Expression>, &str> {
        Ok(Box::new(StringExpr{ value: token.lexeme.to_string() }))
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

        while is_body_parsing {
            for i in 0..level_number {
                next_token = self.advance();
            }
            let body_expr = self.parse_expr(&next_token);
            is_body_parsing = body_expr.is_ok();

            if is_body_parsing {
                body_expr_list.push(body_expr.unwrap());
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
        return self.current_token_index >= self.tokens.len();
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