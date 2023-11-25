#![cfg(test)]

use crate::lexer::{token::Token, Lexer};

#[test]
fn test_lexer_one() {
    let input = "=+(){},;";
    let tests = vec![
        Token::Assign,
        Token::Plus,
        Token::Lparen,
        Token::Rparen,
        Token::Lbrace,
        Token::Rbrace,
        Token::Comma,
        Token::Semicolon,
    ];
    let tokens: Vec<_> = input.tokens().collect();
    assert_eq!(tests, tokens);
}

#[test]
fn test_lexer_two() {
    let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
";
    let tests = vec![
        Token::Let,
        Token::Identifier(String::from("five")),
        Token::Assign,
        Token::Int(String::from("5")),
        Token::Semicolon,
        Token::Let,
        Token::Identifier(String::from("ten")),
        Token::Assign,
        Token::Int(String::from("10")),
        Token::Semicolon,
        Token::Let,
        Token::Identifier(String::from("add")),
        Token::Assign,
        Token::Function,
        Token::Lparen,
        Token::Identifier(String::from("x")),
        Token::Comma,
        Token::Identifier(String::from("y")),
        Token::Rparen,
        Token::Lbrace,
        Token::Identifier(String::from("x")),
        Token::Plus,
        Token::Identifier(String::from("y")),
        Token::Semicolon,
        Token::Rbrace,
        Token::Semicolon,
        Token::Let,
        Token::Identifier(String::from("result")),
        Token::Assign,
        Token::Identifier(String::from("add")),
        Token::Lparen,
        Token::Identifier(String::from("five")),
        Token::Comma,
        Token::Identifier(String::from("ten")),
        Token::Rparen,
        Token::Semicolon,
    ];
    let tokens: Vec<_> = input.tokens().collect();
    assert_eq!(tests, tokens);
}

#[test]
fn test_lexer_three() {
    let input = "!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
";

    let tests = vec![
        Token::Bang,
        Token::Minus,
        Token::Slash,
        Token::Asterisk,
        Token::Int(String::from("5")),
        Token::Semicolon,
        Token::Int(String::from("5")),
        Token::Lt,
        Token::Int(String::from("10")),
        Token::Gt,
        Token::Int(String::from("5")),
        Token::Semicolon,
        Token::If,
        Token::Lparen,
        Token::Int(String::from("5")),
        Token::Lt,
        Token::Int(String::from("10")),
        Token::Rparen,
        Token::Lbrace,
        Token::Return,
        Token::True,
        Token::Semicolon,
        Token::Rbrace,
        Token::Else,
        Token::Lbrace,
        Token::Return,
        Token::False,
        Token::Semicolon,
        Token::Rbrace,
        Token::Int(String::from("10")),
        Token::Eq,
        Token::Int(String::from("10")),
        Token::Semicolon,
        Token::Int(String::from("10")),
        Token::Noteq,
        Token::Int(String::from("9")),
        Token::Semicolon,
    ];
    let tokens: Vec<_> = input.tokens().collect();
    assert_eq!(tests, tokens);
}
