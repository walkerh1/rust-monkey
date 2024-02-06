use std::io;

use crate::repl::Repl;

mod code;
mod compiler;
mod evaluator;
pub mod lexer;
mod object;
mod parser;
pub mod repl;
mod symtab;
mod vm;

fn main() -> io::Result<()> {
    Repl::start()
}
