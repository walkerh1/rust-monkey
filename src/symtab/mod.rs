use std::{collections::HashMap, rc::Rc};

mod tests;

#[derive(Debug, PartialEq)]
pub enum SymbolScope {
    Global,
    Local,
    Builtin,
    Free,
    Function,
}

#[derive(Debug, PartialEq)]
pub struct Symbol {
    name: String,
    pub scope: SymbolScope,
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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SymbolTable {
    pub outer: Option<Box<SymbolTable>>,
    store: HashMap<String, Rc<Symbol>>,
    pub num_definitions: u32,
    pub free_symbols: Vec<Rc<Symbol>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            outer: None,
            store: HashMap::new(),
            num_definitions: 0,
            free_symbols: vec![],
        }
    }

    pub fn new_enclosed(table: SymbolTable) -> Self {
        let mut new = Self::new();
        new.outer = Some(Box::new(table));
        new
    }

    pub fn define_builtin(&mut self, idx: u32, name: String) -> Rc<Symbol> {
        let symbol = Rc::new(Symbol::new(name.as_str(), SymbolScope::Builtin, idx));
        self.store.insert(name, Rc::clone(&symbol));
        symbol
    }

    pub fn define_all_builtins(&mut self) {
        self.define_builtin(0, "len".to_string());
        self.define_builtin(1, "first".to_string());
        self.define_builtin(2, "last".to_string());
        self.define_builtin(3, "rest".to_string());
        self.define_builtin(4, "push".to_string());
        self.define_builtin(5, "puts".to_string());
    }

    pub fn define_function_name(&mut self, name: String) -> Rc<Symbol> {
        let symbol = Rc::new(Symbol::new(name.as_str(), SymbolScope::Function, 0));
        self.store.insert(name, Rc::clone(&symbol));
        symbol
    }

    pub fn define(&mut self, name: String) -> Rc<Symbol> {
        let scope = match &self.outer {
            Some(_) => SymbolScope::Local,
            None => SymbolScope::Global,
        };
        let symbol = Rc::new(Symbol::new(name.as_str(), scope, self.num_definitions));
        self.store.insert(name, Rc::clone(&symbol));
        self.num_definitions += 1;
        symbol
    }

    pub fn resolve(&mut self, name: String) -> Option<Rc<Symbol>> {
        let symbol = self.store.get(&name).cloned();
        if let Some(sym) = symbol {
            return Some(Rc::clone(&sym));
        } else if let Some(outer) = &mut self.outer {
            if let Some(object) = outer.resolve(name) {
                match object.scope {
                    SymbolScope::Global | SymbolScope::Builtin => {
                        return Some(object);
                    }
                    _ => {
                        return Some(self.define_free(object));
                    }
                }
            }
        }
        None
    }

    fn define_free(&mut self, original: Rc<Symbol>) -> Rc<Symbol> {
        self.free_symbols.push(original.clone());
        let sym = Rc::new(Symbol::new(
            &original.name,
            SymbolScope::Free,
            (self.free_symbols.len() - 1) as u32,
        ));
        self.store.insert(original.name.clone(), Rc::clone(&sym));
        Rc::clone(&sym)
    }
}
