use std::fmt::{format, Display, Formatter};

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
    Prefix(Prefix, Box<Expression>),
    Infix(Box<Expression>, Infix, Box<Expression>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Expression::Identifier(id) => id.to_string(),
                Expression::Integer(val) => val.to_string(),
                Expression::Prefix(prefix, exp) => format!("{prefix}{exp}"),
                Expression::Infix(left, infix, right) => format!("{left} {infix} {right}"),
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

#[derive(Debug, PartialEq)]
pub enum Prefix {
    Minus,
    Bang,
}

impl Display for Prefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Prefix::Minus => "-",
                Prefix::Bang => "!",
            }
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum Infix {
    Plus,
    Minus,
    Multiply,
    Divide,
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
}

impl Display for Infix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Infix::Plus => "+",
                Infix::Minus => "-",
                Infix::Multiply => "*",
                Infix::Divide => "/",
                Infix::GreaterThan => ">",
                Infix::LessThan => "<",
                Infix::Equal => "==",
                Infix::NotEqual => "!=",
            }
        )
    }
}
