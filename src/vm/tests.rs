use crate::compiler::Compiler;
use crate::evaluator::object::Object;
use crate::parser::Parser;
use crate::vm::{VirtualMachine, VmError, STACK_SIZE};
use std::rc::Rc;

#[cfg(test)]

fn compile_and_run(input: &str) -> (Option<Rc<Object>>, Option<VmError>) {
    let mut result = None;
    let mut error = None;
    let ast = Parser::parse_program(input).expect("got a parsing error");
    let byte_code = Compiler::compile(ast).expect("got a compiler error");
    match VirtualMachine::run(byte_code) {
        Ok(object) => result = Some(object),
        Err(err) => error = Some(err),
    }
    (result, error)
}

#[test]
fn test_vm_integer_object() {
    let input = "1024";
    let expected = Rc::new(Object::Integer(1024));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_vm_stack_overflow_not_dependent_on_number_of_statements() {
    let input = "1024;".repeat(STACK_SIZE + 1);
    let expected = Rc::new(Object::Integer(1024));
    let (result, error) = compile_and_run(input.as_str());
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_vm_integer_arithmetic_one() {
    let input = "1 + 2";
    let expected = Rc::new(Object::Integer(3));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_vm_integer_arithmetic_two() {
    let input = "1 - 2";
    let expected = Rc::new(Object::Integer(-1));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_vm_integer_arithmetic_three() {
    let input = "2 * 3";
    let expected = Rc::new(Object::Integer(6));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_vm_integer_arithmetic_four() {
    let input = "4 / 2";
    let expected = Rc::new(Object::Integer(2));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_vm_integer_arithmetic_five() {
    let input = "25 / 5 * 2 - 5 + 20";
    let expected = Rc::new(Object::Integer(25));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_vm_integer_arithmetic_six() {
    let input = "3 * 4 + 5";
    let expected = Rc::new(Object::Integer(17));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_vm_integer_arithmetic_seven() {
    let input = "3 + 4 * 5";
    let expected = Rc::new(Object::Integer(23));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_vm_integer_arithmetic_eight() {
    let input = "(5 + 2) * 6";
    let expected = Rc::new(Object::Integer(42));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}
