use std::iter::Peekable;

use self::ast::{Expression, ParsingError, Statement};
use crate::lexer::{token::Token, Lexer, LexerIter};

mod ast;
mod tests;

type PrefixParseFn = fn(&Token) -> Result<Expression, ParsingError>;
type InfixParseFn = fn(&mut ParserIter) -> Result<Expression, ParsingError>;

enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

pub struct ParserIter<'a> {
    iter: Peekable<LexerIter<'a>>,
}

impl<'a> ParserIter<'a> {
    fn parse_let(&mut self) -> Result<Statement, ParsingError> {
        // after 'let' next token should be an identifier
        let expected = Token::Identifier(String::from("IDENT"));
        let identifier = Expression::Identifier(match self.next_token_or_end() {
            Some(Token::Identifier(id)) => id,
            Some(received) => return Err(ParsingError::new(&expected, &received)),
            None => return Err(ParsingError::new(&expected, &Token::Eof)),
        });

        // after identifier next token should be '='
        match self.next_token_or_end() {
            Some(Token::Assign) => {}
            Some(token) => return Err(ParsingError::new(&Token::Assign, &token)),
            None => return Err(ParsingError::new(&Token::Assign, &Token::Eof)),
        }

        // after '=' next token should be the start of an expression, which
        // means it should not be ';' or EOF
        let token = match self.next_token_or_end() {
            Some(Token::Semicolon) => {
                return Err(ParsingError(format!(
                    "Expected expression, got ';' instead"
                )))
            }
            Some(t) => t,
            None => {
                return Err(ParsingError(format!(
                    "Expected expression, got 'EOF' instead"
                )))
            }
        };

        let expression = match self.parse_expression(&token) {
            Ok(exp) => exp,
            Err(e) => return Err(e),
        };

        // after expression next token should be ';'
        match self.iter.peek() {
            Some(Token::Semicolon) => {}
            Some(token) => return Err(ParsingError::new(&Token::Semicolon, token)),
            None => return Err(ParsingError::new(&Token::Semicolon, &Token::Eof)),
        }

        Ok(Statement::Let(identifier, expression))
    }

    fn parse_return(&mut self) -> Result<Statement, ParsingError> {
        // after 'let' next token should be beginning of expression, which
        // means it should not be ';' or EOF
        let token = match self.next_token_or_end() {
            Some(Token::Semicolon) => {
                return Err(ParsingError(format!(
                    "Expected expression, got ';' instead"
                )))
            }
            Some(t) => t,
            None => {
                return Err(ParsingError(format!(
                    "Expected expression, got 'EOF' instead"
                )))
            }
        };

        let expression = match self.parse_expression(&token) {
            Ok(exp) => exp,
            Err(e) => return Err(e),
        };

        // after expression next token should be ';'
        match self.iter.peek() {
            Some(Token::Semicolon) => {}
            Some(token) => return Err(ParsingError::new(&Token::Semicolon, token)),
            None => return Err(ParsingError::new(&Token::Semicolon, &Token::Eof)),
        };

        Ok(Statement::Return(expression))
    }

    fn parse_expression_statement(&mut self, token: &Token) -> Result<Statement, ParsingError> {
        let expression = match self.parse_expression(token) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };
        Ok(Statement::Expression(expression))
    }

    fn parse_expression(&mut self, token: &Token) -> Result<Expression, ParsingError> {
        let prefix = match ParserIter::get_prefix_parse_fn(token) {
            Some(func) => func,
            None => {
                return Err(ParsingError(format!(
                    "Cannot parse {token} as a prefix operator"
                )))
            }
        };
        prefix(token)
    }

    fn get_prefix_parse_fn(token: &Token) -> Option<PrefixParseFn> {
        match token {
            Token::Identifier(_) => Some(Self::parse_identifier),
            Token::Int(_) => Some(Self::parse_integer),
            _ => None,
        }
    }

    fn parse_identifier(token: &Token) -> Result<Expression, ParsingError> {
        // already know token is the right type
        match token {
            Token::Identifier(val) => Ok(Expression::Identifier(val.clone())),
            _ => Err(ParsingError(String::from(
                "Error while processing identifier expression",
            ))),
        }
    }

    fn parse_integer(token: &Token) -> Result<Expression, ParsingError> {
        match token {
            Token::Int(int) => int
                .parse::<i64>()
                .map(Expression::Integer)
                .map_err(|_| ParsingError(format!("Expected an INT, got {}", int))),
            _ => Err(ParsingError(String::from(
                "Error while processing identifier expression",
            ))),
        }
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
