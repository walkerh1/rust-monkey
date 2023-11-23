use std::io;

use crate::repl::Repl;

pub mod lexer;
pub mod repl;

fn main() -> io::Result<()> {
    Repl::start()
}
