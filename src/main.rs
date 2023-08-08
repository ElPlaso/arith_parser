use crate::parser::Parser;

pub mod expression;
pub mod parser;
pub mod test;

fn main() {
    loop {
        println!("Enter an expression to evaluate:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let mut prog = Parser::new(input.trim());
        match prog.parse() {
            Ok(parsed) => {
                match parsed.eval() {
                    Ok(result) => {
                        println!("-----");
                        println!("Problem: {}", format!("{}", &parsed));
                        println!("Answer: {}", format!("{}", &result));
                        println!("-----");
                    }
                    Err(error) => {
                        eprintln!("Error evaluating expression: {}", error);
                    }
                }
            }
            Err(error) => {
                eprintln!("Error parsing expression: {}", error);
            }
        }
    }
}
