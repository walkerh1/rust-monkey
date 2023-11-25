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
                Statement::Let(exp1, exp2) => format!("let {} = {};", exp1, exp2),
                Statement::Return(exp) => format!("return {};", exp),
                Statement::Expression(exp) => exp.to_string(),
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
                Expression::Identifier(id) => id.to_string(),
                Expression::Integer(val) => val.to_string(),
            }
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct ParsingError(pub String);

impl ParsingError {
    pub fn new(expected: &Token, received: &Token) -> ParsingError {
        ParsingError(format!(
            "Expected next token to be '{}', got '{}' instead",
            expected, received
        ))
    }
}
