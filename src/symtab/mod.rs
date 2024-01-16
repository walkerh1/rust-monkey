use std::{collections::HashMap, rc::Rc};

mod tests;

#[derive(Debug, PartialEq)]
pub enum SymbolScope {
    Global,
}

#[derive(Debug, PartialEq)]
pub struct Symbol {
    name: String,
    scope: SymbolScope,
    pub index: u32,
}

impl Symbol {
    pub fn new(name: &str, scope: SymbolScope, index: u32) -> Self {
        Symbol {
            name: name.to_string(),
            scope,
            index,
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct SymbolTable {
    store: HashMap<String, Rc<Symbol>>,
    num_definitions: u32,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            store: HashMap::new(),
            num_definitions: 0,
        }
    }

    pub fn define(&mut self, name: String) -> Rc<Symbol> {
        let symbol = Rc::new(Symbol::new(
            name.as_str(),
            SymbolScope::Global,
            self.num_definitions,
        ));
        self.store.insert(name, Rc::clone(&symbol));
        self.num_definitions += 1;
        symbol
    }

    pub fn resolve(&mut self, name: String) -> Option<Rc<Symbol>> {
        self.store.get(&name).cloned()
    }
}
