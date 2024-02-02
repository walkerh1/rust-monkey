use crate::compiler::Compiler;
use crate::evaluator::object::{Hashable, Object};
use crate::parser::Parser;
use crate::vm::{VirtualMachine, VmError, STACK_SIZE};
use std::collections::HashMap;
use std::rc::Rc;

#[cfg(test)]

fn compile_and_run(input: &str) -> (Option<Rc<Object>>, Option<VmError>) {
    let mut result = None;
    let mut error = None;
    let ast = Parser::parse_program(input).expect("got a parsing error");
    let mut compiler = Compiler::new();
    let byte_code = compiler.compile(ast).expect("got a compiler error");
    let mut vm = VirtualMachine::new(byte_code);
    match vm.run() {
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

#[test]
fn test_vm_boolean_true() {
    let input = "true";
    let expected = Rc::new(Object::Boolean(true));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_vm_boolean_false() {
    let input = "false";
    let expected = Rc::new(Object::Boolean(false));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_vm_boolean_expression_one() {
    let input = "1 < 2";
    let expected = Rc::new(Object::Boolean(true));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_vm_boolean_expression_two() {
    let input = "1 > 2";
    let expected = Rc::new(Object::Boolean(false));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_vm_boolean_expression_three() {
    let input = "1 == 1";
    let expected = Rc::new(Object::Boolean(true));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_vm_boolean_expression_four() {
    let input = "1 != 1";
    let expected = Rc::new(Object::Boolean(false));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_vm_boolean_expression_five() {
    let input = "true == true";
    let expected = Rc::new(Object::Boolean(true));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_vm_boolean_expression_six() {
    let input = "true == false";
    let expected = Rc::new(Object::Boolean(false));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_vm_boolean_expression_seven() {
    let input = "true != false";
    let expected = Rc::new(Object::Boolean(true));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_vm_boolean_expression_eight() {
    let input = "1 < 2 == true";
    let expected = Rc::new(Object::Boolean(true));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_vm_minus_expression() {
    let input = "-10";
    let expected = Rc::new(Object::Integer(-10));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_vm_minus_in_infix_expression() {
    let input = "2 + -10";
    let expected = Rc::new(Object::Integer(-8));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_vm_bang_expression() {
    let input = "!true";
    let expected = Rc::new(Object::Boolean(false));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_vm_bang_in_infix_expression() {
    let input = "!true == false";
    let expected = Rc::new(Object::Boolean(true));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_vm_bang_expression_error_if_used_on_integer() {
    let input = "!5";
    let expected_error = VmError::IncompatibleTypes;
    let (result, error) = compile_and_run(input);
    assert_eq!(error, Some(expected_error));
    assert_eq!(result, None);
}

#[test]
fn test_vm_minus_expression_error_if_used_on_boolean() {
    let input = "-true";
    let expected_error = VmError::IncompatibleTypes;
    let (result, error) = compile_and_run(input);
    assert_eq!(error, Some(expected_error));
    assert_eq!(result, None);
}

#[test]
fn test_conditional_one() {
    let input = "if (true) { 10 }";
    let expected = Rc::new(Object::Integer(10));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_conditional_two() {
    let input = "if (true) { 10 } else { 20 }";
    let expected = Rc::new(Object::Integer(10));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_conditional_three() {
    let input = "if (false) { 10 } else { 20 }";
    let expected = Rc::new(Object::Integer(20));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_conditional_four() {
    let input = "if (1) { 10 }";
    let expected = Rc::new(Object::Integer(10));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_conditional_five() {
    let input = "if (1 < 2) { 10 }";
    let expected = Rc::new(Object::Integer(10));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_conditional_six() {
    let input = "if (1 > 2) { 10 } else { 20 }";
    let expected = Rc::new(Object::Integer(20));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_conditional_seven() {
    let input = "if (false) { 10 }";
    let expected = Rc::new(Object::Null);
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_conditional_eight() {
    let input = "!(if (false) { 10 })";
    let expected = Rc::new(Object::Boolean(true));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_global_let_statement_one() {
    let input = "let one = 1; one";
    let expected = Rc::new(Object::Integer(1));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_global_let_statement_two() {
    let input = "let one = 1; let two = 2; one + two";
    let expected = Rc::new(Object::Integer(3));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_global_let_statement_three() {
    let input = "let one = 1; let two = one + one; one + two";
    let expected = Rc::new(Object::Integer(3));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_string_expression_one() {
    let input = "\"monkey\"";
    let expected = Rc::new(Object::String("monkey".to_string()));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_string_expression_two() {
    let input = "\"mon\" + \"key\"";
    let expected = Rc::new(Object::String("monkey".to_string()));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_string_expression_three() {
    let input = "\"mon\" + \"key\" + \"banana\"";
    let expected = Rc::new(Object::String("monkeybanana".to_string()));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_array_expression_one() {
    let input = "[]";
    let expected = Rc::new(Object::Array(vec![]));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_array_expression_two() {
    let input = "[1, 2, 3]";
    let expected = Rc::new(Object::Array(vec![
        Rc::new(Object::Integer(1)),
        Rc::new(Object::Integer(2)),
        Rc::new(Object::Integer(3)),
    ]));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_array_expression_three() {
    let input = "[1 + 2, 3 - 4, 5 * 6]";
    let expected = Rc::new(Object::Array(vec![
        Rc::new(Object::Integer(3)),
        Rc::new(Object::Integer(-1)),
        Rc::new(Object::Integer(30)),
    ]));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_hash_literal_one() {
    let input = "{}";
    let expected = Rc::new(Object::Hash(HashMap::new()));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_hash_literal_two() {
    let input = "{1: 2, 3: 4}";
    let expected = Rc::new(Object::Hash(HashMap::from([
        (Hashable::Integer(1), Rc::new(Object::Integer(2))),
        (Hashable::Integer(3), Rc::new(Object::Integer(4))),
    ])));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_hash_literal_three() {
    let input = "{1 + 1: 2 * 2, 4 - 3: 12 / 4}";
    let expected = Rc::new(Object::Hash(HashMap::from([
        (Hashable::Integer(2), Rc::new(Object::Integer(4))),
        (Hashable::Integer(1), Rc::new(Object::Integer(3))),
    ])));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_index_expression_one() {
    let input = "[1, 2, 3][1]";
    let expected = Rc::new(Object::Integer(2));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_index_expression_two() {
    let input = "[1, 2, 3][1 + 1]";
    let expected = Rc::new(Object::Integer(3));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_index_expression_three() {
    let input = "[[1, 2, 3]][0][0]";
    let expected = Rc::new(Object::Integer(1));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_index_expression_four() {
    let input = "[][0]";
    let expected = Rc::new(Object::Null);
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_index_expression_five() {
    let input = "[1, 2][40]";
    let expected = Rc::new(Object::Null);
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_index_expression_six() {
    let input = "[1, 2][-1]";
    let expected = Rc::new(Object::Null);
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_index_expression_seven() {
    let input = "{1: 2}[0]";
    let expected = Rc::new(Object::Null);
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_index_expression_eight() {
    let input = "{}[0]";
    let expected = Rc::new(Object::Null);
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_index_expression_nine() {
    let input = "{1: 1, 2: 2}[1]";
    let expected = Rc::new(Object::Integer(1));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_index_expression_ten() {
    let input = "{1: 1, 2: 2}[2]";
    let expected = Rc::new(Object::Integer(2));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}
