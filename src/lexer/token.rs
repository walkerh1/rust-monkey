#[derive(Debug, PartialEq)]
pub enum Token {
    // identifiers and literals
    Identifier(String),
    Int(u64),

    // operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Eq,
    Noteq,

    // delimiters
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    // keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,

    // bad token
    Illegal,
}
