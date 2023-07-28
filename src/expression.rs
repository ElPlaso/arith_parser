use std::fmt::{Display, Error};

#[derive(PartialEq)]
pub enum Expression{}

impl Display for Expression{
    fn fmt(&self, f : &mut std::fmt::Formatter<'_>) -> Result<(), Error>{
        unimplemented!()
    }
}

impl Expression {

    pub fn eval(&self) -> Result<Expression, String>{
        unimplemented!()
    }

}
