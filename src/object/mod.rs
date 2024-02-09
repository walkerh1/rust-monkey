use crate::code::Instructions;
use crate::evaluator::environment::Environment;
use crate::parser::ast::Statement;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use self::builtins::Builtin;

pub mod builtins;

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    Null,
    Integer(i64),
    Boolean(bool),
    String(String),
    Return(Rc<Object>),
    Function(Function),
    Builtin(Builtin),
    Array(Vec<Rc<Object>>),
    Hash(HashMap<Hashable, Rc<Object>>),
    CompiledFunc(Rc<CompiledFunction>),
    Closure(Rc<Closure>),
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
                Object::Builtin(_) => "".to_string(),
                Object::Array(elements) => format!(
                    "[{}]",
                    elements
                        .iter()
                        .map(|element| element.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                Object::Hash(pairs) => format!(
                    "{{{}}}",
                    pairs
                        .iter()
                        .map(|(k, v)| format!("{}: {}", k, v))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                Object::CompiledFunc(_) => "".to_string(),
                Object::Closure(_) => "".to_string(),
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub parameters: Vec<String>,
    pub body: Statement,
    pub env: Rc<RefCell<Environment>>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Hashable {
    String(String),
    Integer(i64),
    Boolean(bool),
}

impl Display for Hashable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Hashable::String(key) => format!("\"{}\"", key),
                Hashable::Integer(key) => key.to_string(),
                Hashable::Boolean(key) => key.to_string(),
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompiledFunction {
    pub instructions: Rc<Instructions>,
    pub num_locals: u32,
    pub num_params: u32,
}

impl CompiledFunction {
    pub fn new(instructions: Instructions, num_locals: u32, num_params: u32) -> Self {
        CompiledFunction {
            instructions: Rc::new(instructions),
            num_locals,
            num_params,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Closure {
    pub function: Rc<CompiledFunction>,
    pub free: Vec<Rc<Object>>,
}

impl Closure {
    pub fn new(function: CompiledFunction) -> Self {
        Closure {
            function: Rc::new(function),
            free: vec![],
        }
    }
}
