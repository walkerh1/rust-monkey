#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    // non-visible tokens
    ILLEGAL,

    // identifiers and literals
    IDENTIFIER(&'a str),
    INT(&'a str),

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