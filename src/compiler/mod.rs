use crate::code::{make, Instructions, OpCode, WORD_SIZE};
use crate::evaluator::object::{CompiledFunction, Object};
use crate::parser::ast::{Expression, Infix, Prefix, Program, Statement};
use crate::symtab::{SymbolScope, SymbolTable};
use std::ops::Deref;
use std::rc::Rc;

mod tests;

#[derive(Debug, PartialEq)]
pub struct ByteCode(pub Instructions, pub Vec<Rc<Object>>);

#[derive(Debug, PartialEq)]
pub struct Compiler {
    pub constants: Vec<Rc<Object>>,
    pub symbol_table: SymbolTable,
    scopes: Vec<Instructions>,
    scope_idx: usize,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            constants: vec![],
            symbol_table: SymbolTable::new(),
            scopes: vec![Instructions::new()],
            scope_idx: 0,
        }
    }

    pub fn new_with_state(symbol_table: SymbolTable, constants: Vec<Rc<Object>>) -> Self {
        let mut compiler = Self::new();
        compiler.symbol_table = symbol_table;
        compiler.constants = constants;
        compiler
    }

    pub fn compile(&mut self, program: Program) -> Result<ByteCode, CompilerError> {
        let Program(statements) = program;
        self.compile_statements(&statements)?;
        Ok(ByteCode(
            self.scopes[self.scope_idx].clone(),
            self.constants.clone(),
        ))
    }

    fn compile_statements(&mut self, statements: &[Statement]) -> Result<(), CompilerError> {
        for statement in statements.iter() {
            self.compile_statement(statement)?;
        }
        Ok(())
    }

    fn compile_statement(&mut self, statement: &Statement) -> Result<(), CompilerError> {
        match statement {
            Statement::Let(id, val) => self.compile_let(id, val)?,
            Statement::Return(val) => {
                self.compile_expression(val)?;
                self.emit(OpCode::ReturnValue, &[]);
            }
            Statement::Expression(expression) => {
                self.compile_expression(expression)?;
                self.emit(OpCode::Pop, &[]);
            }
            Statement::BlockStatement(statements) => self.compile_block_statement(statements)?,
            Statement::Assignment(_, _) => todo!(),
        }
        Ok(())
    }

    fn compile_let(&mut self, id: &Expression, val: &Expression) -> Result<(), CompilerError> {
        self.compile_expression(val)?;
        if let Expression::Identifier(id) = id {
            let symbol = self.symbol_table.define(id.to_string());
            match symbol.scope {
                SymbolScope::Global => self.emit(OpCode::SetGlobal, &[symbol.index]),
                SymbolScope::Local => self.emit(OpCode::SetLocal, &[symbol.index]),
            };
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
            Expression::Identifier(id) => match self.symbol_table.resolve(id.to_string()) {
                Some(binding) => match binding.scope {
                    SymbolScope::Global => {
                        self.emit(OpCode::GetGlobal, &[binding.index]);
                    }
                    SymbolScope::Local => {
                        self.emit(OpCode::GetLocal, &[binding.index]);
                    }
                },
                None => {
                    return Err(CompilerError::UndefinedVariable);
                }
            },
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
            Expression::Function(_, body) => {
                self.enter_scope();
                self.compile_statement(body)?;
                if self.last_instruction_is(OpCode::Pop) {
                    let address = self.scopes[self.scope_idx].len() - WORD_SIZE;
                    self.replace_instruction(address, &make(OpCode::ReturnValue, &[]))?;
                }
                if !self.last_instruction_is(OpCode::ReturnValue) {
                    self.emit(OpCode::Return, &[]);
                }
                let num_locals = self.symbol_table.num_definitions;
                let instructions = self.leave_scope();
                let compilted_fn =
                    Object::CompiledFunc(Rc::new(CompiledFunction::new(instructions, num_locals)));
                let address = self.add_constant(compilted_fn);
                self.emit(OpCode::Constant, &[address]);
            }
            Expression::Call(func, args) => {
                self.compile_expression(func)?;
                for arg in args {
                    self.compile_expression(arg)?;
                }
                self.emit(OpCode::Call, &[args.len() as u32]);
            }
            Expression::String(val) => {
                let str = Object::String(val.clone());
                let address = self.add_constant(str);
                self.emit(OpCode::Constant, &[address]);
            }
            Expression::Array(val) => {
                for exp in val.into_iter() {
                    self.compile_expression(exp)?;
                }
                self.emit(OpCode::Array, &[val.len() as u32]);
            }
            Expression::Index(store, i) => {
                self.compile_expression(store)?;
                self.compile_expression(i)?;
                self.emit(OpCode::Index, &[]);
            }
            Expression::Hash(val) => {
                for (k, v) in val {
                    self.compile_expression(k)?;
                    self.compile_expression(v)?;
                }
                self.emit(OpCode::Hash, &[(val.len() * 2) as u32]);
            }
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

        if self.last_instruction_is(OpCode::Pop) {
            self.remove_last_instruction();
        }

        let jump_pos = self.emit(OpCode::Jump, &[9999_u32]);

        let after_consequence_pos = self.scopes[self.scope_idx].len() as u32;
        self.change_operand(jump_not_truthy_pos as usize, after_consequence_pos)?;

        if alternative.is_none() {
            self.emit(OpCode::Null, &[]);
        } else {
            let else_block = alternative.as_ref().unwrap();
            self.compile_statement(&else_block)?;

            if self.last_instruction_is(OpCode::Pop) {
                self.remove_last_instruction();
            }
        }

        let after_consequence_pos = self.scopes[self.scope_idx].len() as u32;
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

    fn last_instruction_is(&self, target_opcode: OpCode) -> bool {
        if self.scopes[self.scope_idx].len() == 0 {
            return false;
        }
        let last_op_code = self.get_instruction_at(self.scopes[self.scope_idx].len() - WORD_SIZE);
        if let Ok(op_code) = last_op_code {
            return op_code == target_opcode;
        }
        false
    }

    fn remove_last_instruction(&mut self) {
        for _ in 0..WORD_SIZE {
            self.scopes[self.scope_idx].pop();
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
            self.scopes[self.scope_idx][address + i] = new_instruction[i];
            i += 1;
        }
        Ok(())
    }

    fn get_instruction_at(&self, idx: usize) -> Result<OpCode, CompilerError> {
        OpCode::try_from(self.scopes[self.scope_idx][idx] as u8)
            .map_err(|_| CompilerError::InvalidOpCode)
    }

    fn emit(&mut self, op: OpCode, operands: &[u32]) -> u32 {
        let instruction = make(op, operands);
        let instruction_address = self.scopes[self.scope_idx].len();
        self.scopes[self.scope_idx].extend(&instruction);
        instruction_address as u32
    }

    fn enter_scope(&mut self) {
        self.symbol_table = SymbolTable::new_enclosed(self.symbol_table.clone());
        self.scopes.push(Instructions::new());
        self.scope_idx += 1;
    }

    fn leave_scope(&mut self) -> Instructions {
        self.symbol_table = self.symbol_table.outer.as_ref().unwrap().as_ref().clone();
        self.scope_idx -= 1;
        self.scopes.pop().unwrap()
    }
}

#[derive(Debug, PartialEq)]
pub enum CompilerError {
    InvalidOpCode,
    UndefinedVariable,
}
