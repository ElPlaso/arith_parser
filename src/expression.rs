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
        match self {
            Expression::Integer(_) => {
                // Integers just evaluate to themselves
                Ok(self.clone())
            }
            Expression::Variable(_) => {
                // Variables are not evaluated
                Ok(self.clone())
            }
            Expression::Boolean(_) => {
                // Booleans just evaluate to themselves
                Ok(self.clone())
            }
            Expression::UnaryOp { op, child } => {
                // Evaluate the child expression
                let eval_child = child.eval()?;

                // Apply the unary operator
                match op {
                    UnaryOperator::Not => match eval_child {
                        Expression::Boolean(b) => Ok(Expression::Boolean(!b)),
                        _ => Err("Invalid operand for 'Not' operator".to_string()),
                    },
                }
            }
            Expression::BinaryOp { op, lhs, rhs } => {
                // Evaluate the left and right child expressions
                let eval_lhs = lhs.eval()?;
                let eval_rhs = rhs.eval()?;

                // Apply the binary operator
                match op {
                    BinaryOperator::Add => {
                        if let (Expression::Integer(a), Expression::Integer(b)) =
                            (eval_lhs, eval_rhs)
                        {
                            Ok(Expression::Integer(a + b))
                        } else {
                            Err("Invalid operands for 'Add' operator".to_string())
                        }
                    }
                    BinaryOperator::Subtract => {
                        if let (Expression::Integer(a), Expression::Integer(b)) =
                            (eval_lhs, eval_rhs)
                        {
                            Ok(Expression::Integer(a - b))
                        } else {
                            Err("Invalid operands for 'Subtract' operator".to_string())
                        }
                    }
                    BinaryOperator::Multiply => {
                        if let (Expression::Integer(a), Expression::Integer(b)) =
                            (eval_lhs, eval_rhs)
                        {
                            Ok(Expression::Integer(a * b))
                        } else {
                            Err("Invalid operands for 'Multiply' operator".to_string())
                        }
                    }
                    BinaryOperator::Divide => {
                        if let (Expression::Integer(a), Expression::Integer(b)) =
                            (eval_lhs, eval_rhs)
                        {
                            Ok(Expression::Integer(a / b))
                        } else {
                            Err("Invalid operands for 'Divide' operator".to_string())
                        }
                    }
                    BinaryOperator::Equals => {
                        if let (Expression::Integer(a), Expression::Integer(b)) =
                            (eval_lhs, eval_rhs)
                        {
                            Ok(Expression::Boolean(a == b))
                        } else {
                            Err("Invalid operands for 'Equals' operator".to_string())
                        }
                    }
                    BinaryOperator::LessThan => {
                        if let (Expression::Integer(a), Expression::Integer(b)) =
                            (eval_lhs, eval_rhs)
                        {
                            Ok(Expression::Boolean(a < b))
                        } else {
                            Err("Invalid operands for 'LessThan' operator".to_string())
                        }
                    }
                    BinaryOperator::And => {
                        if let (Expression::Boolean(a), Expression::Boolean(b)) =
                            (eval_lhs, eval_rhs)
                        {
                            Ok(Expression::Boolean(a && b))
                        } else {
                            Err("Invalid operands for 'And' operator".to_string())
                        }
                    }
                    BinaryOperator::Or => {
                        if let (Expression::Boolean(a), Expression::Boolean(b)) =
                            (eval_lhs, eval_rhs)
                        {
                            Ok(Expression::Boolean(a || b))
                        } else {
                            Err("Invalid operands for 'Or' operator".to_string())
                        }
                    }
                }
            }
            Expression::Func { param, body } => todo!(),
            Expression::If { condition, then_expr, else_expr } => todo!(),
            Expression::Apply { func_expr, arg_expr } => todo!(),
        }
    }
}
