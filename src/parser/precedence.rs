use crate::lexer::token::Token;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Precedence {
    Lowest = 0,
    Logical,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
    Index,
}

impl Precedence {
    pub fn get_precedence(token: &Token) -> Precedence {
        match token {
            Token::And | Token::Or => Precedence::Logical,
            Token::Eq | Token::Noteq => Precedence::Equals,
            Token::Lt | Token::Gt => Precedence::LessGreater,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Asterisk | Token::Slash => Precedence::Product,
            Token::Lparen => Precedence::Call,
            Token::Lbracket => Precedence::Index,
            _ => Precedence::Lowest,
        }
    }
}
