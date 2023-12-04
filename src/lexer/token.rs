use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // identifiers and literals
    Identifier(String),
    Int(String),
    String(String),

    // operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Eq,
    Noteq,

    // delimiters
    Comma,
    Semicolon,
    Colon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Lbracket,
    Rbracket,

    // keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,

    // misc
    Illegal,
    Eof,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::Identifier(id) => id.to_string(),
                Token::Int(val) => val.to_string(),
                Token::String(string) => string.to_string(),
                Token::Assign => String::from("="),
                Token::Plus => String::from("+"),
                Token::Minus => String::from("-"),
                Token::Bang => String::from("!"),
                Token::Asterisk => String::from("*"),
                Token::Slash => String::from("/"),
                Token::Lt => String::from("<"),
                Token::Gt => String::from(">"),
                Token::Eq => String::from("=="),
                Token::Noteq => String::from("!="),
                Token::Comma => String::from(","),
                Token::Semicolon => String::from(";"),
                Token::Colon => String::from(":"),
                Token::Lparen => String::from("("),
                Token::Rparen => String::from(")"),
                Token::Lbrace => String::from("{"),
                Token::Rbrace => String::from("}"),
                Token::Lbracket => String::from("["),
                Token::Rbracket => String::from("]"),
                Token::Function => String::from("fn"),
                Token::Let => String::from("let"),
                Token::True => String::from("true"),
                Token::False => String::from("false"),
                Token::If => String::from("if"),
                Token::Else => String::from("else"),
                Token::Return => String::from("return"),
                Token::Illegal => String::from("illegal token"),
                Token::Eof => String::from("EOF"),
            }
        )
    }
}
