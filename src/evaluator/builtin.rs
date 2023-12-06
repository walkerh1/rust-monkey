use crate::evaluator::object::Object;
use crate::evaluator::EvalError;
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Builtin {
    Len,
    First,
    Last,
    Rest,
    Push,
    Puts,
}

impl Builtin {
    pub fn get(id: &str) -> Option<Rc<Object>> {
        Some(match id {
            "len" => Rc::new(Object::Builtin(Builtin::Len)),
            "first" => Rc::new(Object::Builtin(Builtin::First)),
            "last" => Rc::new(Object::Builtin(Builtin::Last)),
            "rest" => Rc::new(Object::Builtin(Builtin::Rest)),
            "push" => Rc::new(Object::Builtin(Builtin::Push)),
            "puts" => Rc::new(Object::Builtin(Builtin::Puts)),
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
                let result = match &**args.first().unwrap() {
                    Object::String(string) => string.len() as i64,
                    Object::Array(array) => array.len() as i64,
                    _ => return Err(EvalError::IncompatibleTypes),
                };

                Rc::new(Object::Integer(result))
            }
            Builtin::First => {
                if args.len() != 1 {
                    return Err(EvalError::IncorrectNumberOfArgs);
                }

                if let Object::Array(array) = &**args.first().unwrap() {
                    match array.first() {
                        Some(element) => Rc::clone(element),
                        None => Rc::new(Object::Null),
                    }
                } else {
                    return Err(EvalError::IncompatibleTypes);
                }
            }
            Builtin::Last => {
                if args.len() != 1 {
                    return Err(EvalError::IncorrectNumberOfArgs);
                }

                if let Object::Array(array) = &**args.first().unwrap() {
                    match array.last() {
                        Some(object) => Rc::clone(object),
                        None => Rc::new(Object::Null),
                    }
                } else {
                    return Err(EvalError::IncompatibleTypes);
                }
            }
            Builtin::Rest => {
                if args.len() != 1 {
                    return Err(EvalError::IncorrectNumberOfArgs);
                }

                if let Object::Array(array) = &**args.first().unwrap() {
                    if array.is_empty() {
                        Rc::new(Object::Null)
                    } else {
                        Rc::new(Object::Array(array[1..].to_vec()))
                    }
                } else {
                    return Err(EvalError::IncompatibleTypes);
                }
            }
            Builtin::Push => {
                if args.len() != 2 {
                    return Err(EvalError::IncorrectNumberOfArgs);
                }

                if let Object::Array(array) = &**args.first().unwrap() {
                    let mut new_array = vec![];
                    array
                        .iter()
                        .for_each(|element| new_array.push(element.clone()));
                    let element = Rc::clone(&args[1].clone());
                    new_array.push(element);
                    Rc::new(Object::Array(new_array))
                } else {
                    return Err(EvalError::IncompatibleTypes);
                }
            }
            Builtin::Puts => {
                for arg in args {
                    println!("{arg}");
                }
                Rc::new(Object::Null)
            }
        })
    }
}
