use std::{str::Chars, iter::Peekable, num::ParseIntError};

use crate::token::Token;

pub struct TokensIter<'a> {
    iter: Peekable<Chars<'a>>
}

impl<'a> TokensIter<'a> {
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.iter.peek() {
            if c.is_ascii_whitespace() {
                self.iter.next();
            } else {
                break;
            }
        }
    }

    fn get_rest_of_word(&mut self, ch: char) -> String {
        let mut word = String::from(ch);
        while let Some(c) = self.iter.peek() {
            if c.is_ascii_alphabetic() || *c == '_' {
                // unwrap safe here since already peeked
                word.push(self.iter.next().unwrap());
            } else {
                break;
            }
        }
        word
    }

    fn get_rest_of_number(&mut self, ch: char) -> Result<u64, ParseIntError> {
        let mut num = String::from(ch);
        while let Some(c) = self.iter.peek() {
            if c.is_ascii_digit() {
                // unwrap safe here since already peeked
                num.push(self.iter.next().unwrap());
            } else {
                break;
            }
        }
        num.parse::<u64>()
    }
}

impl<'a> Iterator for TokensIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        let ch = self.iter.next()?;
        match ch {
            '+' => Some(Token::PLUS),
            '-' => Some(Token::MINUS),
            '*' => Some(Token::ASTERISK),
            '/' => Some(Token::SLASH),
            '<' => Some(Token::LT),
            '>' => Some(Token::GT),
            ';' => Some(Token::SEMICOLON),
            '(' => Some(Token::LPAREN),
            ')' => Some(Token::RPAREN),
            ',' => Some(Token::COMMA),
            '{' => Some(Token::LBRACE),
            '}' => Some(Token::RBRACE),
            '=' => {
                if let Some(c) = self.iter.peek() {
                    if *c == '=' {
                        self.iter.next();
                        return Some(Token::EQ);
                    }
                }
                Some(Token::ASSIGN)
            },
            '!' => {
                if let Some(c) = self.iter.peek() {
                    if *c == '=' {
                        self.iter.next();
                        return Some(Token::NOTEQ);
                    }
                }
                Some(Token::BANG)
            },
            _ => {
                if ch.is_ascii_alphabetic() || ch == '_' {
                    let word = self.get_rest_of_word(ch);
                    match word.as_str() {
                        "let" => Some(Token::LET),
                        "fn" => Some(Token::FUNCTION),
                        "true" => Some(Token::TRUE),
                        "false" => Some(Token::FALSE),
                        "if" => Some(Token::IF),
                        "else" => Some(Token::ELSE),
                        "return" => Some(Token::RETURN),
                        _ => Some(Token::IDENTIFIER(word)),
                    }
                } else if ch.is_ascii_digit() {
                    let num = self.get_rest_of_number(ch);
                    match num {
                        Ok(val) => Some(Token::INT(val)),
                        _ => Some(Token::ILLEGAL) // badly formatted number
                    }
                } else {
                    Some(Token::ILLEGAL)
                }
            }
        }
    }
}

trait Lexer {
    fn tokens(&self) -> TokensIter;
}

impl Lexer for str {
    fn tokens(&self) -> TokensIter {
        TokensIter { iter: self.chars().peekable() }
    }
}

#[test]
fn test_lexer_one() {
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
        Token::LET,
        Token::IDENTIFIER(String::from("five")),
        Token::ASSIGN,
        Token::INT(5),
        Token::SEMICOLON,
        Token::LET,
        Token::IDENTIFIER(String::from("ten")),
        Token::ASSIGN,
        Token::INT(10),
        Token::SEMICOLON,
        Token::LET,
        Token::IDENTIFIER(String::from("add")),
        Token::ASSIGN,
        Token::FUNCTION,
        Token::LPAREN,
        Token::IDENTIFIER(String::from("x")),
        Token::COMMA,
        Token::IDENTIFIER(String::from("y")),
        Token::RPAREN,
        Token::LBRACE,
        Token::IDENTIFIER(String::from("x")),
        Token::PLUS,
        Token::IDENTIFIER(String::from("y")),
        Token::SEMICOLON,
        Token::RBRACE,
        Token::SEMICOLON,
        Token::LET,
        Token::IDENTIFIER(String::from("result")),
        Token::ASSIGN,
        Token::IDENTIFIER(String::from("add")),
        Token::LPAREN,
        Token::IDENTIFIER(String::from("five")),
        Token::COMMA,
        Token::IDENTIFIER(String::from("ten")),
        Token::RPAREN,
        Token::SEMICOLON,
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
        Token::BANG,
        Token::MINUS,
        Token::SLASH,
        Token::ASTERISK,
        Token::INT(5),
        Token::SEMICOLON,
        Token::INT(5),
        Token::LT,
        Token::INT(10),
        Token::GT,
        Token::INT(5),
        Token::SEMICOLON,
        Token::IF,
        Token::LPAREN,
        Token::INT(5),
        Token::LT,
        Token::INT(10),
        Token::RPAREN,
        Token::LBRACE,
        Token::RETURN,
        Token::TRUE,
        Token::SEMICOLON,
        Token::RBRACE,
        Token::ELSE,
        Token::LBRACE,
        Token::RETURN,
        Token::FALSE,
        Token::SEMICOLON,
        Token::RBRACE,
        Token::INT(10),
        Token::EQ,
        Token::INT(10),
        Token::SEMICOLON,
        Token::INT(10),
        Token::NOTEQ,
        Token::INT(9),
        Token::SEMICOLON,
    ];
    let tokens: Vec<_> = input.tokens().collect();
    assert_eq!(tests, tokens);
}