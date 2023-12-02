use crate::evaluator::builtin::Builtin;
use crate::evaluator::environment::Environment;
use crate::parser::ast::Statement;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum Object {
    Null,
    Integer(i64),
    Boolean(bool),
    String(String),
    Return(Rc<Object>),
    Function(Function),
    BuiltIn(Builtin),
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Object::Null => "Null".to_string(),
                Object::Integer(int) => int.to_string(),
                Object::Boolean(bool) => bool.to_string(),
                Object::String(string) => string.to_string(),
                Object::Return(object) => object.to_string(),
                Object::Function(_) => "".to_string(),
                Object::BuiltIn(_) => "".to_string(),
            }
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub parameters: Vec<String>,
    pub body: Statement,
    pub env: Rc<RefCell<Environment>>,
}
