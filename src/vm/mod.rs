use crate::code::{read_u16, OpCode, WORD_SIZE};
use crate::compiler::ByteCode;
use crate::evaluator::object::Object;
use std::rc::Rc;

mod tests;

const STACK_SIZE: usize = 2048; // 2KB

const TRUE: Object = Object::Boolean(true);
const FALSE: Object = Object::Boolean(false);

#[derive(Debug, PartialEq)]
pub struct VirtualMachine {
    stack: Vec<Rc<Object>>,
}

impl VirtualMachine {
    pub fn run(byte_code: ByteCode) -> Result<Rc<Object>, VmError> {
        let mut vm = VirtualMachine {
            stack: Vec::with_capacity(STACK_SIZE),
        };

        let mut last_popped = None;

        let ByteCode(instructions, constants) = byte_code;

        for word in instructions.chunks_exact(WORD_SIZE) {
            let op = match OpCode::try_from(word[0]) {
                Ok(op_code) => op_code,
                Err(_) => return Err(VmError::UnknownOpCode),
            };

            match op {
                OpCode::Constant => {
                    let const_index = read_u16(&word[1..=2]);
                    let object = Rc::clone(&constants[const_index as usize]);
                    vm.push(&object)?;
                }
                OpCode::Add | OpCode::Subtract | OpCode::Multiply | OpCode::Divide => {
                    let right = vm.pop()?;
                    let left = vm.pop()?;
                    match (&*left, &*right) {
                        (Object::Integer(left_val), Object::Integer(right_val)) => {
                            vm.execute_integer_operation(*left_val, op, *right_val)?;
                        }
                        _ => return Err(VmError::IncompatibleTypes),
                    }
                }
                OpCode::True => {
                    vm.push(&Rc::new(TRUE))?;
                }
                OpCode::False => {
                    vm.push(&Rc::new(FALSE))?;
                }
                OpCode::Pop => {
                    last_popped = Some(vm.pop()?);
                }
            }
        }

        match last_popped {
            Some(obj) => Ok(obj),
            None => Err(VmError::EmptyStack),
        }
    }

    fn execute_integer_operation(
        &mut self,
        left: i64,
        op_code: OpCode,
        right: i64,
    ) -> Result<(), VmError> {
        let result = match op_code {
            OpCode::Add => left + right,
            OpCode::Subtract => left - right,
            OpCode::Multiply => left * right,
            OpCode::Divide => left / right,
            _ => return Err(VmError::IncompatibleTypes),
        };
        self.push(&Rc::new(Object::Integer(result)))
    }

    fn push(&mut self, object: &Rc<Object>) -> Result<(), VmError> {
        if self.stack.len() == STACK_SIZE {
            return Err(VmError::StackOverflow);
        }
        self.stack.push(Rc::clone(object));
        Ok(())
    }

    fn pop(&mut self) -> Result<Rc<Object>, VmError> {
        match self.stack.pop() {
            Some(object) => Ok(Rc::clone(&object)),
            None => Err(VmError::StackUnderflow),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum VmError {
    UnknownOpCode,
    StackOverflow,
    StackUnderflow,
    EmptyStack,
    IncompatibleTypes,
}
