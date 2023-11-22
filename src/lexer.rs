use crate::token::Token;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer<'_> {
    pub fn new(input: &str) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        let token = if let Some(token) = self.ch {
            match token {
                '=' => Token::ASSIGN,
                ';' => Token::SEMICOLON,
                '(' => Token::LPAREN,
                ')' => Token::RPAREN,
                ',' => Token::COMMA,
                '+' => Token::PLUS,
                '{' => Token::LBRACE,
                '}' => Token::RBRACE,
                _ => Token::ILLEGAL,
            }
        } else {
            Token::EOF
        };
        self.read_char();
        token
    }

    fn read_char(&mut self) {
        self.ch = if self.read_position < self.input.len() {
            self.input.chars().nth(self.read_position)
        } else {
            None
        };
        self.position = self.read_position;
        self.read_position += 1;
    }
}

#[test]
fn test_next_token() {
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
        Token::EOF,
    ];
    let mut lexer = Lexer::new(input);
    tests.iter().for_each(|token| assert_eq!(*token, lexer.next_token()));
}