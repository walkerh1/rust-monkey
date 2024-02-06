use crate::object::Object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Environment {
    store: HashMap<String, Rc<Object>>,
    outer: Option<Rc<RefCell<Environment>>>,
}

#[allow(unused)]
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
        let result = match self.store.get(key) {
            Some(val) => Some(Rc::clone(val)),
            None => match &self.outer {
                Some(outer) => outer.borrow().get(key),
                None => None,
            },
        };

        result
    }

    pub fn set(&mut self, key: &str, val: Rc<Object>) {
        if self.store.get(key).is_some() {
            self.store.insert(key.to_string(), Rc::clone(&val));
            return;
        } else if let Some(outer) = &self.outer {
            if outer.borrow().get(key).is_some() {
                outer.borrow_mut().set(key, Rc::clone(&val));
                return;
            }
        }
        self.store.insert(key.to_string(), Rc::clone(&val));
    }
}
