use crate::compiler::Compiler;
use crate::object::{Hashable, Object};
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

#[test]
fn test_calling_function_with_no_args_one() {
    let input = "
let fivePlusTen = fn() { 5 + 10; };
fivePlusTen();
";
    let expected = Rc::new(Object::Integer(15));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_calling_function_with_no_args_two() {
    let input = "
let a = fn() { 1 };
let b = fn() { a() + 1 };
let c = fn() { b() + 1 };
c();
";
    let expected = Rc::new(Object::Integer(3));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_calling_function_with_no_args_three() {
    let input = "
let one = fn() { 1; };
let two = fn() { 2; };
one() + two();
";
    let expected = Rc::new(Object::Integer(3));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_calling_function_early_exit_one() {
    let input = "
let a = fn() { return 99; 100; };
a();
";
    let expected = Rc::new(Object::Integer(99));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_calling_function_early_exit_two() {
    let input = "
let a = fn() { return 99; return 100; };
a();
";
    let expected = Rc::new(Object::Integer(99));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_calling_function_without_return_value() {
    let input = "
let a = fn() { };
a();
";
    let expected = Rc::new(Object::Null);
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_calling_first_class_function() {
    let input = "
let a = fn() { 1 };
let b = fn() { a };
b()();
";
    let expected = Rc::new(Object::Integer(1));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_calling_functions_with_bindings_one() {
    let input = "
let one = fn() { let one = 1; one };
one();
";
    let expected = Rc::new(Object::Integer(1));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_calling_functions_with_bindings_two() {
    let input = "
let add = fn() { let one = 1; let two = 2; one + two };
add();
";
    let expected = Rc::new(Object::Integer(3));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_calling_functions_with_bindings_three() {
    let input = "
let addOneTwo = fn() { let one = 1; let two = 2; one + two };
let addThreeFour = fn() { let three = 3; let four = 4; three + four };
addOneTwo() + addThreeFour();
";
    let expected = Rc::new(Object::Integer(10));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_calling_functions_with_bindings_four() {
    let input = "
let fooOne = fn() { let foo = 50; foo; };
let fooTwo = fn() { let foo = 100; foo; };
fooOne() + fooTwo();
";
    let expected = Rc::new(Object::Integer(150));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_calling_functions_with_bindings_five() {
    let input = "
let globalSeed = 50;
let minusOne = fn() {
    let num = 1;
    globalSeed - num;
};
let minusTwo = fn() {
    let num = 2;
    globalSeed - num;
};
minusOne() + minusTwo();
";
    let expected = Rc::new(Object::Integer(97));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_first_class_function_with_locals() {
    let input = "
let returnsOneReturner = fn() {
    let returnsOne = fn() { 1; };
    returnsOne;
};
returnsOneReturner()();
";
    let expected = Rc::new(Object::Integer(1));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_calling_functions_with_args_and_no_bindings_one() {
    let input = "
let id = fn(a) { a };
id(1);
";
    let expected = Rc::new(Object::Integer(1));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_calling_functions_with_args_and_no_bindings_two() {
    let input = "
let sum = fn(a, b) { a + b; };
sum(1, 2);
";
    let expected = Rc::new(Object::Integer(3));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_calling_functions_with_args_and_bindings_one() {
    let input = "
let sum = fn(a, b) {
    let c = a + b;
    c;
};
sum(1, 2);
";
    let expected = Rc::new(Object::Integer(3));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_calling_functions_with_args_and_bindings_two() {
    let input = "
let sum = fn(a, b) {
    let c = a + b;
    c;
};
sum(1, 2) + sum(3, 4);
";
    let expected = Rc::new(Object::Integer(10));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_calling_functions_with_args_and_bindings_three() {
    let input = "
let sum = fn(a, b) {
    let c = a + b;
    c;
};
let outer = fn() {
    sum(1, 2) + sum(3, 4);
};
outer();
";
    let expected = Rc::new(Object::Integer(10));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_calling_functions_with_args_and_bindings_four() {
    let input = "
let globalNum = 10;
let sum = fn(a, b) {
    let c = a + b;
    c + globalNum;
};
let outer = fn() {
    sum(1, 2) + sum(3, 4) + globalNum;
};
outer() + globalNum;
";
    let expected = Rc::new(Object::Integer(50));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_calling_function_with_wrong_args_one() {
    let input = "fn() { 1; }(1);";
    let expected_error = VmError::WrongArguments;
    let (result, error) = compile_and_run(input);
    assert_eq!(error, Some(expected_error));
    assert_eq!(result, None);
}

#[test]
fn test_calling_function_with_wrong_args_two() {
    let input = "fn(a) { 1; }();";
    let expected_error = VmError::WrongArguments;
    let (result, error) = compile_and_run(input);
    assert_eq!(error, Some(expected_error));
    assert_eq!(result, None);
}

#[test]
fn test_calling_function_with_wrong_args_three() {
    let input = "fn(a, b) { 1; }(1);";
    let expected_error = VmError::WrongArguments;
    let (result, error) = compile_and_run(input);
    assert_eq!(error, Some(expected_error));
    assert_eq!(result, None);
}

#[test]
fn test_builtin_one() {
    let input = "len(\"\");";
    let expected = Rc::new(Object::Integer(0));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_builtin_two() {
    let input = "len(\"four\");";
    let expected = Rc::new(Object::Integer(4));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_builtin_three() {
    let input = "len(\"hello world\");";
    let expected = Rc::new(Object::Integer(11));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_builtin_four() {
    let input = "len(1);";
    let expected_error = VmError::IncompatibleTypes;
    let (result, error) = compile_and_run(input);
    assert_eq!(error, Some(expected_error));
    assert_eq!(result, None);
}

#[test]
fn test_builtin_five() {
    let input = "len(\"one\", \"two\");";
    let expected_error = VmError::WrongArguments;
    let (result, error) = compile_and_run(input);
    assert_eq!(error, Some(expected_error));
    assert_eq!(result, None);
}

#[test]
fn test_builtin_six() {
    let input = "len([1,2,3]);";
    let expected = Rc::new(Object::Integer(3));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_builtin_seven() {
    let input = "len([]);";
    let expected = Rc::new(Object::Integer(0));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_builtin_eight() {
    let input = "puts(\"hello\", \"world\");";
    let expected = Rc::new(Object::Null);
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_builtin_nine() {
    let input = "first([1,2,3]);";
    let expected = Rc::new(Object::Integer(1));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_builtin_ten() {
    let input = "first([]);";
    let expected = Rc::new(Object::Null);
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_builtin_eleven() {
    let input = "first(1);";
    let expected_error = VmError::IncompatibleTypes;
    let (result, error) = compile_and_run(input);
    assert_eq!(error, Some(expected_error));
    assert_eq!(result, None);
}

#[test]
fn test_builtin_twelve() {
    let input = "last([1,2,3]);";
    let expected = Rc::new(Object::Integer(3));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_builtin_thirteen() {
    let input = "last([]);";
    let expected = Rc::new(Object::Null);
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_builtin_fourteen() {
    let input = "last(1);";
    let expected_error = VmError::IncompatibleTypes;
    let (result, error) = compile_and_run(input);
    assert_eq!(error, Some(expected_error));
    assert_eq!(result, None);
}

#[test]
fn test_builtin_fifteen() {
    let input = "rest([1,2,3]);";
    let expected = Rc::new(Object::Array(vec![
        Rc::new(Object::Integer(2)),
        Rc::new(Object::Integer(3)),
    ]));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_builtin_sixteen() {
    let input = "rest([]);";
    let expected = Rc::new(Object::Null);
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_builtin_seventeen() {
    let input = "push([], 1);";
    let expected = Rc::new(Object::Array(vec![Rc::new(Object::Integer(1))]));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_builtin_eighteen() {
    let input = "push(1,1);";
    let expected_error = VmError::IncompatibleTypes;
    let (result, error) = compile_and_run(input);
    assert_eq!(error, Some(expected_error));
    assert_eq!(result, None);
}

#[test]
fn test_closure_one() {
    let input = "
let newClosure = fn(a) {
    fn() { a; };
};
let closure = newClosure(99);
closure();
";
    let expected = Rc::new(Object::Integer(99));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_closure_two() {
    let input = "
let newAdder = fn(a, b) {
    fn(c) { a + b + c; };
};
let adder = newAdder(1, 2);
adder(8);
";
    let expected = Rc::new(Object::Integer(11));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_closure_three() {
    let input = "
let newAdder = fn(a, b) {
    let c = a + b;
    fn(d) { c + d; };
};
let adder = newAdder(1, 2);
adder(8);
";
    let expected = Rc::new(Object::Integer(11));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_closure_four() {
    let input = "
let newAdder = fn(a, b) {
    let c = a + b;
    fn(d) {
        let e = d + c;
        fn(f) { e + f; };
    };
};
let adderInner = newAdder(1, 2);
let adder = adderInner(3);
adder(8);
";
    let expected = Rc::new(Object::Integer(14));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_closure_five() {
    let input = "
let newClosure = fn(a, b) {
    let one = fn() { a; };
    let two = fn() { b; };
    fn() { one() + two(); };
};
let closure = newClosure(9, 90);
closure();
";
    let expected = Rc::new(Object::Integer(99));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_recursive_closure_one() {
    let input = "
let countdown = fn(x) {
    if (x == 0) {
        return 0;
    } else {
        countdown(x-1);
    }
};
countdown(1);
";
    let expected = Rc::new(Object::Integer(0));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_recursive_closure_two() {
    let input = "
let countdown = fn(x) {
    if (x == 0) {
        return 0;
    } else {
        countdown(x-1);
    }
};
let wrapper = fn() {
    countdown(1);
};
wrapper();
";
    let expected = Rc::new(Object::Integer(0));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_recursive_closure_three() {
    let input = "
let wrapper = fn() {
    let countdown = fn(x) {
        if (x == 0) {
            return 0;
        } else {
            countdown(x-1);
        }
    };
    countdown(1);
};
wrapper();
";
    let expected = Rc::new(Object::Integer(0));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_logical_operator_one() {
    let input = "
true && false
";
    let expected = Rc::new(Object::Boolean(false));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_logical_operator_two() {
    let input = "
true && (1 > 0)
";
    let expected = Rc::new(Object::Boolean(true));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_logical_operator_three() {
    let input = "
false || (1 > 0)
";
    let expected = Rc::new(Object::Boolean(true));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}

#[test]
fn test_logical_operator_four() {
    let input = "
false || (1 < 0)
";
    let expected = Rc::new(Object::Boolean(false));
    let (result, error) = compile_and_run(input);
    assert_eq!(error, None);
    assert_eq!(result, Some(expected));
}
