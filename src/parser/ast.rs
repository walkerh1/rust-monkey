use std::fmt::{Display, Formatter};

use crate::lexer::token::Token;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let(Expression, Expression),
    Return(Expression),
    Expression(Expression),
}

pub type Block = Vec<Statement>;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Identifier(String),
    Integer(i64),
    Prefix(Prefix, Box<Expression>),
    Infix(Box<Expression>, Infix, Box<Expression>),
    Boolean(Boolean),
    If(Box<Expression>, Block, Option<Block>),
    Function(Vec<Expression>, Block),
    Call(Box<Expression>, Vec<Expression>),
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

#[derive(Debug, PartialEq)]
pub enum Boolean {
    True,
    False,
}

impl Display for Boolean {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Boolean::True => "true",
                Boolean::False => "false",
            }
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum ParsingError {
    UnexpectedToken(Token),
    UnexpectedEof,
    UnexpectedSemicolon,
    InvalidPrefixOperator(Token),
    InvalidInteger(String),
    Generic(String),
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ParsingError::UnexpectedToken(token) => format!("Unexpected token: '{token}'"),
                ParsingError::UnexpectedEof => "Unexpected EOF".to_string(),
                ParsingError::UnexpectedSemicolon => "Unexpected end of statement: ';'".to_string(),
                ParsingError::InvalidPrefixOperator(token) =>
                    format!("'{token}' is not a valid prefix operator"),
                ParsingError::InvalidInteger(string) =>
                    format!("Cannot parse '{}' as a valid integer", *string),
                ParsingError::Generic(string) => string.to_string(),
            }
        )
    }
}
