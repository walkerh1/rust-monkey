use std::iter::Peekable;

use self::ast::{Expression, Infix, ParsingError, Prefix, Statement};
use crate::lexer::{token::Token, Lexer, LexerIter};

mod ast;
mod tests;

type PrefixParseFn = fn(&mut ParserIter, &Token) -> Result<Expression, ParsingError>;
type InfixParseFn = fn(&mut ParserIter, Expression, &Token) -> Result<Expression, ParsingError>;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    Lowest = 0,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

impl Precedence {
    fn get_precedence(token: &Token) -> Precedence {
        match token {
            Token::Eq | Token::Noteq => Precedence::Equals,
            Token::Lt | Token::Gt => Precedence::LessGreater,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Asterisk | Token::Slash => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }
}

pub struct ParserIter<'a> {
    iter: Peekable<LexerIter<'a>>,
}

impl<'a> ParserIter<'a> {
    fn parse_let(&mut self) -> Result<Statement, ParsingError> {
        // after 'let' next token should be an identifier
        let identifier = Expression::Identifier(match self.next_token_or_end() {
            Some(Token::Identifier(id)) => id,
            Some(Token::Semicolon) => return Err(ParsingError::UnexpectedSemicolon),
            Some(token) => return Err(ParsingError::UnexpectedToken(token)),
            None => return Err(ParsingError::UnexpectedEof),
        });

        // after identifier next token should be '='
        match self.next_token_or_end() {
            Some(Token::Assign) => {}
            Some(token) => return Err(ParsingError::UnexpectedToken(token)),
            None => return Err(ParsingError::UnexpectedEof),
        };

        // after '=' next token should be the start of an expression, which
        // means it should not be ';' or EOF
        let token = match self.next_token_or_end() {
            Some(Token::Semicolon) => return Err(ParsingError::UnexpectedSemicolon),
            Some(t) => t,
            None => return Err(ParsingError::UnexpectedEof),
        };

        let expression = match self.parse_expression(&token, Precedence::Lowest) {
            Ok(exp) => exp,
            Err(e) => return Err(e),
        };

        // after expression next token should be ';'
        match self.iter.peek() {
            Some(Token::Semicolon) => {}
            Some(token) => return Err(ParsingError::UnexpectedToken(token.clone())),
            None => return Err(ParsingError::UnexpectedEof),
        }

        Ok(Statement::Let(identifier, expression))
    }

    fn parse_return(&mut self) -> Result<Statement, ParsingError> {
        // after 'let' next token should be beginning of expression, which
        // means it should not be ';' or EOF
        let token = match self.next_token_or_end() {
            Some(Token::Semicolon) => return Err(ParsingError::UnexpectedSemicolon),
            Some(t) => t,
            None => return Err(ParsingError::UnexpectedEof),
        };

        let expression = match self.parse_expression(&token, Precedence::Lowest) {
            Ok(exp) => exp,
            Err(e) => return Err(e),
        };

        // after expression next token should be ';'
        match self.iter.peek() {
            Some(Token::Semicolon) => {}
            Some(token) => return Err(ParsingError::UnexpectedToken(token.clone())),
            None => return Err(ParsingError::UnexpectedEof),
        };

        Ok(Statement::Return(expression))
    }

    fn parse_expression_statement(&mut self, token: &Token) -> Result<Statement, ParsingError> {
        let expression = match self.parse_expression(token, Precedence::Lowest) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        Ok(Statement::Expression(expression))
    }

    // only advances iterator when next token is not ';' and not EOF
    fn next_token_or_end(&mut self) -> Option<Token> {
        match self.iter.peek() {
            Some(Token::Semicolon) => Some(Token::Semicolon),
            Some(_) => Some(self.iter.next().unwrap()),
            None => None,
        }
    }

    fn skip_to_semicolon(&mut self) {
        while let Some(token) = self.iter.peek() {
            if *token != Token::Semicolon {
                self.iter.next();
            } else {
                break;
            }
        }
    }

