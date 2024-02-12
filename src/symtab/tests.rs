#![cfg(test)]

use std::rc::Rc;

use crate::symtab::{Symbol, SymbolScope};

use super::SymbolTable;

#[test]
fn test_define() {
    let mut global = SymbolTable::new();
    let a = global.define("a".to_string());
    let b = global.define("b".to_string());

    assert_eq!(a, Rc::new(Symbol::new("a", SymbolScope::Global, 0)));
    assert_eq!(b, Rc::new(Symbol::new("b", SymbolScope::Global, 1)));

    let mut first_local = SymbolTable::new_enclosed(global.clone());
    let c = first_local.define("c".to_string());
    let d = first_local.define("d".to_string());

    assert_eq!(c, Rc::new(Symbol::new("c", SymbolScope::Local, 0)));
    assert_eq!(d, Rc::new(Symbol::new("d", SymbolScope::Local, 1)));

    let mut second_local = SymbolTable::new_enclosed(first_local.clone());
    let e = second_local.define("e".to_string());
    let f = second_local.define("f".to_string());

    assert_eq!(e, Rc::new(Symbol::new("e", SymbolScope::Local, 0)));
    assert_eq!(f, Rc::new(Symbol::new("f", SymbolScope::Local, 1)));
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

#[test]
fn test_resolve_local() {
    let mut global = SymbolTable::new();
    global.define("a".to_string());
    global.define("b".to_string());

    let a = global.resolve("a".to_string()).unwrap();
    assert_eq!(a, Rc::new(Symbol::new("a", SymbolScope::Global, 0)));

    let b = global.resolve("b".to_string()).unwrap();
    assert_eq!(b, Rc::new(Symbol::new("b", SymbolScope::Global, 1)));

    let mut local = SymbolTable::new_enclosed(global.clone());
    local.define("c".to_string());
    local.define("d".to_string());

    let a = local.resolve("a".to_string()).unwrap();
    assert_eq!(a, Rc::new(Symbol::new("a", SymbolScope::Global, 0)));

    let b = local.resolve("b".to_string()).unwrap();
    assert_eq!(b, Rc::new(Symbol::new("b", SymbolScope::Global, 1)));

    let c = local.resolve("c".to_string()).unwrap();
    assert_eq!(c, Rc::new(Symbol::new("c", SymbolScope::Local, 0)));

    let d = local.resolve("d".to_string()).unwrap();
    assert_eq!(d, Rc::new(Symbol::new("d", SymbolScope::Local, 1)));
}

#[test]
fn test_resolve_nested_local() {
    let mut global = SymbolTable::new();
    global.define("a".to_string());
    global.define("b".to_string());

    let mut first_local = SymbolTable::new_enclosed(global.clone());
    first_local.define("c".to_string());
    first_local.define("d".to_string());

    let mut second_local = SymbolTable::new_enclosed(first_local.clone());
    second_local.define("e".to_string());
    second_local.define("f".to_string());

    let a = second_local.resolve("a".to_string()).unwrap();
    assert_eq!(a, Rc::new(Symbol::new("a", SymbolScope::Global, 0)));

    let b = second_local.resolve("b".to_string()).unwrap();
    assert_eq!(b, Rc::new(Symbol::new("b", SymbolScope::Global, 1)));

    let e = second_local.resolve("e".to_string()).unwrap();
    assert_eq!(e, Rc::new(Symbol::new("e", SymbolScope::Local, 0)));

    let f = second_local.resolve("f".to_string()).unwrap();
    assert_eq!(f, Rc::new(Symbol::new("f", SymbolScope::Local, 1)));
}

#[test]
fn test_define_resolve_builtin() {
    let mut global = SymbolTable::new();
    global.define_builtin(0, "a".to_string());
    global.define_builtin(1, "b".to_string());

    let mut first_local = SymbolTable::new_enclosed(global.clone());
    let mut second_local = SymbolTable::new_enclosed(first_local.clone());

    let ag = global.resolve("a".to_string()).unwrap();
    assert_eq!(ag, Rc::new(Symbol::new("a", SymbolScope::Builtin, 0)));

    let al1 = first_local.resolve("a".to_string()).unwrap();
    assert_eq!(al1, Rc::new(Symbol::new("a", SymbolScope::Builtin, 0)));

    let al2 = second_local.resolve("a".to_string()).unwrap();
    assert_eq!(al2, Rc::new(Symbol::new("a", SymbolScope::Builtin, 0)));

    let bg = global.resolve("b".to_string()).unwrap();
    assert_eq!(bg, Rc::new(Symbol::new("b", SymbolScope::Builtin, 1)));

    let bl1 = first_local.resolve("b".to_string()).unwrap();
    assert_eq!(bl1, Rc::new(Symbol::new("b", SymbolScope::Builtin, 1)));

    let bl2 = second_local.resolve("b".to_string()).unwrap();
    assert_eq!(bl2, Rc::new(Symbol::new("b", SymbolScope::Builtin, 1)));
}

#[test]
fn test_resolve_free() {
    let mut global = SymbolTable::new();
    global.define("a".to_string());
    global.define("b".to_string());

    let mut first_local = SymbolTable::new_enclosed(global.clone());
    first_local.define("c".to_string());
    first_local.define("d".to_string());

    let mut second_local = SymbolTable::new_enclosed(first_local.clone());
    second_local.define("e".to_string());
    second_local.define("f".to_string());

    let ag = global.resolve("a".to_string()).unwrap();
    assert_eq!(ag, Rc::new(Symbol::new("a", SymbolScope::Global, 0)));

    let bg = global.resolve("b".to_string()).unwrap();
    assert_eq!(bg, Rc::new(Symbol::new("b", SymbolScope::Global, 1)));

    let al1 = first_local.resolve("a".to_string()).unwrap();
    assert_eq!(al1, Rc::new(Symbol::new("a", SymbolScope::Global, 0)));

    let bl1 = first_local.resolve("b".to_string()).unwrap();
    assert_eq!(bl1, Rc::new(Symbol::new("b", SymbolScope::Global, 1)));

    let cl1 = first_local.resolve("c".to_string()).unwrap();
    assert_eq!(cl1, Rc::new(Symbol::new("c", SymbolScope::Local, 0)));

    let dl1 = first_local.resolve("d".to_string()).unwrap();
    assert_eq!(dl1, Rc::new(Symbol::new("d", SymbolScope::Local, 1)));

    let al2 = second_local.resolve("a".to_string()).unwrap();
    assert_eq!(al2, Rc::new(Symbol::new("a", SymbolScope::Global, 0)));

    let bl2 = second_local.resolve("b".to_string()).unwrap();
    assert_eq!(bl2, Rc::new(Symbol::new("b", SymbolScope::Global, 1)));

    let cl2 = second_local.resolve("c".to_string()).unwrap();
    assert_eq!(cl2, Rc::new(Symbol::new("c", SymbolScope::Free, 0)));

    let dl2 = second_local.resolve("d".to_string()).unwrap();
    assert_eq!(dl2, Rc::new(Symbol::new("d", SymbolScope::Free, 1)));

    let el2 = second_local.resolve("e".to_string()).unwrap();
    assert_eq!(el2, Rc::new(Symbol::new("e", SymbolScope::Local, 0)));

    let fl2 = second_local.resolve("f".to_string()).unwrap();
    assert_eq!(fl2, Rc::new(Symbol::new("f", SymbolScope::Local, 1)));
}

#[test]
fn test_resolve_unresolvable_free() {
    let mut global = SymbolTable::new();
    global.define("a".to_string());

    let mut first_local = SymbolTable::new_enclosed(global.clone());
    first_local.define("c".to_string());

    let mut second_local = SymbolTable::new_enclosed(first_local.clone());
    second_local.define("e".to_string());
    second_local.define("f".to_string());

    let al2 = second_local.resolve("a".to_string()).unwrap();
    assert_eq!(al2, Rc::new(Symbol::new("a", SymbolScope::Global, 0)));

    let bl2 = second_local.resolve("b".to_string());
    assert_eq!(bl2, None);

    let cl2 = second_local.resolve("c".to_string()).unwrap();
    assert_eq!(cl2, Rc::new(Symbol::new("c", SymbolScope::Free, 0)));

    let dl2 = second_local.resolve("d".to_string());
    assert_eq!(dl2, None);

    let el2 = second_local.resolve("e".to_string()).unwrap();
    assert_eq!(el2, Rc::new(Symbol::new("e", SymbolScope::Local, 0)));

    let fl2 = second_local.resolve("f".to_string()).unwrap();
    assert_eq!(fl2, Rc::new(Symbol::new("f", SymbolScope::Local, 1)));
}
