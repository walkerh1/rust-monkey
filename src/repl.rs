use crate::parser::Parser;
use crate::symtab::SymbolTable;
use crate::vm::{VirtualMachine, GLOBAL_SIZE};
use crate::{compiler::Compiler, object::Object};
use std::io::{self, Write};
use std::rc::Rc;

pub struct Repl;

const PROMPT: &str = ">> ";

impl Repl {
    pub fn start() -> io::Result<()> {
        let reader = io::stdin();
        let mut writer = io::stdout();

        let mut symtab = SymbolTable::new();
        let mut constants = vec![];
        let mut globals = vec![Rc::new(Object::Null); GLOBAL_SIZE];

        loop {
            writer.write_all(PROMPT.as_bytes())?;
            writer.flush()?;

            let mut buffer = String::new();
            let bytes_read = reader.read_line(&mut buffer)?;

            if bytes_read == 0 {
                writeln!(writer)?;
                break;
            }

            let parsing_result = Parser::parse_program(buffer.as_str());
            let program = match parsing_result {
                Ok(program) => program,
                Err(errors) => {
                    errors.iter().for_each(|e| println!("{e:?}"));
                    continue;
                }
            };

            let mut compiler = Compiler::new_with_state(symtab, constants);

            let byte_code = match compiler.compile(program) {
                Ok(res) => res,
                Err(e) => {
                    println!("{e:?}");
                    symtab = compiler.symbol_table;
                    constants = compiler.constants;
                    continue;
                }
            };

            let mut vm = VirtualMachine::new_with_global_state(byte_code, globals);

            match vm.run() {
                Ok(obj) => println!("{obj}"),
                Err(e) => println!("{e:?}"),
            }

            symtab = compiler.symbol_table;
            constants = compiler.constants;
            globals = vm.globals;
        }

        Ok(())
    }
}
