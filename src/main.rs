use std::io;

use crate::repl::Repl;

mod evaluator;
pub mod lexer;
mod parser;
pub mod repl;

fn main() -> io::Result<()> {
    Repl::start()
}
