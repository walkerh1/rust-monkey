#![cfg(test)]

use std::{collections::HashMap, rc::Rc};

use crate::symtab::{Symbol, SymbolScope};

use super::SymbolTable;

#[test]
fn test_define() {
    // construct expected symbol table
    let mut store: HashMap<String, Rc<Symbol>> = HashMap::new();
    store.insert(
        "a".to_string(),
        Rc::new(Symbol::new("a", SymbolScope::Global, 0)),
    );
    store.insert(
        "b".to_string(),
        Rc::new(Symbol::new("b", SymbolScope::Global, 1)),
    );
    let expected = SymbolTable {
        store,
        num_definitions: 2,
    };

    let mut global = SymbolTable::new();
    global.define("a".to_string());
    global.define("b".to_string());

    assert_eq!(global, expected);
}

#[test]
fn test_resolve() {
    let mut global = SymbolTable::new();
    global.define("a".to_string());
    global.define("b".to_string());

    let a = global.resolve("a".to_string()).unwrap();
    assert_eq!(a, Rc::new(Symbol::new("a", SymbolScope::Global, 0)));

    let b = global.resolve("b".to_string()).unwrap();
    assert_eq!(b, Rc::new(Symbol::new("b", SymbolScope::Global, 1)));
}
