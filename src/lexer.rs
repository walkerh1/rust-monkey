pub mod lexer;

struct Lexer {
    input: String,
    position: u32,
    read_position: u32,
    ch: char,
}

impl Lexer {
    fn new(input: &str) -> Lexer {
        return Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0
        }
    }
}