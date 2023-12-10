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

        let mut ip = 0;

        while ip < instructions.len() {
            let word = &instructions[ip..ip + WORD_SIZE];

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
                OpCode::Add
                | OpCode::Subtract
                | OpCode::Multiply
                | OpCode::Divide
                | OpCode::Equal
                | OpCode::NotEqual
                | OpCode::GreaterThan => {
                    vm.execute_binary_expression(op)?;
                }
                OpCode::True => {
                    vm.push(&Rc::new(TRUE))?;
                }
                OpCode::False => {
                    vm.push(&Rc::new(FALSE))?;
                }
                OpCode::Minus => {
                    vm.execute_minus_expression()?;
                }
                OpCode::Bang => {
                    vm.execute_bang_expression()?;
                }
                OpCode::Pop => {
                    last_popped = Some(vm.pop()?);
                }
                OpCode::Jump => {
                    let pos = read_u16(&word[1..=2]) as usize;
                    ip = pos;
                    continue;
                }
                OpCode::JumpNotTruthy => {
                    let pos = read_u16(&word[1..=2]) as usize;
                    let condition = vm.pop()?;
                    if !VirtualMachine::is_truthy(&*condition) {
                        ip = pos;
                        continue;
                    }
                }
            }

            ip += WORD_SIZE;
        }

        match last_popped {
            Some(obj) => Ok(obj),
            None => Err(VmError::EmptyStack),
        }
    }

    fn execute_minus_expression(&mut self) -> Result<(), VmError> {
        let right = self.pop()?;
        if let Object::Integer(int) = &*right {
            self.push(&Rc::new(Object::Integer(-int)))?;
        } else {
            return Err(VmError::IncompatibleTypes);
        }
        Ok(())
    }

    fn execute_bang_expression(&mut self) -> Result<(), VmError> {
        let right = self.pop()?;
        if let Object::Boolean(boolean) = &*right {
            let result = if *boolean { FALSE } else { TRUE };
            self.push(&Rc::new(result))?;
        } else {
            return Err(VmError::IncompatibleTypes);
        }
        Ok(())
    }

    fn execute_binary_expression(&mut self, op: OpCode) -> Result<(), VmError> {
        let right = self.pop()?;
        let left = self.pop()?;
        match (&*left, &op, &*right) {
            (Object::Integer(left_val), _, Object::Integer(right_val)) => {
                self.execute_integer_operation(*left_val, op, *right_val)?;
            }
            (Object::Boolean(left_val), OpCode::Equal, Object::Boolean(right_val)) => {
                let result = if left_val == right_val { TRUE } else { FALSE };
                self.push(&Rc::new(result))?;
            }
            (Object::Boolean(left_val), OpCode::NotEqual, Object::Boolean(right_val)) => {
                let result = if left_val != right_val { TRUE } else { FALSE };
                self.push(&Rc::new(result))?;
            }
            (Object::Boolean(left_val), OpCode::GreaterThan, Object::Boolean(right_val)) => {
                let result = if left_val > right_val { TRUE } else { FALSE };
                self.push(&Rc::new(result))?;
            }
            _ => return Err(VmError::IncompatibleTypes),
        }
        Ok(())
    }

    fn execute_integer_operation(
        &mut self,
        left: i64,
        op_code: OpCode,
        right: i64,
    ) -> Result<(), VmError> {
        let result = match op_code {
            OpCode::Add => Object::Integer(left + right),
            OpCode::Subtract => Object::Integer(left - right),
            OpCode::Multiply => Object::Integer(left * right),
            OpCode::Divide => Object::Integer(left / right),
            OpCode::Equal => {
                if left == right {
                    TRUE
                } else {
                    FALSE
                }
            }
            OpCode::NotEqual => {
                if left != right {
                    TRUE
                } else {
                    FALSE
                }
            }
            OpCode::GreaterThan => {
                if left > right {
                    TRUE
                } else {
                    FALSE
                }
            }
            _ => return Err(VmError::IncompatibleTypes),
        };
        self.push(&Rc::new(result))
    }

    fn is_truthy(object: &Object) -> bool {
        match object {
            Object::Null => false,
            Object::Integer(val) => *val != 0,
            Object::Boolean(val) => *val,
            _ => true,
        }
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
