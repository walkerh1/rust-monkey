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
fn test_vm_stack_overflow() {
    let input = "1024;".repeat(STACK_SIZE + 1);
    let expected_error = VmError::StackOverflow;
    let (result, error) = compile_and_run(input.as_str());
    assert_eq!(error, Some(expected_error));
    assert_eq!(result, None);
}

#[test]
fn test_vm_integer_addition() {
    let input = "1 + 2";
    let expected = Rc::new(Object::Integer(3));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}
