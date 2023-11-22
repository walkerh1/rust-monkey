use std::{str::Chars, iter::Peekable};

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

    fn get_rest_of_number(&mut self, ch: char) -> u64 {
        let mut num = String::from(ch);
        while let Some(c) = self.iter.peek() {
            if c.is_ascii_digit() {
                // unwrap safe here since already peeked
                num.push(self.iter.next().unwrap());
            } else {
                break;
            }
        }
        // TODO: this is unsafe, e.g. if num == "00102"
        num.parse::<u64>().unwrap()
    }
}

impl<'a> Iterator for TokensIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        let ch = self.iter.next()?;
        match ch {
            '=' => Some(Token::ASSIGN),
            ';' => Some(Token::SEMICOLON),
            '(' => Some(Token::LPAREN),
            ')' => Some(Token::RPAREN),
            ',' => Some(Token::COMMA),
            '+' => Some(Token::PLUS),
            '{' => Some(Token::LBRACE),
            '}' => Some(Token::RBRACE),
            _ => {
                if ch.is_ascii_alphabetic() || ch == '_' {
                    let word = self.get_rest_of_word(ch);
                    match word.as_str() {
                        "let" => Some(Token::LET),
                        "fn" => Some(Token::FUNCTION),
                        _ => Some(Token::IDENTIFIER(word)),
                    }
                } else if ch.is_ascii_digit() {
                    let num = self.get_rest_of_number(ch);
                    Some(Token::INT(num))
                } else {
                    Some(Token::ILLEGAL)
                }
            }
        }
    }
}

trait Lex {
    fn tokens(&self) -> TokensIter;
}

impl Lex for str {
    fn tokens(&self) -> TokensIter {
        TokensIter { iter: self.chars().peekable() }
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
    let tokens = input.tokens().collect::<Vec<Token>>();
    assert_eq!(tests, tokens);
}