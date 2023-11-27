use std::iter::Peekable;

use self::ast::{Block, Boolean, Expression, Infix, ParsingError, Prefix, Statement};
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
            Token::Lparen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }
}

#[derive(Debug)]
pub struct ParserIter<'a> {
    iter: Peekable<LexerIter<'a>>,
}

impl<'a> ParserIter<'a> {
    fn parse_let(&mut self) -> Result<Statement, ParsingError> {
        // after 'let' next token should be an identifier
        let identifier = Expression::Identifier(match self.next_token_or_end()? {
            Token::Identifier(id) => id,
            token => return Err(ParsingError::UnexpectedToken(token)),
        });

        // after identifier next token should be '='
        match self.next_token_or_end()? {
            Token::Assign => {}
            token => return Err(ParsingError::UnexpectedToken(token)),
        };

        // after '=' next token should be the start of an expression, which
        // means it should not be ';' or EOF
        let token = self.next_token_or_end()?;

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
        let token = self.next_token_or_end()?;

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

    fn parse_block(&mut self) -> Result<Block, ParsingError> {
        let mut block = vec![];

        while let Some(next) = self.iter.peek() {
            if *next != Token::Rbrace {
                match self.next() {
                    Some(res) => block.push(res?),
                    None => return Err(ParsingError::UnexpectedEof),
                };
            } else {
                break;
            }
        }

        Ok(block)
    }

    // only advances iterator when next token is not ';' and not EOF
    fn next_token_or_end(&mut self) -> Result<Token, ParsingError> {
        match self.iter.peek() {
            Some(Token::Semicolon) => Err(ParsingError::UnexpectedSemicolon),
            Some(_) => Ok(self.iter.next().unwrap()), // unwrap safe since peeked value is Some
            None => Err(ParsingError::UnexpectedEof),
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
        let prefix_fn = ParserIter::get_prefix_parse_fn(token)?;

        let mut left_expression = prefix_fn(self, token)?;

        while let Some(right) = self.iter.peek() {
            if *right != Token::Semicolon {
                if precedence < Precedence::get_precedence(right) {
                    let infix_fn = match ParserIter::get_infix_parse_fn(right) {
                        Some(func) => func,
                        None => break,
                    };
                    let operator = self.next_token_or_end()?;
                    left_expression = infix_fn(self, left_expression, &operator)?;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(left_expression)
    }

    fn get_prefix_parse_fn(token: &Token) -> Result<PrefixParseFn, ParsingError> {
        match token {
            Token::Identifier(_) => Ok(ParserIter::parse_identifier),
            Token::Int(_) => Ok(ParserIter::parse_integer),
            Token::Bang | Token::Minus => Ok(ParserIter::parse_prefix_expression),
            Token::True | Token::False => Ok(ParserIter::parse_boolean),
            Token::Lparen => Ok(ParserIter::parse_grouped_expression),
            Token::If => Ok(ParserIter::parse_if_expression),
            _ => Err(ParsingError::InvalidPrefixOperator(token.clone())),
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

    fn parse_boolean(_: &mut ParserIter, token: &Token) -> Result<Expression, ParsingError> {
        match token {
            Token::True => Ok(Expression::Boolean(Boolean::True)),
            Token::False => Ok(Expression::Boolean(Boolean::False)),
            _ => Err(ParsingError::Generic(String::from(
                "should never get here... fix types",
            ))),
        }
    }

    fn parse_grouped_expression(
        parser: &mut ParserIter,
        _: &Token,
    ) -> Result<Expression, ParsingError> {
        let next_token = parser.next_token_or_end()?;
        let exp = parser.parse_expression(&next_token, Precedence::Lowest)?;
        if let Some(token) = parser.iter.peek() {
            if *token != Token::Rparen {
                return Err(ParsingError::UnexpectedToken(token.clone()));
            } else {
                parser.next_token_or_end()?;
            }
        }
        Ok(exp)
    }

    fn parse_if_expression(parser: &mut ParserIter, _: &Token) -> Result<Expression, ParsingError> {
        // expect a left paren after an if token
        match parser.next_token_or_end()? {
            Token::Lparen => {}
            t => return Err(ParsingError::UnexpectedToken(t)),
        }

        let condition = parser.parse_expression(&Token::Lparen, Precedence::Lowest)?;

        // expect left brace after condition parsed
        match parser.next_token_or_end()? {
            Token::Lbrace => {}
            t => return Err(ParsingError::UnexpectedToken(t)),
        };

        let consequence = parser.parse_block()?;

        // expect right brace after block parsed
        match parser.next_token_or_end()? {
            Token::Rbrace => {}
            t => return Err(ParsingError::UnexpectedToken(t)),
        };

        let alternative = match parser.iter.peek() {
            Some(Token::Else) => {
                parser.next_token_or_end()?;

                // expect left brace after else keyword
                match parser.next_token_or_end()? {
                    Token::Lbrace => {}
                    t => return Err(ParsingError::UnexpectedToken(t)),
                };

                let else_branch = parser.parse_block()?;

                // expect right brace after block parsed
                match parser.next_token_or_end()? {
                    Token::Rbrace => {}
                    t => return Err(ParsingError::UnexpectedToken(t)),
                };

                Some(else_branch)
            }
            _ => None,
        };

        Ok(Expression::If(
            Box::new(condition),
            consequence,
            alternative,
        ))
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

        let next_token = parser.next_token_or_end()?;

        let right_expression = parser.parse_expression(&next_token, Precedence::Prefix)?;

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
            _ => {
                return Err(ParsingError::Generic(String::from(
                    "should never get here... fix types",
                )))
            }
        };

        let precedence = Precedence::get_precedence(operator);

        let next_token = parser.next_token_or_end()?;

        let right_expression = parser.parse_expression(&next_token, precedence)?;

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
        match token {
            Token::Let => {
                let r = Some(self.parse_let());
                self.skip_to_semicolon();
                r
            }
            Token::Return => {
                let r = Some(self.parse_return());
                self.skip_to_semicolon();
                r
            }
            _ => Some(self.parse_expression_statement(&token)),
        }
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
