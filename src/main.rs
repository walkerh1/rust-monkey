use lexer::Lexer;

pub mod token;
pub mod lexer;

fn main() {
    let lexer = Lexer::new("123");
}
