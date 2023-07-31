use std::fmt::{Display, Error};

#[derive(PartialEq)]
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
}

#[derive(PartialEq)]
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

#[derive(PartialEq)]
pub enum UnaryOperator {
    Not,
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), Error> {
        match self {
            Expression::Integer(value) => write!(f, "{}", value),
            Expression::Variable(name) => write!(f, "{}", name),
            Expression::Boolean(value) => write!(f, "{}", if *value { "T" } else { "F" }),
            Expression::BinaryOp { op, lhs, rhs } => write!(f, "({} {} {})", lhs, op, rhs),
            Expression::UnaryOp { op, child } => write!(f, "({}{})", op, child),
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
            BinaryOperator::And => write!(f, "&&"),
            BinaryOperator::Or => write!(f, "||"),
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
