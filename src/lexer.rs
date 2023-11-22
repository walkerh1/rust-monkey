use std::str::Chars;

use crate::token::Token;

pub struct Tokens<'a> {
    iter: Chars<'a>
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next()? {
            '=' => Some(Token::ASSIGN),
            ';' => Some(Token::SEMICOLON),
            '(' => Some(Token::LPAREN),
            ')' => Some(Token::RPAREN),
            ',' => Some(Token::COMMA),
            '+' => Some(Token::PLUS),
            '{' => Some(Token::LBRACE),
            '}' => Some(Token::RBRACE),
            _ => Some(Token::ILLEGAL)
        }
    }
}

trait Lex {
    fn tokens(&self) -> Tokens;
}

impl Lex for str {
    fn tokens(&self) -> Tokens {
        Tokens { iter: self.chars() }
    }
}

#[test]
fn test_next_token_one() {
    let input = "=+(){},;";
    let tests = vec![
        Token::ASSIGN,
        Token::PLUS,
        Token::LPAREN,
        Token::RPAREN,
        Token::LBRACE,
        Token::RBRACE,
        Token::COMMA,
        Token::SEMICOLON,
    ];
    let tokens: Vec<Token> = input.tokens().collect();
    assert_eq!(tests, tokens);
}

#[test]
fn test_next_token_two() {
    let input = "
let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
";
    let tests = vec![
        Token::LET,
        Token::IDENTIFIER("five"),
        Token::ASSIGN,
        Token::INT("5"),
        Token::SEMICOLON,
        Token::LET,
        Token::IDENTIFIER("ten"),
        Token::ASSIGN,
        Token::INT("10"),
        Token::SEMICOLON,
        Token::LET,
        Token::IDENTIFIER("add"),
        Token::ASSIGN,
        Token::FUNCTION,
        Token::LPAREN,
        Token::IDENTIFIER("x"),
        Token::COMMA,
        Token::IDENTIFIER("y"),
        Token::RPAREN,
        Token::LBRACE,
        Token::IDENTIFIER("x"),
        Token::PLUS,
        Token::IDENTIFIER("y"),
        Token::SEMICOLON,
        Token::RBRACE,
        Token::SEMICOLON,
        Token::LET,
        Token::IDENTIFIER("result"),
        Token::ASSIGN,
        Token::IDENTIFIER("add"),
        Token::LPAREN,
        Token::IDENTIFIER("five"),
        Token::COMMA,
        Token::IDENTIFIER("ten"),
        Token::RPAREN,
        Token::SEMICOLON,
    ];
    let tokens = input.tokens().collect::<Vec<Token>>();
    assert_eq!(tests, tokens);
}