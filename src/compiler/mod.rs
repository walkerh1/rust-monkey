use crate::code::{make, Instructions, OpCode, WORD_SIZE};
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
            Statement::BlockStatement(statements) => self.compile_block_statement(statements)?,
            Statement::Assignment(_, _) => todo!(),
        }
        Ok(())
    }

    fn compile_block_statement(&mut self, block: &[Statement]) -> Result<(), CompilerError> {
        for statement in block.iter() {
            self.compile_statement(statement)?;
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
            Expression::If(condition, consequence, alternative) => {
                self.compile_if_expression(condition, consequence, alternative)?
            }
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

    fn compile_if_expression(
        &mut self,
        condition: &Expression,
        consequence: &Statement,
        alternative: &Option<Box<Statement>>,
    ) -> Result<(), CompilerError> {
        self.compile_expression(condition)?;

        let jump_not_truthy_pos = self.emit(OpCode::JumpNotTruthy, &[9999_u32]);

        self.compile_statement(consequence)?;

        if self.last_instruction_is_pop() {
            self.remove_last_instruction();
        }

        let jump_pos = self.emit(OpCode::Jump, &[9999_u32]);

        let after_consequence_pos = self.instructions.len() as u32;
        self.change_operand(jump_not_truthy_pos as usize, after_consequence_pos)?;

        if alternative.is_none() {
            self.emit(OpCode::Null, &[]);
        } else {
            let else_block = alternative.as_ref().unwrap();
            self.compile_statement(&else_block)?;

            if self.last_instruction_is_pop() {
                self.remove_last_instruction();
            }
        }

        let after_consequence_pos = self.instructions.len() as u32;
        self.change_operand(jump_pos as usize, after_consequence_pos)?;

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

    fn last_instruction_is_pop(&self) -> bool {
        let last_op_code = self.get_instruction_at(self.instructions.len() - WORD_SIZE);
        if let Ok(op_code) = last_op_code {
            return op_code == OpCode::Pop;
        }
        false
    }

    fn remove_last_instruction(&mut self) {
        for _ in 0..WORD_SIZE {
            self.instructions.pop();
        }
    }

    fn change_operand(&mut self, op_address: usize, operand: u32) -> Result<(), CompilerError> {
        let op = self.get_instruction_at(op_address)?;
        let new_instruction = make(op, &[operand]);
        self.replace_instruction(op_address, &new_instruction)?;
        Ok(())
    }

    fn replace_instruction(
        &mut self,
        address: usize,
        new_instruction: &[u8],
    ) -> Result<(), CompilerError> {
        let mut i = 0;
        for _ in 0..WORD_SIZE {
            self.instructions[address + i] = new_instruction[i];
            i += 1;
        }
        Ok(())
    }

    fn get_instruction_at(&self, idx: usize) -> Result<OpCode, CompilerError> {
        OpCode::try_from(self.instructions[idx] as u8).map_err(|_| CompilerError::InvalidOpCode)
    }

    fn emit(&mut self, op: OpCode, operands: &[u32]) -> u32 {
        let instruction = make(op, operands);
        let instruction_address = self.instructions.len();
        self.instructions.extend(&instruction);
        instruction_address as u32
    }
}

#[derive(Debug, PartialEq)]
pub enum CompilerError {
    InvalidOpCode,
}
