use std::collections::HashMap;
use std::rc::Rc;
use crate::evaluator::object::Object;

pub struct Environment {
    store: HashMap<String, Rc<Object>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            store: HashMap::new()
        }
    }

    pub fn get(&self, key: &str) -> Option<Rc<Object>> {
        match self.store.get(key) {
            Some(object) => Some(Rc::clone(object)),
            None => None,
        }
    }

    pub fn set(&mut self, key: &str, val: Rc<Object>) {
        self.store.insert(key.to_string(), Rc::clone(&val));
    }
}