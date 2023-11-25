use std::fmt::{Display, Formatter};

use crate::lexer::token::Token;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let(Expression, Expression),
    Return(Expression),
    Expression(Expression),
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Statement::Let(exp1,exp2)=>format!("let {} = {};", exp1, exp2),
                Statement::Return(exp)=>format!("return {};", exp),
                Statement::Expression(exp) => format!("{}", exp),
            }
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Identifier(String),
    Integer(i64),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Expression::Identifier(id) => format!("{}", id),
                Expression::Integer(int) => format!("{}", int),
            }
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct ParsingError(pub String);

impl ParsingError {
    pub fn new(expected: &Token, received: &Token) -> ParsingError {
        ParsingError(String::from(format!(
            "Expected next token to be '{}', got '{}' instead",
            expected, received
        )))
    }
}
