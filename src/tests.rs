#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, token::Token};

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
}
