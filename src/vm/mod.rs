use self::frame::Frame;
use crate::code::{read_u16, OpCode, WORD_SIZE};
use crate::compiler::ByteCode;
use crate::evaluator::object::{CompiledFunction, Hashable, Object};
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

pub mod frame;
mod tests;

const STACK_SIZE: usize = 2048; // 2KB
const MAX_FRAMES: usize = 1024; // 1KB
pub const GLOBAL_SIZE: usize = 65536;

const TRUE: Object = Object::Boolean(true);
const FALSE: Object = Object::Boolean(false);
const NULL: Object = Object::Null;

#[derive(Debug, PartialEq)]
pub struct VirtualMachine {
    constants: Vec<Rc<Object>>,
    stack: Vec<Rc<Object>>,
    pub globals: Vec<Rc<Object>>,
    frames: Vec<Frame>,
    frames_idx: usize,
}

impl VirtualMachine {
    pub fn new(bytecode: ByteCode) -> Self {
        let ByteCode(instructions, constants) = bytecode;
        VirtualMachine {
            constants,
            stack: Vec::with_capacity(STACK_SIZE),
            globals: vec![Rc::new(Object::Null); GLOBAL_SIZE],
            frames: vec![Frame::new(CompiledFunction::new(instructions))],
            frames_idx: 0,
        }
    }

    pub fn new_with_global_state(bytecode: ByteCode, globals: Vec<Rc<Object>>) -> VirtualMachine {
        let mut vm = VirtualMachine::new(bytecode);
        vm.globals = globals;
        vm
    }

    pub fn run(&mut self) -> Result<Rc<Object>, VmError> {
        let mut last_popped = None;
        let mut ip: usize;

        while self.frames[self.frames_idx].ip < self.frames[self.frames_idx].instructions().len() {
            ip = self.frames[self.frames_idx].ip;

            let word = &self.frames[self.frames_idx].instructions()[ip..ip + WORD_SIZE];

            let op = match OpCode::try_from(word[0]) {
                Ok(op_code) => op_code,
                Err(_) => return Err(VmError::UnknownOpCode),
            };

            match op {
                OpCode::Constant => {
                    let const_index = read_u16(&word[1..=2]);
                    let object = Rc::clone(&self.constants[const_index as usize]);
                    self.push(&object)?;
                }
                OpCode::Add
                | OpCode::Subtract
                | OpCode::Multiply
                | OpCode::Divide
                | OpCode::Equal
                | OpCode::NotEqual
                | OpCode::GreaterThan => {
                    self.execute_binary_expression(op)?;
                }
                OpCode::True => {
                    self.push(&Rc::new(TRUE))?;
                }
                OpCode::False => {
                    self.push(&Rc::new(FALSE))?;
                }
                OpCode::Minus => {
                    self.execute_minus_expression()?;
                }
                OpCode::Bang => {
                    self.execute_bang_expression()?;
                }
                OpCode::Pop => {
                    last_popped = Some(self.pop()?);
                }
                OpCode::Jump => {
                    let pos = read_u16(&word[1..=2]) as usize;
                    self.frames[self.frames_idx].ip = pos;
                    continue;
                }
                OpCode::JumpNotTruthy => {
                    let pos = read_u16(&word[1..=2]) as usize;
                    let condition = self.pop()?;
                    if !VirtualMachine::is_truthy(&*condition) {
                        self.frames[self.frames_idx].ip = pos;
                        continue;
                    }
                }
                OpCode::Null => {
                    self.push(&Rc::new(NULL))?;
                }
                OpCode::SetGlobal => {
                    let global_idx = read_u16(&word[1..=2]) as usize;
                    self.globals[global_idx] = self.pop()?;
                }
                OpCode::GetGlobal => {
                    let global_idx = read_u16(&word[1..=2]) as usize;
                    self.push(&self.globals[global_idx].clone())?;
                }
                OpCode::Array => {
                    let array_len = read_u16(&word[1..=2]) as usize;
                    let array = self.build_array(array_len)?;
                    self.push(&array)?;
                }
                OpCode::Hash => {
                    let hash_len = read_u16(&word[1..=2]) as usize;
                    let hash = self.build_hash(hash_len)?;
                    self.push(&hash)?;
                }
                OpCode::Index => {
                    self.execute_index_expression()?;
                }
                OpCode::Call => todo!(),
                OpCode::ReturnValue => todo!(),
                OpCode::Return => todo!(),
            }

            self.frames[self.frames_idx].ip += WORD_SIZE;
        }

        match last_popped {
            Some(obj) => Ok(obj),
            None => Err(VmError::EmptyStack),
        }
    }

    fn build_array(&mut self, length: usize) -> Result<Rc<Object>, VmError> {
        let mut elements = vec![Rc::new(Object::Null); length];
        for i in 1..=length {
            elements[length - i] = self.pop()?;
        }
        Ok(Rc::new(Object::Array(elements)))
    }

    fn build_hash(&mut self, length: usize) -> Result<Rc<Object>, VmError> {
        let mut table = HashMap::new();
        for _ in (0..length).step_by(2) {
            let val = self.pop()?;
            let key = match &*self.pop()? {
                Object::Integer(i) => Hashable::Integer(*i),
                Object::Boolean(b) => Hashable::Boolean(*b),
                Object::String(s) => Hashable::String(s.clone()),
                _ => {
                    return Err(VmError::UnhashableKey);
                }
            };
            table.insert(key, val);
        }
        Ok(Rc::new(Object::Hash(table)))
    }

    fn execute_index_expression(&mut self) -> Result<(), VmError> {
        let index = self.pop()?;
        let store = self.pop()?;

        match (&*store, &*index) {
            (Object::Array(array), Object::Integer(i)) => {
                if *i < 0 || *i as usize >= array.len() {
                    self.push(&Rc::new(NULL))
                } else {
                    self.push(&Rc::new(array[*i as usize].deref().clone()))
                }
            }
            (Object::Hash(table), index) => {
                let idx = match index {
                    Object::Integer(i) => Hashable::Integer(*i),
                    Object::Boolean(b) => Hashable::Boolean(*b),
                    Object::String(s) => Hashable::String(s.clone()),
                    _ => {
                        return Err(VmError::UnhashableKey);
                    }
                };

                match table.get(&idx) {
                    Some(val) => self.push(val),
                    None => self.push(&Rc::new(NULL)),
                }
            }
            _ => {
                return Err(VmError::IndexNotSupported);
            }
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
        let result = match &*right {
            Object::Boolean(val) => {
                if *val {
                    FALSE
                } else {
                    TRUE
                }
            }
            Object::Null => TRUE,
            _ => return Err(VmError::IncompatibleTypes),
        };

        self.push(&Rc::new(result))?;

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
            (Object::String(left_val), OpCode::Add, Object::String(right_val)) => {
                let result = Object::String(left_val.to_owned() + right_val);
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

    fn push_frame(&mut self, frame: Frame) -> Result<(), VmError> {
        if self.frames.len() == MAX_FRAMES {
            return Err(VmError::FrameStackOverflow);
        }
        self.frames.push(frame);
        self.frames_idx += 1;
        Ok(())
    }

    fn pop_frame(&mut self) -> Result<Frame, VmError> {
        if self.frames_idx > 0 {
            self.frames_idx -= 1;
        }
        match self.frames.pop() {
            Some(frame) => Ok(frame),
            None => Err(VmError::FrameStackUnderflow),
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
    UnhashableKey,
    IndexNotSupported,
    FrameStackUnderflow,
    FrameStackOverflow,
}
