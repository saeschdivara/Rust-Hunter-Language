use std::collections::HashMap;
use crate::expressions::{ConstExpr, EmptyExpr, Expression, ExpressionVisitor, FunctionExpr, IntExpr, PrintExpr, StringExpr, VariableExpr};
use crate::parser::{Ast};

pub trait RuntimeEngine {

    fn execute_ast(&self, ast: &Ast);
    fn execute_expr(&self, expr: &Box<dyn Expression>);

}

pub struct InterpreterRuntime {
    functions: HashMap<String, FunctionExpr>
}

impl InterpreterRuntime {
    pub fn new() -> Self {
        InterpreterRuntime {
            functions: HashMap::new()
        }
    }
}

impl RuntimeEngine for InterpreterRuntime {
    fn execute_ast(&self, ast: &Ast) {
        for expression in &ast.expressions {
            self.execute_expr(expression);
        }
    }

    fn execute_expr(&self, expr: &Box<dyn Expression>) {
        expr.accept(self)
    }
}

impl ExpressionVisitor for InterpreterRuntime {
    fn accept_empty(&self, expr: &EmptyExpr) {
        todo!("accept_empty")
    }

    fn accept_const(&self, expr: &ConstExpr) {
        todo!("accept_const")
    }

    fn accept_func(&self, expr: &FunctionExpr) {
        todo!("accept_func")
    }

    fn accept_print(&self, expr: &PrintExpr) {
        todo!()
    }

    fn accept_string(&self, expr: &StringExpr) {
        todo!()
    }

    fn accept_int(&self, expr: &IntExpr) {
        todo!()
    }

    fn accept_variable(&self, expr: &VariableExpr) {
        todo!()
    }
}