    fn parse_expression(
        &mut self,
        token: &Token,
        precedence: Precedence,
    ) -> Result<Expression, ParsingError> {
        let prefix_fn = match ParserIter::get_prefix_parse_fn(token) {
            Some(func) => func,
            None => return Err(ParsingError::InvalidPrefixOperator(token.clone())),
        };

        let mut left_expression = prefix_fn(self, token)?;

        loop {
            let infix_fn = if let Some(right) = self.iter.peek() {
                if *right == Token::Semicolon {
                    break;
                }
                if precedence < Precedence::get_precedence(right) {
                    match ParserIter::get_infix_parse_fn(right) {
                        Some(func) => func,
                        None => return Err(ParsingError::InvalidInfixOperator(token.clone())),
                    }
                } else {
                    break;
                }
            } else {
                break;
            };

            let operator = match self.next_token_or_end() {
                Some(Token::Semicolon) => return Err(ParsingError::UnexpectedSemicolon),
                Some(t) => t,
                None => return Err(ParsingError::UnexpectedEof),
            };

            left_expression = infix_fn(self, left_expression, &operator)?;
        }

        Ok(left_expression)
    }

    fn get_prefix_parse_fn(token: &Token) -> Option<PrefixParseFn> {
        match token {
            Token::Identifier(_) => Some(ParserIter::parse_identifier),
            Token::Int(_) => Some(ParserIter::parse_integer),
            Token::Bang | Token::Minus => Some(ParserIter::parse_prefix_expression),
            _ => None,
        }
    }

    fn get_infix_parse_fn(token: &Token) -> Option<InfixParseFn> {
        match token {
            Token::Plus
            | Token::Minus
            | Token::Asterisk
            | Token::Slash
            | Token::Lt
            | Token::Gt
            | Token::Eq
            | Token::Noteq => Some(ParserIter::parse_infix_expression),
            _ => None,
        }
    }

    fn parse_identifier(_: &mut ParserIter, token: &Token) -> Result<Expression, ParsingError> {
        match token {
            Token::Identifier(val) => Ok(Expression::Identifier(val.clone())),
            _ => Err(ParsingError::Generic(String::from(
                "should never get here... fix types",
            ))),
        }
    }

    fn parse_integer(_: &mut ParserIter, token: &Token) -> Result<Expression, ParsingError> {
        match token {
            Token::Int(int) => int
                .parse::<i64>()
                .map(Expression::Integer)
                .map_err(|_| ParsingError::InvalidInteger(int.clone())),
            _ => Err(ParsingError::Generic(String::from(
                "should never get here... fix types",
            ))),
        }
    }

    fn parse_prefix_expression(
        parser: &mut ParserIter,
        token: &Token,
    ) -> Result<Expression, ParsingError> {
        let prefix = match token {
            Token::Bang => Prefix::Bang,
            Token::Minus => Prefix::Minus,
            _ => {
                return Err(ParsingError::Generic(String::from(
                    "should never get here... fix types",
                )))
            }
        };

        let right_expression = match parser.next_token_or_end() {
            Some(Token::Semicolon) => return Err(ParsingError::UnexpectedSemicolon),
            Some(t) => parser.parse_expression(&t, Precedence::Prefix)?,
            None => return Err(ParsingError::UnexpectedEof),
        };

        Ok(Expression::Prefix(prefix, Box::new(right_expression)))
    }

    fn parse_infix_expression(
        parser: &mut ParserIter,
        left_expression: Expression,
        operator: &Token,
    ) -> Result<Expression, ParsingError> {
        let infix = match operator {
            Token::Plus => Infix::Plus,
            Token::Minus => Infix::Minus,
            Token::Asterisk => Infix::Multiply,
            Token::Slash => Infix::Divide,
            Token::Lt => Infix::LessThan,
            Token::Gt => Infix::GreaterThan,
            Token::Eq => Infix::Equal,
            Token::Noteq => Infix::NotEqual,
            _ => todo!(),
        };

        let precedence = Precedence::get_precedence(operator);

        let right_expression = match parser.next_token_or_end() {
            Some(token) => parser.parse_expression(&token, precedence)?,
            None => todo!(),
        };

        Ok(Expression::Infix(
            Box::new(left_expression),
            infix,
            Box::new(right_expression),
        ))
    }
}

impl<'a> Iterator for ParserIter<'a> {
    type Item = Result<Statement, ParsingError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.iter.peek() {
            if *token == Token::Semicolon {
                self.iter.next();
            }
        }
        let token = self.iter.next()?;
        let result = match token {
            Token::Let => Some(self.parse_let()),
            Token::Return => Some(self.parse_return()),
            _ => Some(self.parse_expression_statement(&token)),
        };
        self.skip_to_semicolon();
        result
    }
}

pub trait Parser {
    fn ast_nodes(&self) -> ParserIter;
}

impl<L: ?Sized + Lexer> Parser for L {
    fn ast_nodes(&self) -> ParserIter {
        ParserIter {
            iter: self.tokens().peekable(),
        }
    }
}
