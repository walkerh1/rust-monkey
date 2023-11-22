use std::io;

use repl::Repl;

mod lexer;
pub mod repl;
mod tests;
mod token;

fn main() -> io::Result<()> {
    Repl::start()
}
