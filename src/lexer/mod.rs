use std::{iter::Peekable, str::Chars};

use token::Token;

mod tests;
pub mod token;

#[derive(Debug)]
pub struct LexerIter<'a> {
    iter: Peekable<Chars<'a>>,
}

impl<'a> LexerIter<'a> {
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

    fn get_rest_of_number(&mut self, ch: char) -> String {
        let mut num = String::from(ch);
        while let Some(c) = self.iter.peek() {
            if c.is_ascii_digit() {
                // unwrap safe here since already peeked
                num.push(self.iter.next().unwrap());
            } else {
                break;
            }
        }
        num
    }

    fn get_string(&mut self) -> String {
        let mut string = String::new();
        while let Some(c) = self.iter.peek() {
            if *c == '"' {
                self.iter.next();
                break;
            }
            string.push(*c);
            self.iter.next();
        }
        string
    }
}

impl<'a> Iterator for LexerIter<'a> {
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
            ':' => Some(Token::Colon),
            '(' => Some(Token::Lparen),
            ')' => Some(Token::Rparen),
            ',' => Some(Token::Comma),
            '{' => Some(Token::Lbrace),
            '}' => Some(Token::Rbrace),
            '[' => Some(Token::Lbracket),
            ']' => Some(Token::Rbracket),
            '&' => {
                if let Some(c) = self.iter.peek() {
                    if *c == '&' {
                        self.iter.next();
                        return Some(Token::And);
                    }
                }
                Some(Token::Illegal)
            }
            '|' => {
                if let Some(c) = self.iter.peek() {
                    if *c == '|' {
                        self.iter.next();
                        return Some(Token::Or);
                    }
                }
                Some(Token::Illegal)
            }
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
            '"' => Some(Token::String(self.get_string())),
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
                        "while" => Some(Token::While),
                        _ => Some(Token::Identifier(word)),
                    }
                } else if ch.is_ascii_digit() {
                    Some(Token::Int(self.get_rest_of_number(ch)))
                } else {
                    Some(Token::Illegal)
                }
            }
        }
    }
}

pub trait Lexer {
    fn tokens(&self) -> LexerIter;
}

impl Lexer for str {
    fn tokens(&self) -> LexerIter {
        LexerIter {
            iter: self.chars().peekable(),
        }
    }
}
