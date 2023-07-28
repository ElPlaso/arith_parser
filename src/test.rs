#[cfg(test)]
mod arith_tests {
    use crate::expression::Expression;
    use crate::parser::Parser;

    #[test]
    fn parse_var(){
        let mut prog = Parser::new(&"x");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("x", format!("{}", e));
    }

    #[test]
    fn parse_int(){
        let mut prog = Parser::new(&"123");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("123", format!("{}", e));
    }

    #[test]
    fn parse_bool(){
        let mut prog = Parser::new(&"T");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("T", format!("{}", e));
    }

    #[test]
    fn parse_plus(){
        let mut prog = Parser::new(&"+(1, 1)");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("1 + 1", format!("{}", e));
    }

    #[test]
    fn parse_minus(){
        let mut prog = Parser::new(&"-(1, 1)");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("1 - 1", format!("{}", e));
    }

    #[test]
    fn parse_mult(){
        let mut prog = Parser::new(&"*(1, 1)");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("1 * 1", format!("{}", e));
    }

    #[test]
    fn parse_div(){
        let mut prog = Parser::new(&"/(1, 1)");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("1 / 1", format!("{}", e));
    }

    #[test]
    fn parse_lt(){
        let mut prog = Parser::new(&"<(1, 1)");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("1 < 1", format!("{}", e));
    }

    #[test]
    fn parse_not(){
        let mut prog = Parser::new(&"!T");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("!T", format!("{}", e));
    }

    #[test]
    fn parse_eq(){
        let mut prog = Parser::new(&"=(1, 1)");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("1 = 1", format!("{}", e));
    }

    #[test]
    fn parse_func(){
        let mut prog = Parser::new(&"func x => T");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("func x => T", format!("{}", e));
    }

    #[test]
    fn parse_app(){
        let mut prog = Parser::new(&"apply(func x => x, 1)");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("func x => x (1)", format!("{}", e));
    }

    #[test]
    fn parse_if(){
        let mut prog = Parser::new(&"if <(1, 5) then 8 else 9");
        let result = prog.parse();
        assert!(result.is_ok());
        let e = result.unwrap();
        assert_eq!("if 1 < 5 then 8 else 9", format!("{}", e));
    }
}
