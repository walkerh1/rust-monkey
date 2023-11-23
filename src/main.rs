use std::io;

use crate::repl::Repl;

pub mod lexer;
mod parser;
pub mod repl;

fn main() -> io::Result<()> {
    Repl::start()
}
