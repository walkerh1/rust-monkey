use std::rc::Rc;

use crate::{code::Instructions, evaluator::object::CompiledFunction};

#[derive(Debug, PartialEq)]
pub struct Frame {
    pub function: CompiledFunction,
    pub ip: usize,
    pub bp: usize,
}

impl Frame {
    pub fn new(function: CompiledFunction, bp: usize) -> Self {
        Frame {
            function,
            ip: 0,
            bp,
        }
    }

    pub fn instructions(&self) -> &Rc<Instructions> {
        &self.function.instructions
    }
}
