use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    Null,
    Integer(i64),
    Boolean(bool),
    Return(Rc<Object>),
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
                Object::Return(object) => object.to_string(),
            }
        )
    }
}
