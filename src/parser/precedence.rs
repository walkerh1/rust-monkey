use crate::lexer::token::Token;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Lowest = 0,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

impl Precedence {
    pub fn get_precedence(token: &Token) -> Precedence {
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
