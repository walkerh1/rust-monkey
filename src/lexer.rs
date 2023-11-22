use std::{iter::Peekable, num::ParseIntError, str::Chars};

use crate::token::Token;

pub struct TokensIter<'a> {
    iter: Peekable<Chars<'a>>,
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
            }
            '!' => {
                if let Some(c) = self.iter.peek() {
                    if *c == '=' {
                        self.iter.next();
                        return Some(Token::NOTEQ);
                    }
                }
                Some(Token::BANG)
            }
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
                    if let Ok(val) = num {
                        return Some(Token::INT(val));
                    }
                    Some(Token::ILLEGAL)
                } else {
                    Some(Token::ILLEGAL)
                }
            }
        }
    }
}

pub trait Lexer {
    fn tokens(&self) -> TokensIter;
}

impl Lexer for str {
    fn tokens(&self) -> TokensIter {
        TokensIter {
            iter: self.chars().peekable(),
        }
    }
}
