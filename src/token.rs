#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    ILLEGAL,
    EOF,
    IDENTIFIER(&'a str),
    INT(&'a str),
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
}