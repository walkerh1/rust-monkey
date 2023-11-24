use std::iter::Peekable;

use self::ast::{Expression, ParsingError, Result, Statement};
use crate::lexer::{token::Token, Lexer, TokensIter};

mod ast;
mod tests;

pub struct NodesIter<'a> {
    iter: Peekable<TokensIter<'a>>,
}

impl<'a> NodesIter<'a> {
    fn parse_let(&mut self) -> Option<Result<Statement>> {
        // next token after 'let' should be an identifier
        let id = match self.get_identifier() {
            Ok(val) => val,
            Err(e) => {
                return Some(Err(e));
            }
        };

        // next token after identifier should be '='
        if let Err(e) = self.next_token_expecting(&Token::Assign) {
            return Some(Err(e));
        }

        // assume for now that there will always be a number after '='
        let int_token = self
            .iter
            .next()
            .expect("for now expect int, later this will be an expression");
        let val = match int_token {
            Token::Int(int) => int,
            _ => -1,
        };

        // for now just iterate till a semicolon is reached

        Some(Ok(Statement::Let(
            Expression::Identifier(id),
            Expression::Integer(val),
        )))
    }

    fn parse_return(&mut self) -> Option<Result<Statement>> {
        todo!()
    }

    fn next_token_expecting(
        &mut self,
        expected: &Token,
    ) -> std::result::Result<Token, ParsingError> {
        match self.iter.peek() {
            Some(found) => {
                if *found == *expected {
                    // unwrap safe here since already peeked
                    Ok(self.iter.next().unwrap())
                } else {
                    Err(ParsingError::new(expected, found))
                }
            }
            None => Err(ParsingError::new(expected, &Token::Eof)),
        }
    }

    fn get_identifier(&mut self) -> std::result::Result<String, ParsingError> {
        let expected = Token::Identifier(String::from("IDENT"));
        match self.iter.next() {
            Some(token) => match token {
                Token::Identifier(val) => Ok(val),
                _ => return Err(ParsingError::new(&expected, &token)),
            },
            None => return Err(ParsingError::new(&expected, &Token::Eof)),
        }
    }

    fn skip_to_semicolon(&mut self) {
        while let Some(token) = self.iter.peek() {
            println!("{}", token);
            if *token != Token::Semicolon {
                self.iter.next();
            } else {
                break;
            }
        }
    }
}

impl<'a> Iterator for NodesIter<'a> {
    type Item = Result<Statement>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.iter.next()?;
        let result = match token {
            Token::Let => self.parse_let(),
            Token::Return => self.parse_return(),
            _ => None,
        };
        self.skip_to_semicolon();
        // returns semicolon error only if result is not already an error
        if let Err(e) = self.next_token_expecting(&Token::Semicolon) {
            if result.as_ref().is_some_and(|s| s.is_ok()) {
                return Some(Err(e));
            }
        }
        result
    }
}

pub trait Parser {
    fn ast_nodes(&self) -> NodesIter;
}

impl<L: ?Sized + Lexer> Parser for L {
    fn ast_nodes(&self) -> NodesIter {
        NodesIter {
            iter: self.tokens().peekable(),
        }
    }
}
