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
        // next token after 'let' should be an identifier
        let expected = Token::Identifier(String::from("IDENT"));
        let id = match self.iter.next() {
            Some(Token::Identifier(val)) => val,
            Some(token) => {
                return Err(ParsingError::new(&expected, &token));
            }
            None => {
                return Err(ParsingError::new(&expected, &Token::Eof));
            }
        };

        // next token after identifier should be '='
        self.next_token_expecting(&Token::Assign)?;

        // assume for now that there will always be a number after '='
        let int_token = self
            .iter
            .next()
            .expect("for now expect int, later this will be an expression");
        let val = match int_token {
            Token::Int(int) => int,
            _ => -1,
        };

        // next token after single expression on RHS of '=' should be ';'
        match self.iter.peek() {
            Some(Token::Semicolon) => {}
            Some(token) => return Err(ParsingError::new(&Token::Semicolon, token)),
            None => return Err(ParsingError::new(&Token::Semicolon, &Token::Eof)),
        }

        Ok(Statement::Let(
            Expression::Identifier(id),
            Expression::Integer(val),
        ))
    }

    fn parse_return(&mut self) -> Result<Statement, ParsingError> {
        let exp = self.iter.next().expect("for now expect int");
        // this will change when we start to parse expressions
        let val = match exp {
            Token::Int(int) => int,
            _ => -1,
        };
        Ok(Statement::Return(Expression::Integer(val)))
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
            None => todo!(),
        };
        prefix(token)
    }

    fn get_prefix_parse_fn(token: &Token) -> Option<PrefixParseFn> {
        match token {
            Token::Identifier(_) => Some(Self::parse_identifier),
            _ => None,
        }
    }

    fn parse_identifier(token: &Token) -> Result<Expression, ParsingError> {
        // already know token is the right type
        match token {
            Token::Identifier(val) => Ok(Expression::Identifier(val.clone())),
            _ => Err(ParsingError(String::from("Impossible to reach this line"))),
        }
    }

    fn next_token_expecting(&mut self, expected: &Token) -> Result<Token, ParsingError> {
        match self.iter.peek() {
            Some(found) => {
                if found == expected {
                    // unwrap safe here since already peeked
                    Ok(self.iter.next().unwrap())
                } else {
                    Err(ParsingError::new(expected, found))
                }
            }
            None => Err(ParsingError::new(expected, &Token::Eof)),
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
                self.iter.next()?;
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
