use std::io::{self, Write};

use crate::lexer::Lexer;

pub struct Repl;

const PROMPT: &'static str = ">> ";

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
                write!(writer, "\n")?;
                break;
            }

            buffer.as_str().tokens().for_each(|token| {
                write!(writer, "{:?}\n", token).expect("Failed to write to stdout")
            })
        }

        Ok(())
    }
}
