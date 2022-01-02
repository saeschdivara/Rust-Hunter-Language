
pub trait ExpressionVisitor {
    fn accept_empty(&self, expr: &EmptyExpr);
    fn accept_const(&self, expr: &ConstExpr);
    fn accept_func(&self, expr: &FunctionExpr);
    fn accept_print(&self, expr: &PrintExpr);
    fn accept_string(&self, expr: &StringExpr);
    fn accept_int(&self, expr: &IntExpr);
    fn accept_variable(&self, expr: &VariableExpr);
}

pub struct EmptyExpr{}

pub struct FunctionExpr {
    pub name: String,
    pub return_type: Box<dyn Expression>,
    pub body: Vec<Box<dyn Expression>>,
}

pub struct PrintExpr {
    pub values: Vec<Box<dyn Expression>>
}

pub struct ConstExpr {
    pub variable: String,
    pub value: Box<dyn Expression>
}

pub struct StringExpr {
    pub value: String,
}

pub struct VariableExpr {
    pub name: String,
}

pub struct IntExpr {
    pub value: i64,
}

pub trait Expression {
    fn accept(&self, visitor: &dyn ExpressionVisitor);
    fn dump(&self) -> String;
}

impl Expression for EmptyExpr {
    fn accept(&self, visitor: &dyn ExpressionVisitor) {
        visitor.accept_empty(self);
    }

    fn dump(&self) -> String {
        String::from("<empty>")
    }
}

impl Expression for ConstExpr {
    fn accept(&self, visitor: &dyn ExpressionVisitor) {
        visitor.accept_const(self);
    }

    fn dump(&self) -> String {
        String::from(format!("<Const> {} = {}", &self.variable, &self.value.dump()))
    }
}

impl Expression for FunctionExpr {
    fn accept(&self, visitor: &dyn ExpressionVisitor) {
        visitor.accept_func(self);
    }

    fn dump(&self) -> String {
        let mut parameters_output = String::from("");

        for body_expr in &self.body {
            parameters_output += &*String::from(format!("<Body-Expr> {}\n", body_expr.dump()));
        }

        String::from(format!("<Func> {}\n{}", self.name, parameters_output))
    }
}

impl Expression for StringExpr {
    fn accept(&self, visitor: &dyn ExpressionVisitor) {
        visitor.accept_string(self);
    }

    fn dump(&self) -> String {
        String::from(format!("<String> \"{}\"", self.value))
    }
}

impl Expression for IntExpr {
    fn accept(&self, visitor: &dyn ExpressionVisitor) {
        visitor.accept_int(self);
    }

    fn dump(&self) -> String {
        String::from(format!("<Int> \"{}\"", self.value))
    }
}

impl Expression for VariableExpr {
    fn accept(&self, visitor: &dyn ExpressionVisitor) {
        visitor.accept_variable(self);
    }

    fn dump(&self) -> String {
        String::from(format!("<Variable> \"{}\"", self.name))
    }
}

impl Expression for PrintExpr {
    fn accept(&self, visitor: &dyn ExpressionVisitor) {
        visitor.accept_print(self);
    }

    fn dump(&self) -> String {
        let mut parameters_output = String::from("");

        for value_expr in &self.values {
            parameters_output += &*String::from(format!("{}, ", value_expr.dump()));
        }

        String::from(format!("<Print> {}", parameters_output))
    }
}