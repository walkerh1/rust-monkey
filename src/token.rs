#[derive(Debug, PartialEq)]
pub enum Token {
    // non-visible tokens
    ILLEGAL,

    // identifiers and literals
    IDENTIFIER(String),
    INT(u64),

    // operators
    ASSIGN,
    PLUS,

    // delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // keywords
    FUNCTION,
    LET,
}