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
        unimplemented!()
    }
}

impl Expression {
    pub fn eval(&self) -> Result<Expression, String> {
        unimplemented!()
    }
}
