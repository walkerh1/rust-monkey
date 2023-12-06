use crate::code::{make, Instructions, OpCode};
use crate::evaluator::object::Object;
use crate::parser::ast::{Expression, Infix, Prefix, Program, Statement};
use std::rc::Rc;

mod tests;

#[derive(Debug, PartialEq)]
pub struct ByteCode(pub Instructions, pub Vec<Rc<Object>>);

#[derive(Debug, PartialEq)]
pub struct Compiler {
    instructions: Instructions,
    constants: Vec<Rc<Object>>,
}

impl Compiler {
    pub fn compile(program: Program) -> Result<ByteCode, CompilerError> {
        let mut compiler = Compiler {
            instructions: vec![],
            constants: vec![],
        };
        let Program(statements) = program;
        compiler.compile_statements(&statements)?;
        Ok(ByteCode(compiler.instructions, compiler.constants))
    }

    fn compile_statements(&mut self, statements: &[Statement]) -> Result<(), CompilerError> {
        for statement in statements.iter() {
            self.compile_statement(statement)?;
        }
        Ok(())
    }

    fn compile_statement(&mut self, statement: &Statement) -> Result<(), CompilerError> {
        match statement {
            Statement::Let(_, _) => todo!(),
            Statement::Return(_) => todo!(),
            Statement::Expression(expression) => {
                self.compile_expression(expression)?;
                self.emit(OpCode::Pop, &[]);
            }
            Statement::BlockStatement(_) => todo!(),
            Statement::Assignment(_, _) => todo!(),
        }
        Ok(())
    }

    fn compile_expression(&mut self, expression: &Expression) -> Result<(), CompilerError> {
        match expression {
            Expression::Identifier(_) => todo!(),
            Expression::Integer(integer) => self.compile_integer_expression(*integer)?,
            Expression::Prefix(prefix, right) => self.compile_prefix_expression(prefix, right)?,
            Expression::Infix(left, infix, right) => {
                self.compile_infix_expression(left, infix, right)?
            }
            Expression::Boolean(value) => {
                if *value {
                    self.emit(OpCode::True, &[]);
                } else {
                    self.emit(OpCode::False, &[]);
                }
            }
            Expression::If(_, _, _) => todo!(),
            Expression::Function(_, _) => todo!(),
            Expression::Call(_, _) => todo!(),
            Expression::String(_) => todo!(),
            Expression::Array(_) => todo!(),
            Expression::Index(_, _) => todo!(),
            Expression::Hash(_) => todo!(),
            Expression::While(_, _) => todo!(),
        }
        Ok(())
    }

    fn compile_prefix_expression(
        &mut self,
        prefix: &Prefix,
        right: &Expression,
    ) -> Result<(), CompilerError> {
        self.compile_expression(right)?;
        match prefix {
            Prefix::Minus => {
                self.emit(OpCode::Minus, &[]);
            }
            Prefix::Bang => {
                self.emit(OpCode::Bang, &[]);
            }
        }
        Ok(())
    }

    fn compile_integer_expression(&mut self, integer: i64) -> Result<(), CompilerError> {
        let constant_address = self.add_constant(Object::Integer(integer));
        self.emit(OpCode::Constant, &[constant_address]);
        Ok(())
    }

    fn compile_infix_expression(
        &mut self,
        left: &Expression,
        infix: &Infix,
        right: &Expression,
    ) -> Result<(), CompilerError> {
        if *infix == Infix::LessThan {
            self.compile_expression(right)?;
            self.compile_expression(left)?;
        } else {
            self.compile_expression(left)?;
            self.compile_expression(right)?;
        }
        match infix {
            Infix::Plus => {
                self.emit(OpCode::Add, &[]);
            }
            Infix::Minus => {
                self.emit(OpCode::Subtract, &[]);
            }
            Infix::Multiply => {
                self.emit(OpCode::Multiply, &[]);
            }
            Infix::Divide => {
                self.emit(OpCode::Divide, &[]);
            }
            Infix::GreaterThan => {
                self.emit(OpCode::GreaterThan, &[]);
            }
            Infix::LessThan => {
                self.emit(OpCode::GreaterThan, &[]);
            }
            Infix::Equal => {
                self.emit(OpCode::Equal, &[]);
            }
            Infix::NotEqual => {
                self.emit(OpCode::NotEqual, &[]);
            }
            Infix::And => todo!(),
            Infix::Or => todo!(),
        }
        Ok(())
    }

    fn add_constant(&mut self, object: Object) -> u32 {
        self.constants.push(Rc::new(object));
        (self.constants.len() - 1) as u32
    }

    fn emit(&mut self, op: OpCode, operands: &[u32]) -> u32 {
        let instruction = make(op, operands);
        let instruction_address = self.instructions.len();
        self.instructions.extend(&instruction);
        instruction_address as u32
    }
}

#[derive(Debug, PartialEq)]
pub enum CompilerError {}
