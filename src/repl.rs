use crate::parser::ast::Program;
use crate::parser::{Parser, ParsingError};
use std::io::{self, Write};

pub struct Repl;

const PROMPT: &str = ">> ";

impl Repl {
    pub fn start() -> io::Result<()> {
        let reader = io::stdin();
        let mut writer = io::stdout();

        loop {
            writer.write_all(PROMPT.as_bytes())?;
            writer.flush()?;

            let mut buffer = String::new();
            let bytes_read = reader.read_line(&mut buffer)?;

            if bytes_read == 0 {
                writeln!(writer)?;
                break;
            }

            let result = Parser::parse_program(buffer.as_str());
            match result {
                Ok(nodes) => println!("{nodes:?}"),
                Err(errors) => errors.iter().for_each(|e| println!("{e:?}")),
            }
        }

        Ok(())
    }
}
