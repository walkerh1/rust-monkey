use std::{iter::Peekable, num::ParseIntError, str::Chars};

use token::Token;

mod tests;
pub mod token;

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
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Asterisk),
            '/' => Some(Token::Slash),
            '<' => Some(Token::Lt),
            '>' => Some(Token::Gt),
            ';' => Some(Token::Semicolon),
            '(' => Some(Token::Lparen),
            ')' => Some(Token::Rparen),
            ',' => Some(Token::Comma),
            '{' => Some(Token::Lbrace),
            '}' => Some(Token::Rbrace),
            '=' => {
                if let Some(c) = self.iter.peek() {
                    if *c == '=' {
                        self.iter.next();
                        return Some(Token::Eq);
                    }
                }
                Some(Token::Assign)
            }
            '!' => {
                if let Some(c) = self.iter.peek() {
                    if *c == '=' {
                        self.iter.next();
                        return Some(Token::Noteq);
                    }
                }
                Some(Token::Bang)
            }
            _ => {
                if ch.is_ascii_alphabetic() || ch == '_' {
                    let word = self.get_rest_of_word(ch);
                    match word.as_str() {
                        "let" => Some(Token::Let),
                        "fn" => Some(Token::Function),
                        "true" => Some(Token::True),
                        "false" => Some(Token::False),
                        "if" => Some(Token::If),
                        "else" => Some(Token::Else),
                        "return" => Some(Token::Return),
                        _ => Some(Token::Identifier(word)),
                    }
                } else if ch.is_ascii_digit() {
                    let num = self.get_rest_of_number(ch);
                    if let Ok(val) = num {
                        return Some(Token::Int(val));
                    }
                    Some(Token::Illegal)
                } else {
                    Some(Token::Illegal)
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
