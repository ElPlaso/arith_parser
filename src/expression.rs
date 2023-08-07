use std::fmt::{Display, Error};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Integer(i64),
    Variable(String),
    Boolean(bool),
    BinaryOp {
        op: BinaryOperator,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    UnaryOp {
        op: UnaryOperator,
        child: Box<Expression>,
    },
    Func {
        param: String,
        body: Box<Expression>,
    },
    If {
        condition: Box<Expression>,
        then_expr: Box<Expression>,
        else_expr: Box<Expression>,
    },
    Apply {
        func_expr: Box<Expression>,
        arg_expr: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    LessThan,
    Equals,
    And,
    Or,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Not,
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), Error> {
        match self {
            Expression::Integer(value) => write!(f, "{}", value),
            Expression::Variable(name) => write!(f, "{}", name),
            Expression::Boolean(value) => write!(f, "{}", if *value { "T" } else { "F" }),
            Expression::BinaryOp { op, lhs, rhs } => write!(f, "{} {} {}", lhs, op, rhs),
            Expression::UnaryOp { op, child } => write!(f, "{}{}", op, child),
            Expression::Func { param, body } => write!(f, "func {} => {}", param, body),
            Expression::If {
                condition,
                then_expr,
                else_expr,
            } => write!(f, "if {} then {} else {}", condition, then_expr, else_expr),
            Expression::Apply {
                func_expr,
                arg_expr,
            } => write!(f, "{} ({})", func_expr, arg_expr),
        }
    }
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), Error> {
        match self {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Subtract => write!(f, "-"),
            BinaryOperator::Multiply => write!(f, "*"),
            BinaryOperator::Divide => write!(f, "/"),
            BinaryOperator::LessThan => write!(f, "<"),
            BinaryOperator::Equals => write!(f, "="),
            BinaryOperator::And => write!(f, "&"),
            BinaryOperator::Or => write!(f, "|"),
        }
    }
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), Error> {
        match self {
            UnaryOperator::Not => write!(f, "!"),
        }
    }
}

impl Expression {
    pub fn eval(&self) -> Result<Expression, String> {
        unimplemented!()
    }
}
