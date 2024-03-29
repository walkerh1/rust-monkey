use crate::object::Object;
use std::rc::Rc;

pub const NUM_BUILTINS: usize = 6;

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

    pub fn get_by_idx(id: usize) -> Option<Rc<Object>> {
        Some(match id {
            0 => Rc::new(Object::Builtin(Builtin::Len)),
            1 => Rc::new(Object::Builtin(Builtin::First)),
            2 => Rc::new(Object::Builtin(Builtin::Last)),
            3 => Rc::new(Object::Builtin(Builtin::Rest)),
            4 => Rc::new(Object::Builtin(Builtin::Push)),
            5 => Rc::new(Object::Builtin(Builtin::Puts)),
            _ => return None,
        })
    }

    pub fn apply(&self, args: &[Rc<Object>]) -> Result<Rc<Object>, BuiltinError> {
        Ok(match self {
            Builtin::Len => {
                if args.len() != 1 {
                    return Err(BuiltinError::IncorrectNumberOfArgs);
                }

                // safe to unwrap as the length of args is 1
                let result = match &**args.first().unwrap() {
                    Object::String(string) => string.len() as i64,
                    Object::Array(array) => array.len() as i64,
                    _ => return Err(BuiltinError::IncompatibleTypes),
                };

                Rc::new(Object::Integer(result))
            }
            Builtin::First => {
                if args.len() != 1 {
                    return Err(BuiltinError::IncorrectNumberOfArgs);
                }

                if let Object::Array(array) = &**args.first().unwrap() {
                    match array.first() {
                        Some(element) => Rc::clone(element),
                        None => Rc::new(Object::Null),
                    }
                } else {
                    return Err(BuiltinError::IncompatibleTypes);
                }
            }
            Builtin::Last => {
                if args.len() != 1 {
                    return Err(BuiltinError::IncorrectNumberOfArgs);
                }

                if let Object::Array(array) = &**args.first().unwrap() {
                    match array.last() {
                        Some(object) => Rc::clone(object),
                        None => Rc::new(Object::Null),
                    }
                } else {
                    return Err(BuiltinError::IncompatibleTypes);
                }
            }
            Builtin::Rest => {
                if args.len() != 1 {
                    return Err(BuiltinError::IncorrectNumberOfArgs);
                }

                if let Object::Array(array) = &**args.first().unwrap() {
                    if array.is_empty() {
                        Rc::new(Object::Null)
                    } else {
                        Rc::new(Object::Array(array[1..].to_vec()))
                    }
                } else {
                    return Err(BuiltinError::IncompatibleTypes);
                }
            }
            Builtin::Push => {
                if args.len() != 2 {
                    return Err(BuiltinError::IncorrectNumberOfArgs);
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
                    return Err(BuiltinError::IncompatibleTypes);
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

pub enum BuiltinError {
    IncompatibleTypes,
    IncorrectNumberOfArgs,
}
