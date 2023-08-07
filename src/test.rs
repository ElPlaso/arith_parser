#[cfg(test)]
mod display_tests {
    use crate::expression::{BinaryOperator, Expression, UnaryOperator};

    #[test]
    fn test_display_integer() {
        let expr = Expression::Integer(42);
        assert_eq!(format!("{}", expr), "42");
    }

    #[test]
    fn test_display_variable() {
        let expr = Expression::Variable("x".to_string());
        assert_eq!(format!("{}", expr), "x");
    }

    #[test]
    fn test_display_boolean() {
        let expr1 = Expression::Boolean(true);
        assert_eq!(format!("{}", expr1), "T");

        let expr2 = Expression::Boolean(false);
        assert_eq!(format!("{}", expr2), "F");
    }

    #[test]
    fn test_display_binary_op() {
        let expr = Expression::BinaryOp {
            op: BinaryOperator::Add,
            lhs: Box::new(Expression::Integer(3)),
            rhs: Box::new(Expression::Integer(4)),
        };
        assert_eq!(format!("{}", expr), "3 + 4");
    }

    #[test]
    fn test_display_unary_op() {
        let expr = Expression::UnaryOp {
            op: UnaryOperator::Not,
            child: Box::new(Expression::Boolean(true)),
        };
        assert_eq!(format!("{}", expr), "!T");
    }

    #[test]
    fn test_display_func() {
        let expr = Expression::Func {
            param: "x".to_string(),
            body: Box::new(Expression::BinaryOp {
                op: BinaryOperator::Multiply,
                lhs: Box::new(Expression::Variable("x".to_string())),
                rhs: Box::new(Expression::Integer(2)),
            }),
        };
        assert_eq!(format!("{}", expr), "func x => x * 2");
    }

    #[test]
    fn test_display_if() {
        let expr = Expression::If {
            condition: Box::new(Expression::Boolean(true)),
            then_expr: Box::new(Expression::Integer(42)),
            else_expr: Box::new(Expression::Integer(0)),
        };
        assert_eq!(format!("{}", expr), "if T then 42 else 0");
    }

    #[test]
    fn test_display_apply() {
        let expr = Expression::Apply {
            func_expr: Box::new(Expression::Variable("f".to_string())),
            arg_expr: Box::new(Expression::Integer(10)),
        };
        assert_eq!(format!("{}", expr), "f (10)");
    }
}

#[cfg(test)]
mod lexing_tests {
    use crate::expression::BinaryOperator;
    use crate::parser::{lex, LexItem};

    #[test]
    fn lex_integer() {
        let input = "123";
        let result = lex(input);
        assert_eq!(result, Ok(vec![LexItem::Integer(123)]));
    }

    #[test]
    fn lex_variable() {
        let input = "abc";
        let result = lex(input);
        assert_eq!(result, Ok(vec![LexItem::Variable("abc".to_string())]));
    }

    #[test]
    fn lex_boolean_true() {
        let input = "T";
        let result = lex(input);
        assert_eq!(result, Ok(vec![LexItem::Boolean(true)]));
    }

    #[test]
    fn lex_boolean_false() {
        let input = "F";
        let result = lex(input);
        assert_eq!(result, Ok(vec![LexItem::Boolean(false)]));
    }

    #[test]
    fn lex_binary_operator() {
        let input = "+";
        let result = lex(input);
        assert_eq!(result, Ok(vec![LexItem::BinaryOp(BinaryOperator::Add)]));
    }

    #[test]
    fn lex_addition_expression() {
        let input = "+(1, 1)";
        let result = lex(input);
        assert_eq!(
            result,
            Ok(vec![
                LexItem::BinaryOp(BinaryOperator::Add),
                LexItem::OpenParen('('),
                LexItem::Integer(1),
                LexItem::Comma(','),
                LexItem::Integer(1),
                LexItem::CloseParen(')')
            ])
        );
    }

    #[test]
    fn lex_subtraction_expression() {
        let input = "-(1, 1)";
        let result = lex(input);
        assert_eq!(
            result,
            Ok(vec![
                LexItem::BinaryOp(BinaryOperator::Subtract),
                LexItem::OpenParen('('),
                LexItem::Integer(1),
                LexItem::Comma(','),
                LexItem::Integer(1),
                LexItem::CloseParen(')')
            ])
        );
    }
}

#[cfg(test)]
mod arith_tests {
    use crate::parser::Parser;

    #[test]
    fn parse_var() {
        let mut prog = Parser::new(&"x");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("x", format!("{}", e));
    }

    #[test]
    fn parse_int() {
        let mut prog = Parser::new(&"123");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("123", format!("{}", e));
    }

    #[test]
    fn parse_bool() {
        let mut prog = Parser::new(&"T");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("T", format!("{}", e));
    }

    #[test]
    fn parse_plus() {
        let mut prog = Parser::new(&"+(1, 1)");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("1 + 1", format!("{}", e));
    }

    #[test]
    fn parse_nested_plus() {
        let mut prog = Parser::new(&"+(1, +(1, 1))");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("1 + 1 + 1", format!("{}", e));
    }

    #[test]
    fn parse_minus() {
        let mut prog = Parser::new(&"-(1, 1)");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("1 - 1", format!("{}", e));
    }

    #[test]
    fn parse_mult() {
        let mut prog = Parser::new(&"*(1, 1)");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("1 * 1", format!("{}", e));
    }

    #[test]
    fn parse_div() {
        let mut prog = Parser::new(&"/(1, 1)");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("1 / 1", format!("{}", e));
    }

    #[test]
    fn parse_lt() {
        let mut prog = Parser::new(&"<(1, 1)");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("1 < 1", format!("{}", e));
    }

    #[test]
    fn parse_not() {
        let mut prog = Parser::new(&"!T");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("!T", format!("{}", e));
    }

    #[test]
    fn parse_eq() {
        let mut prog = Parser::new(&"=(1, 1)");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("1 = 1", format!("{}", e));
    }

    #[test]
    fn parse_func() {
        let mut prog = Parser::new(&"func x => T");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("func x => T", format!("{}", e));
    }

    #[test]
    fn parse_app() {
        let mut prog = Parser::new(&"apply(func x => x, 1)");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("func x => x (1)", format!("{}", e));
    }

    #[test]
    fn parse_if() {
        let mut prog = Parser::new(&"if <(1, 5) then 8 else 9");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("if 1 < 5 then 8 else 9", format!("{}", e));
    }
}
