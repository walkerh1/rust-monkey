use std::fmt::Formatter;
use std::iter::Peekable;

use self::ast::{Expression, Infix, Prefix, Statement};
use crate::lexer::{token::Token, Lexer, LexerIter};
use crate::parser::ast::Program;
use crate::parser::precedence::Precedence;

pub mod ast;
mod precedence;
mod tests;

pub struct Parser<'a> {
    iter: Peekable<LexerIter<'a>>,
}

impl<'a> Parser<'a> {
    pub fn parse_program(program: &str) -> Result<Program, Vec<ParsingError>> {
        let mut parser = Parser {
            iter: program.tokens().peekable(),
        };

        let mut program = vec![];
        let mut errors = vec![];

        loop {
            let token = match parser.iter.peek() {
                Some(Token::Semicolon) => {
                    parser.iter.next();
                    continue;
                }
                Some(tok) => tok.clone(),
                None => break,
            };

            match parser.parse_statement(&token) {
                Ok(statement) => program.push(statement),
                Err(error) => errors.push(error),
            }
        }

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(Program(program))
        }
    }

    fn parse_statement(&mut self, token: &Token) -> Result<Statement, ParsingError> {
        self.iter.next();
        match token {
            Token::Let => {
                let r = self.parse_let();
                self.skip_to_semicolon();
                r
            }
            Token::Return => {
                let r = self.parse_return();
                self.skip_to_semicolon();
                r
            }
            t => {
                if let Some(Token::Assign) = self.iter.peek() {
                    let r = self.parse_assignment(t);
                    self.skip_to_semicolon();
                    r
                } else {
                    match self.parse_expression_statement(t) {
                        Ok(s) => Ok(s),
                        Err(e) => {
                            self.skip_to_semicolon();
                            Err(e)
                        }
                    }
                }
            }
        }
    }

    fn parse_assignment(&mut self, token: &Token) -> Result<Statement, ParsingError> {
        // first token in assignment statement has to be an identifier
        let identifier = Expression::Identifier(match token {
            Token::Identifier(id) => id.clone(),
            token => return Err(ParsingError::UnexpectedToken(token.clone())),
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

        Ok(Statement::Assignment(identifier, expression))
    }

    fn parse_let(&mut self) -> Result<Statement, ParsingError> {
        // after 'let' next token should be an identifier
        let name: String;
        let identifier = Expression::Identifier(match self.next_token_or_end()? {
            Token::Identifier(id) => {
                name = id.clone();
                id
            }
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
            Ok(exp) => {
                if let Expression::Function(args, block, _) = exp {
                    Expression::Function(args, block, name)
                } else {
                    exp
                }
            }
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

    fn parse_block_statement(&mut self) -> Result<Statement, ParsingError> {
        // expect first token of block to be '{'
        match self.next_token_or_end()? {
            Token::Lbrace => {}
            token => return Err(ParsingError::UnexpectedToken(token)),
        }

        let mut block = vec![];

        loop {
            let token = match self.iter.peek() {
                Some(Token::Semicolon) => {
                    self.iter.next();
                    continue;
                }
                Some(tok) => tok.clone(),
                None => return Err(ParsingError::UnexpectedEof),
            };

            if token == Token::Rbrace {
                break;
            } else {
                let result = self.parse_statement(&token)?;
                block.push(result)
            }
        }

        // expect last token of block to be '}'
        match self.next_token_or_end()? {
            Token::Rbrace => {}
            token => return Err(ParsingError::UnexpectedToken(token)),
        }

        Ok(Statement::BlockStatement(block))
    }

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
        // prefix parse functions
        let mut left_expression = match token {
            Token::Identifier(id) => Self::parse_identifier(id),
            Token::Int(int) => Self::parse_integer(int),
            Token::Bang | Token::Minus => self.parse_prefix_expression(token),
            Token::True => Parser::parse_boolean(true),
            Token::False => Parser::parse_boolean(false),
            Token::Lparen => self.parse_grouped_expression(),
            Token::If => self.parse_if_expression(),
            Token::Function => self.parse_function_literal(),
            Token::String(string) => Self::parse_string(string),
            Token::Lbracket => self.parse_array_literal(),
            Token::Lbrace => self.parse_hash_literal(),
            Token::While => self.parse_while_expression(),
            _ => return Err(ParsingError::InvalidPrefixOperator(token.clone())),
        }?;

        loop {
            let right = match self.iter.peek() {
                Some(Token::Semicolon) | None => break,
                Some(tok) => tok.clone(),
            };

            if precedence < Precedence::get_precedence(&right) {
                let operator = self.next_token_or_end()?;
                // infix parse functions
                left_expression = match right {
                    Token::Plus
                    | Token::Minus
                    | Token::Asterisk
                    | Token::Slash
                    | Token::Lt
                    | Token::Gt
                    | Token::Eq
                    | Token::Noteq
                    | Token::And
                    | Token::Or => self.parse_infix_expression(left_expression, &operator)?,
                    Token::Lparen => self.parse_call_expression(left_expression)?,
                    Token::Lbracket => self.parse_index_expression(left_expression)?,
                    _ => break,
                }
            } else {
                break;
            }
        }

        Ok(left_expression)
    }

    fn parse_while_expression(&mut self) -> Result<Expression, ParsingError> {
        // get and expect next token to be '(' after 'if'
        let token = match self.next_token_or_end()? {
            Token::Lparen => Token::Lparen,
            t => return Err(ParsingError::UnexpectedToken(t)),
        };

        // expect grouped expression after 'if' token
        let condition = self.parse_expression(&token, Precedence::Lowest)?;

        let loop_block = Box::new(self.parse_block_statement()?);

        Ok(Expression::While(Box::new(condition), loop_block))
    }

    fn parse_hash_literal(&mut self) -> Result<Expression, ParsingError> {
        let mut hash = vec![];

        loop {
            let mut curr_token = self.next_token_or_end()?;
            if curr_token == Token::Rbrace {
                break;
            }

            let key = self.parse_expression(&curr_token, Precedence::Lowest)?;

            curr_token = self.next_token_or_end()?;
            if curr_token != Token::Colon {
                return Err(ParsingError::UnexpectedToken(curr_token));
            }

            curr_token = self.next_token_or_end()?;
            let value = self.parse_expression(&curr_token, Precedence::Lowest)?;

            hash.push((key, value));

            match self.iter.peek() {
                Some(Token::Comma) => {
                    self.next_token_or_end()?;
                }
                Some(Token::Rbrace) => continue,
                Some(token) => return Err(ParsingError::UnexpectedToken(token.clone())),
                None => return Err(ParsingError::UnexpectedEof),
            }
        }

        Ok(Expression::Hash(hash))
    }

    fn parse_index_expression(&mut self, left: Expression) -> Result<Expression, ParsingError> {
        if let Some(Token::Rbracket) = self.iter.peek() {
            return Err(ParsingError::UnexpectedToken(Token::Rbracket));
        }

        let next_token = self.next_token_or_end()?;
        let right = self.parse_expression(&next_token, Precedence::Lowest)?;

        match self.next_token_or_end()? {
            Token::Rbracket => {}
            token => return Err(ParsingError::UnexpectedToken(token)),
        }

        Ok(Expression::Index(Box::new(left), Box::new(right)))
    }

    fn parse_array_literal(&mut self) -> Result<Expression, ParsingError> {
        if let Some(Token::Rbracket) = self.iter.peek() {
            self.next_token_or_end()?;
            return Ok(Expression::Array(vec![]));
        }

        let array = self.parse_expression_list()?;

        match self.next_token_or_end()? {
            Token::Rbracket => {}
            token => return Err(ParsingError::UnexpectedToken(token)),
        }

        Ok(Expression::Array(array))
    }

    fn parse_expression_list(&mut self) -> Result<Vec<Expression>, ParsingError> {
        let mut array = vec![];

        let next_token = self.next_token_or_end()?;
        array.push(self.parse_expression(&next_token, Precedence::Lowest)?);

        while let Some(Token::Comma) = self.iter.peek() {
            self.next_token_or_end()?;
            let next_token = self.next_token_or_end()?;
            array.push(self.parse_expression(&next_token, Precedence::Lowest)?);
        }

        Ok(array)
    }

    fn parse_identifier(id: &str) -> Result<Expression, ParsingError> {
        Ok(Expression::Identifier(id.to_string()))
    }

    fn parse_integer(int: &str) -> Result<Expression, ParsingError> {
        int.parse::<i64>()
            .map(Expression::Integer)
            .map_err(|_| ParsingError::InvalidInteger(int.to_string()))
    }

    fn parse_string(string: &str) -> Result<Expression, ParsingError> {
        Ok(Expression::String(string.to_string()))
    }

    fn parse_boolean(val: bool) -> Result<Expression, ParsingError> {
        Ok(Expression::Boolean(val))
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, ParsingError> {
        let next_token = self.next_token_or_end()?;
        let exp = self.parse_expression(&next_token, Precedence::Lowest)?;
        if let Some(token) = self.iter.peek() {
            if *token != Token::Rparen {
                return Err(ParsingError::UnexpectedToken(token.clone()));
            } else {
                self.next_token_or_end()?;
            }
        }
        Ok(exp)
    }

    fn parse_if_expression(&mut self) -> Result<Expression, ParsingError> {
        // get and expect next token to be '(' after 'if'
        let token = match self.next_token_or_end()? {
            Token::Lparen => Token::Lparen,
            t => return Err(ParsingError::UnexpectedToken(t)),
        };

        // expect grouped expression after 'if' token
        let condition = self.parse_expression(&token, Precedence::Lowest)?;

        let consequence = Box::new(self.parse_block_statement()?);

        let alternative = match self.iter.peek() {
            Some(Token::Else) => {
                self.next_token_or_end()?;

                Some(Box::new(self.parse_block_statement()?))
            }
            _ => None,
        };

        Ok(Expression::If(
            Box::new(condition),
            consequence,
            alternative,
        ))
    }

    fn parse_function_literal(&mut self) -> Result<Expression, ParsingError> {
        // expect parameter list after 'fn' keyword
        let parameters = self.parse_function_parameters()?;

        // expect block statement after parameter list
        let body = Box::new(self.parse_block_statement()?);

        Ok(Expression::Function(parameters, body, String::new()))
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<Expression>, ParsingError> {
        // expect first token of parameter list to be '('
        match self.next_token_or_end()? {
            Token::Lparen => {}
            token => return Err(ParsingError::UnexpectedToken(token)),
        }

        let mut parameters = vec![];

        // handle case where there are no parameters
        if let Some(Token::Rparen) = self.iter.peek() {
            self.next_token_or_end()?;
            return Ok(parameters);
        }

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

    fn parse_prefix_expression(&mut self, token: &Token) -> Result<Expression, ParsingError> {
        let prefix = match token {
            Token::Bang => Prefix::Bang,
            Token::Minus => Prefix::Minus,
            _ => {
                return Err(ParsingError::Generic(String::from(
                    "should never get here... fix types",
                )))
            }
        };

        let next_token = self.next_token_or_end()?;

        let right_expression = self.parse_expression(&next_token, Precedence::Prefix)?;

        Ok(Expression::Prefix(prefix, Box::new(right_expression)))
    }

    fn parse_infix_expression(
        &mut self,
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
            Token::And => Infix::And,
            Token::Or => Infix::Or,
            _ => {
                return Err(ParsingError::Generic(String::from(
                    "should never get here... fix types",
                )))
            }
        };

        let precedence = Precedence::get_precedence(operator);

        let next_token = self.next_token_or_end()?;

        let right_expression = self.parse_expression(&next_token, precedence)?;

        Ok(Expression::Infix(
            Box::new(left_expression),
            infix,
            Box::new(right_expression),
        ))
    }

    fn parse_call_expression(
        &mut self,
        left_expression: Expression,
    ) -> Result<Expression, ParsingError> {
        if let Some(Token::Rparen) = self.iter.peek() {
            self.next_token_or_end()?;
            return Ok(Expression::Call(Box::new(left_expression), vec![]));
        }

        let arguments = self.parse_expression_list()?;

        match self.next_token_or_end()? {
            Token::Rparen => {}
            token => return Err(ParsingError::UnexpectedToken(token)),
        }

        Ok(Expression::Call(Box::new(left_expression), arguments))
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
