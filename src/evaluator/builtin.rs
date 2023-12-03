use crate::evaluator::object::Object;
use crate::evaluator::EvalError;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum Builtin {
    Len,
    First,
}

impl Builtin {
    pub fn get(id: &str) -> Option<Rc<Object>> {
        Some(match id {
            "len" => Rc::new(Object::BuiltIn(Builtin::Len)),
            "first" => Rc::new(Object::BuiltIn(Builtin::First)),
            _ => return None,
        })
    }

    pub fn apply(&self, args: &[Rc<Object>]) -> Result<Rc<Object>, EvalError> {
        Ok(match self {
            Builtin::Len => {
                if args.len() != 1 {
                    return Err(EvalError::IncorrectNumberOfArgs);
                }

                // safe to unwrap as the length of args is 1
                let result = match &**args.get(0).unwrap() {
                    Object::String(string) => string.len() as i64,
                    Object::Array(array) => array.len() as i64,
                    _ => return Err(EvalError::IncompatibleTypes),
                };

                Rc::new(Object::Integer(result))
            }
            _ => return Err(EvalError::NotAFunction),
        })
    }
}
