use std::fmt::Formatter;
use std::iter::Peekable;

use self::ast::{Expression, Infix, Prefix, Statement};
use crate::lexer::{token::Token, Lexer, LexerIter};
use crate::parser::precedence::Precedence;

pub mod ast;
mod precedence;
mod tests;

type PrefixParseFn = fn(&mut ParserIter, &Token) -> Result<Expression, ParsingError>;
type InfixParseFn = fn(&mut ParserIter, Expression, &Token) -> Result<Expression, ParsingError>;

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

    fn parse_block(&mut self) -> Result<Statement, ParsingError> {
        // expect first token of block to be '{'
        match self.next_token_or_end()? {
            Token::Lbrace => {}
            token => return Err(ParsingError::UnexpectedToken(token)),
        }

        let mut block = vec![];

        loop {
            if let Some(Token::Rbrace) = self.iter.peek() {
                break;
            }

            match self.next() {
                Some(result) => block.push(result?),
                None => break, // means a Rbrace was detected in self.next
            }
        }

        // expect last token of block to be '}'
        match self.next_token_or_end()? {
            Token::Rbrace => {}
            token => return Err(ParsingError::UnexpectedToken(token)),
        }

        Ok(Statement::BlockStatement(block))
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
            Token::Function => Ok(ParserIter::parse_function_literal),
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
            Token::Lparen => Some(ParserIter::parse_call_expression),
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
            Token::True => Ok(Expression::Boolean(true)),
            Token::False => Ok(Expression::Boolean(false)),
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
        // get and expect next token to be '(' after 'if'
        let token = match parser.next_token_or_end()? {
            Token::Lparen => Token::Lparen,
            t => return Err(ParsingError::UnexpectedToken(t)),
        };

        // expect grouped expression after 'if' token
        let condition = parser.parse_expression(&token, Precedence::Lowest)?;

        let consequence = Box::new(parser.parse_block()?);

        let alternative = match parser.iter.peek() {
            Some(Token::Else) => {
                parser.next_token_or_end()?;

                Some(Box::new(parser.parse_block()?))
            }
            _ => None,
        };

        Ok(Expression::If(
            Box::new(condition),
            consequence,
            alternative,
        ))
    }

    fn parse_function_literal(
        parser: &mut ParserIter,
        _: &Token,
    ) -> Result<Expression, ParsingError> {
        // expect parameter list after 'fn' keyword
        let parameters = parser.parse_function_parameters()?;

        // expect block statement after parameter list
        let body = Box::new(parser.parse_block()?);

        Ok(Expression::Function(parameters, body))
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<Expression>, ParsingError> {
        // expect first token of parameter list to be '('
        match self.next_token_or_end()? {
            Token::Lparen => {}
            token => return Err(ParsingError::UnexpectedToken(token)),
        }

        let mut parameters = vec![];

        loop {
            match self.next_token_or_end()? {
                Token::Identifier(id) => parameters.push(Expression::Identifier(id)),
                t => return Err(ParsingError::UnexpectedToken(t)),
            }

            match self.iter.peek() {
                Some(Token::Comma) => {
                    self.next_token_or_end()?;
                }
                Some(Token::Rparen) => {
                    self.next_token_or_end()?;
                    break;
                }
                Some(t) => return Err(ParsingError::UnexpectedToken(t.clone())),
                None => return Err(ParsingError::UnexpectedEof),
            }
        }

        Ok(parameters)
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

    fn parse_call_expression(
        parser: &mut ParserIter,
        left_expression: Expression,
        _: &Token,
    ) -> Result<Expression, ParsingError> {
        let mut arguments = vec![];

        if let Some(Token::Rparen) = parser.iter.peek() {
            parser.next_token_or_end()?;
            return Ok(Expression::Call(Box::new(left_expression), arguments));
        }

        let next_token = parser.next_token_or_end()?;
        arguments.push(parser.parse_expression(&next_token, Precedence::Lowest)?);

        loop {
            if let Some(Token::Comma) = parser.iter.peek() {
                parser.next_token_or_end()?;
                let next_token = parser.next_token_or_end()?;
                arguments.push(parser.parse_expression(&next_token, Precedence::Lowest)?);
            } else {
                break;
            }
        }

        match parser.next_token_or_end()? {
            Token::Rparen => {}
            token => return Err(ParsingError::UnexpectedToken(token)),
        }

        Ok(Expression::Call(Box::new(left_expression), arguments))
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
        let token = self.iter.peek()?;
        match token {
            Token::Let => {
                self.iter.next()?;
                let r = Some(self.parse_let());
                self.skip_to_semicolon();
                r
            }
            Token::Return => {
                self.iter.next()?;
                let r = Some(self.parse_return());
                self.skip_to_semicolon();
                r
            }
            Token::Rbrace => None,
            _ => {
                let t = token.clone();
                self.iter.next()?;
                match self.parse_expression_statement(&t) {
                    Ok(s) => Some(Ok(s)),
                    Err(e) => {
                        self.skip_to_semicolon();
                        Some(Err(e))
                    }
                }
            }
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

#[derive(Debug, PartialEq)]
pub enum ParsingError {
    UnexpectedToken(Token),
    UnexpectedEof,
    UnexpectedSemicolon,
    InvalidPrefixOperator(Token),
    InvalidInteger(String),
    Generic(String),
}

impl std::fmt::Display for ParsingError {
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
