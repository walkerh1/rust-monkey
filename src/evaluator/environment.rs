use crate::evaluator::object::Object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Environment {
    store: HashMap<String, Rc<Object>>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>> {
        Rc::new(RefCell::new(Environment {
            store: HashMap::new(),
            outer: Some(outer),
        }))
    }

    pub fn get(&self, key: &str) -> Option<Rc<Object>> {
        // checking `store` before `outer` means a variable in the inner scope with
        // the same name as a variable in the outer scope will SHADOW that variable
        // in the outer scope.
        match self.store.get(key) {
            Some(val) => Some(Rc::clone(val)),
            None => match &self.outer {
                Some(outer) => outer.borrow().get(key),
                None => None,
            },
        }
    }

    pub fn set(&mut self, key: &str, val: Rc<Object>) {
        self.store.insert(key.to_string(), Rc::clone(&val));
    }
}
