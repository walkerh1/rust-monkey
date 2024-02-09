use std::rc::Rc;

use crate::{code::Instructions, object::Closure};

#[derive(Debug, PartialEq)]
pub struct Frame {
    pub closure: Closure,
    pub ip: usize,
    pub bp: usize,
}

impl Frame {
    pub fn new(closure: Closure, bp: usize) -> Self {
        Frame { closure, ip: 0, bp }
    }

    pub fn instructions(&self) -> &Rc<Instructions> {
        &self.closure.function.instructions
    }
}
