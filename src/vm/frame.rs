use std::rc::Rc;

use crate::{code::Instructions, evaluator::object::CompiledFunction};

#[derive(Debug, PartialEq)]
pub struct Frame {
    pub function: CompiledFunction,
    pub ip: usize,
}

impl Frame {
    pub fn new(function: CompiledFunction) -> Self {
        Frame { function, ip: 0 }
    }

    pub fn instructions(&self) -> &Rc<Instructions> {
        &self.function.instructions
    }
}
