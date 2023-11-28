use std::io::{self, Write};

use crate::parser::Parser;

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

            let mut errors = vec![];
            let nodes: Vec<_> = buffer
                .as_str()
                .ast_nodes()
                .filter_map(|node| node.map_err(|e| errors.push(e)).ok())
                .collect();

            if errors.len() > 0 {
                errors.iter().for_each(|e| println!("{e:?}"));
            } else {
                nodes.iter().for_each(|node| println!("{node:?}"));
            }
        }

        Ok(())
    }
}
