use std::collections::HashMap;
use crate::evaluator::object::Object;

pub struct Environment {
    store: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            store: HashMap::new()
        }
    }

    pub fn get(&self, key: &str) -> Option<&Object> {
        self.store.get(key)
    }

    pub fn set(&mut self, key: &str, val: Object) {
        self.store.insert(key.to_string(), val);
    }
}