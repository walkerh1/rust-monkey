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
pub enum ParsingError {
    UnexpectedToken(Token),
    UnexpectedEof,
    UnexpectedSemicolon,
    InvalidPrefixOperator(Token),
    InvalidInfixOperator(Token),
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
                ParsingError::InvalidInfixOperator(token) =>
                    format!("'{token}' is not a valid infix operator"),
                ParsingError::InvalidInteger(string) =>
                    format!("Cannot parse '{}' as a valid integer", *string),
                ParsingError::Generic(string) => string.to_string(),
            }
        )
    }
}
