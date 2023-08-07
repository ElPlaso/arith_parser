use crate::expression::{BinaryOperator, Expression, UnaryOperator};

#[derive(Debug, PartialEq, Clone)]
pub enum LexItem {
    OpenParen(char),          // "("
    CloseParen(char),         // ")"
    Comma(char),              // ","
    Integer(i64),             // "0", "1", "2", ...
    Variable(String),         // "a", "b", "c", ...
    Boolean(bool),            // "T" or "F"
    If(String),               // "if"
    Then(String),             // "then"
    Else(String),             // "else"
    Func(String),             // "func"
    Apply(String),            // "apply"
    BinaryOp(BinaryOperator), // "+", "-", "*", "/", "<", "="
    UnaryOp(UnaryOperator),   // "!"
    Arrow(String),            // "=>"
}

pub fn lex(input: &str) -> Result<Vec<LexItem>, String> {
    let mut result = Vec::new();

    let mut iterable = input.chars().peekable();
    while let Some(&c) = iterable.peek() {
        match c {
            '0'..='9' => {
                let mut value = String::new();
                while let Some(&c) = iterable.peek() {
                    match c {
                        '0'..='9' => {
                            value.push(c);
                            iterable.next();
                        }
                        _ => break,
                    }
                }
                result.push(LexItem::Integer(value.parse().unwrap()));
            }
            'a'..='z' => {
                let mut value = String::new();
                while let Some(&c) = iterable.peek() {
                    match c {
                        'a'..='z' => {
                            value.push(c);
                            iterable.next();
                        }
                        _ => break,
                    }
                }
                match value.as_str() {
                    "if" => result.push(LexItem::If(value)),
                    "then" => result.push(LexItem::Then(value)),
                    "else" => result.push(LexItem::Else(value)),
                    "func" => result.push(LexItem::Func(value)),
                    "apply" => result.push(LexItem::Apply(value)),
                    _ => result.push(LexItem::Variable(value)),
                }
            }
            'T' => {
                result.push(LexItem::Boolean(true));
                iterable.next();
            }
            'F' => {
                result.push(LexItem::Boolean(false));
                iterable.next();
            }
            '+' => {
                result.push(LexItem::BinaryOp(BinaryOperator::Add));
                iterable.next();
            }
            '-' => {
                result.push(LexItem::BinaryOp(BinaryOperator::Subtract));
                iterable.next();
            }
            '*' => {
                result.push(LexItem::BinaryOp(BinaryOperator::Multiply));
                iterable.next();
            }
            '/' => {
                result.push(LexItem::BinaryOp(BinaryOperator::Divide));
                iterable.next();
            }
            '<' => {
                result.push(LexItem::BinaryOp(BinaryOperator::LessThan));
                iterable.next();
            }
            '!' => {
                result.push(LexItem::UnaryOp(UnaryOperator::Not));
                iterable.next();
            }
            '=' => {
                // Check for "=>" and "="
                iterable.next();
                if let Some(&c) = iterable.peek() {
                    match c {
                        '>' => {
                            result.push(LexItem::Arrow("=>".to_string()));
                            iterable.next();
                        }
                        _ => {
                            result.push(LexItem::BinaryOp(BinaryOperator::Equals));
                        }
                    }
                }
            }
            '(' => {
                result.push(LexItem::OpenParen('('));
                iterable.next();
            }
            ',' => {
                result.push(LexItem::Comma(','));
                iterable.next();
            }
            ')' => {
                result.push(LexItem::CloseParen(')'));
                iterable.next();
            }
            ' ' | '\t' => {
                // Skip whitespace
                iterable.next();
            }
            _ => {
                return Err(format!("unexpected character {}", c));
            }
        }
    }
    Ok(result)
}

pub struct Parser {
    tokens: Vec<LexItem>,
    current: usize,
}

impl Parser {
    pub fn new(program: &str) -> Self {
        let tokens = lex(program).unwrap_or_else(|err| {
            eprintln!("Error during lexing: {}", err);
            Vec::new()
        });

        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expression, String> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        if let Some(token) = self.tokens.get(self.current) {
            match token {
                LexItem::Integer(value) => {
                    self.current += 1;
                    Ok(Expression::Integer(*value))
                }
                LexItem::Variable(name) => {
                    self.current += 1;
                    Ok(Expression::Variable(name.clone()))
                }
                LexItem::Boolean(value) => {
                    self.current += 1;
                    Ok(Expression::Boolean(*value))
                }
                LexItem::UnaryOp(op) => self.parse_unary_expression(op.clone()),
                LexItem::BinaryOp(op) => self.parse_binary_expression(op.clone()),
                _ => Err("Expected expression".to_string()),
            }
        } else {
            Err("Unexpected end of input".to_string())
        }
    }

    fn parse_unary_expression(&mut self, op: UnaryOperator) -> Result<Expression, String> {
        self.current += 1;
        let child = self.parse_expression()?;
        Ok(Expression::UnaryOp {
            op,
            child: Box::new(child),
        })
    }

    fn parse_binary_expression(&mut self, op: BinaryOperator) -> Result<Expression, String> {
        // Increment the current index to move past the binary operator token
        self.current += 1;

        // Expect an opening parenthesis '('
        if let Some(LexItem::OpenParen('(')) = self.tokens.get(self.current) {
            self.current += 1;

            // Parse the left-hand side (lhs) expression
            let lhs = self.parse_expression()?;

            // Expect a comma ',' after the lhs
            if let Some(LexItem::Comma(',')) = self.tokens.get(self.current) {
                self.current += 1;
            } else {
                return Err("Expected ',' after left operand of binary expression".to_string());
            }

            // Parse the right-hand side (rhs) expression
            let rhs = self.parse_expression()?;

            // Expect a closing parenthesis ')' after the rhs
            if let Some(LexItem::CloseParen(')')) = self.tokens.get(self.current) {
                self.current += 1;
            } else {
                return Err("Expected closing parenthesis ')'".to_string());
            }

            // Construct the BinaryOp expression
            let binary_expr = Expression::BinaryOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };

            Ok(binary_expr)
        } else {
            Err(
                "Expected opening parenthesis '('. Parentheses are required for binary operations."
                    .to_string(),
            )
        }
    }
}
