#![cfg(test)]

use crate::evaluator::environment::Environment;
use crate::evaluator::{eval, EvalError};
use crate::object::{Function, Hashable, Object};
use crate::parser::ast::{Expression, Infix, Statement};
use crate::parser::Parser;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

fn parse_and_eval(input: &str) -> Result<Rc<Object>, EvalError> {
    // PRE: `input` is a well-formed (i.e. parsable) program
    let program = Parser::parse_program(input).unwrap();
    let env = Rc::new(RefCell::new(Environment::new()));
    eval(program, env)
}

#[test]
fn test_eval_for_integer_expression() {
    let input = "5";
    let expected = Rc::new(Object::Integer(5));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_for_boolean_expression() {
    let input = "true";
    let expected = Rc::new(Object::Boolean(true));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_bang_operator_one() {
    let input = "!true";
    let expected = Rc::new(Object::Boolean(false));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_bang_operator_two() {
    let input = "!false";
    let expected = Rc::new(Object::Boolean(true));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_bang_operator_three() {
    let input = "!5";
    let expected = Rc::new(Object::Boolean(false));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_bang_operator_four() {
    let input = "!!true";
    let expected = Rc::new(Object::Boolean(true));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_bang_operator_five() {
    let input = "!!false";
    let expected = Rc::new(Object::Boolean(false));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_bang_operator_six() {
    let input = "!!5";
    let expected = Rc::new(Object::Boolean(true));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_bang_operator_seven() {
    let input = "!0";
    let expected = Rc::new(Object::Boolean(true));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_minus_operator_one() {
    let input = "-5";
    let expected = Rc::new(Object::Integer(-5));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_minus_operator_two() {
    let input = "-10";
    let expected = Rc::new(Object::Integer(-10));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_minus_operator_three() {
    let input = "--10";
    let expected = Rc::new(Object::Integer(10));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_minus_operator_error_if_not_integer() {
    let input = "-true";
    let expected_error = EvalError::UnknownOperator;
    let error = parse_and_eval(input).err().unwrap();
    assert_eq!(error, expected_error);
}

#[test]
fn test_eval_infix_operators_one() {
    let input = "5 + 5 + 5 + 5 - 10";
    let expected = Rc::new(Object::Integer(10));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_infix_operators_two() {
    let input = "2 * 2 * 2 * 2 * 2";
    let expected = Rc::new(Object::Integer(32));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_infix_operators_three() {
    let input = "5 + 2 * 10";
    let expected = Rc::new(Object::Integer(25));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_infix_operators_four() {
    let input = "(5 + 2) * 10";
    let expected = Rc::new(Object::Integer(70));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_infix_operators_five() {
    let input = "15 - 5 * -4";
    let expected = Rc::new(Object::Integer(35));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_infix_operators_six() {
    let input = "(5 + 10 * 2 + 10 / 2) * 4 + -2";
    let expected = Rc::new(Object::Integer(118));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_infix_error_if_boolean_passed_to_arithmetic_operator() {
    let input = "2 + true";
    let expected_error = EvalError::IncompatibleTypes;
    let error = parse_and_eval(input).err().unwrap();
    assert_eq!(error, expected_error);
}

#[test]
fn test_eval_infix_error_if_integer_compared_to_boolean() {
    let input = "2 == true";
    let expected_error = EvalError::IncompatibleTypes;
    let error = parse_and_eval(input).err().unwrap();
    assert_eq!(error, expected_error);
}

#[test]
fn test_eval_infix_error_if_invalid_infix_with_bools() {
    let input = "true + false";
    let expected_error = EvalError::UnknownOperator;
    let error = parse_and_eval(input).err().unwrap();
    assert_eq!(error, expected_error);
}

#[test]
fn test_eval_if_expression_one() {
    let input = "if (true) { 10 }";
    let expected = Rc::new(Object::Integer(10));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_if_expression_two() {
    let input = "if (false) { 10 }";
    let expected = Rc::new(Object::Null);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_if_expression_three() {
    let input = "if (1) { 10 }";
    let expected = Rc::new(Object::Integer(10));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_if_expression_four() {
    let input = "if (1 < 2) { 10 }";
    let expected = Rc::new(Object::Integer(10));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_if_expression_five() {
    let input = "if (1 < 2) { 10 } else { 20 }";
    let expected = Rc::new(Object::Integer(10));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_if_expression_six() {
    let input = "if (1 > 2) { 10 } else { 20 }";
    let expected = Rc::new(Object::Integer(20));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_if_expression_seven() {
    let input = "if (1 > 2) { let x = 10; }";
    let expected = Rc::new(Object::Null);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_return_statement_one() {
    let input = "return 10;";
    let expected = Rc::new(Object::Integer(10));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_return_statement_two() {
    let input = "return 10; 9";
    let expected = Rc::new(Object::Integer(10));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_return_statement_three() {
    let input = "2 + 5; return 10; 9";
    let expected = Rc::new(Object::Integer(10));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_return_statement_four() {
    let input = "
if (10 > 1) {
    if (10 > 1) {
        return 10;
        return 9;
    }
    return 1;
}";
    let expected = Rc::new(Object::Integer(10));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_return_statement_five() {
    let input = "
if (10 > 1) {
    if (10 > 1) {
        if (10 > 1) {
            return 10;
        }
    }
    return 1;
}";
    let expected = Rc::new(Object::Integer(10));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_let_statement_binding_one() {
    let input = "let a = 5; a;";
    let expected = Rc::new(Object::Integer(5));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_let_statement_binding_two() {
    let input = "let a = 2 * 5; a;";
    let expected = Rc::new(Object::Integer(10));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_let_statement_binding_three() {
    let input = "let a = 5; let b = a; b;";
    let expected = Rc::new(Object::Integer(5));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_let_statement_binding_four() {
    let input = "let a = 5; let b = a; let c = a + b; c;";
    let expected = Rc::new(Object::Integer(10));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_let_statement_error_if_identifier_unbound() {
    let input = "foo";
    let expected_error = EvalError::UnrecognisedIdentifier;
    let error = parse_and_eval(input).err().unwrap();
    assert_eq!(error, expected_error);
}

#[test]
fn test_eval_function_definition() {
    let input = "fn(x) { x + 2 }";
    let expected = Rc::new(Object::Function(Function {
        parameters: vec![String::from("x")],
        body: Statement::BlockStatement(vec![Statement::Expression(Expression::Infix(
            Box::new(Expression::Identifier(String::from("x"))),
            Infix::Plus,
            Box::new(Expression::Integer(2)),
        ))]),
        env: Rc::new(RefCell::new(Environment::new())),
    }));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_function_call_one() {
    let input = "let identity = fn(x) { x }; identity(5)";
    let expected = Rc::new(Object::Integer(5));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_function_call_two() {
    let input = "let identity = fn(x) { return x; }; identity(5)";
    let expected = Rc::new(Object::Integer(5));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_function_call_three() {
    let input = "let double = fn(x) { x * 2; }; double(5)";
    let expected = Rc::new(Object::Integer(10));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_function_call_four() {
    let input = "let add = fn(x, y) { x + y; }; add(5, 5)";
    let expected = Rc::new(Object::Integer(10));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_function_call_five() {
    let input = "let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));";
    let expected = Rc::new(Object::Integer(20));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_function_call_six() {
    let input = "fn(x) { x; }(5)";
    let expected = Rc::new(Object::Integer(5));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_function_call_seven() {
    let input = "fn() { 5 }()";
    let expected = Rc::new(Object::Integer(5));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_function_closure_one() {
    let input = "let x = 4;
let addFour = fn(i) { x + i };
addFour(5)";
    let expected = Rc::new(Object::Integer(9));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_function_closure_two() {
    let input = "
let newAdder = fn(x) { fn(y) { x + y } };
let addTwo = newAdder(2);
addTwo(2)
";
    let expected = Rc::new(Object::Integer(4));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_function_closure_three() {
    let input = "
let multiply = fn(x, y) { x * y };
let applyFunc = fn(x, y, func) { func(x, y) };
applyFunc(3, 14, multiply)
";
    let expected = Rc::new(Object::Integer(42));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_function_with_return_statement() {
    let input = "let addOne = fn(x) { return x + 1; };
let y = addOne(5);
return y + 1;";
    let expected = Rc::new(Object::Integer(7));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_recursive_function() {
    let input = "
let counter = fn(x) {
    if (x > 100) {
        return true;
    } else {
        let foobar = 9999;
        counter(x + 1);
    }
};
counter(0);
";
    let expected = Rc::new(Object::Boolean(true));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_string_expression() {
    let input = "\"hello world\"";
    let expected = Rc::new(Object::String(String::from("hello world")));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_string_concatenation() {
    let input = "\"hello\" + \"world\"";
    let expected = Rc::new(Object::String(String::from("helloworld")));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_string_concatenation_error_if_unsupported_infix() {
    let input = "\"hello\" * \"world\"";
    let expected_error = EvalError::UnknownOperator;
    let error = parse_and_eval(input).err().unwrap();
    assert_eq!(error, expected_error);
}

#[test]
fn test_eval_builtin_len_function() {
    let input = "len(\"hello world\")";
    let expected = Rc::new(Object::Integer(11));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_builtin_len_error_if_too_many_args() {
    let input = "len(\"hello\", \"world\")";
    let expected_error = EvalError::IncorrectNumberOfArgs;
    let error = parse_and_eval(input).err().unwrap();
    assert_eq!(error, expected_error);
}

#[test]
fn test_eval_array_literal() {
    let input = "
let a = 4;
[1, a, 1 + 1, 2 * 3]
";
    let expected = Rc::new(Object::Array(vec![
        Rc::new(Object::Integer(1)),
        Rc::new(Object::Integer(4)),
        Rc::new(Object::Integer(2)),
        Rc::new(Object::Integer(6)),
    ]));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_indexing_into_array_one() {
    let input = "[1, 2][0]";
    let expected = Rc::new(Object::Integer(1));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_indexing_into_array_two() {
    let input = "[1, 2][1]";
    let expected = Rc::new(Object::Integer(2));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_indexing_into_array_three() {
    let input = "let i = 0; [10][i]";
    let expected = Rc::new(Object::Integer(10));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_indexing_into_array_four() {
    let input = "let arr = [1, 2, 3]; arr[2]";
    let expected = Rc::new(Object::Integer(3));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_indexing_into_array_five() {
    let input = "let arr = [1, 2, 3]; arr[0] + arr[1] + arr[2]";
    let expected = Rc::new(Object::Integer(6));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_index_out_of_bounds_one() {
    let input = "[1, 2, 3][3]";
    let expected_error = EvalError::IndexOutOfBounds;
    let error = parse_and_eval(input).err().unwrap();
    assert_eq!(error, expected_error);
}

#[test]
fn test_eval_index_out_of_bounds_two() {
    let input = "[1, 2, 3][-1]";
    let expected_error = EvalError::IndexOutOfBounds;
    let error = parse_and_eval(input).err().unwrap();
    assert_eq!(error, expected_error);
}

#[test]
fn test_eval_builtin_len_for_array() {
    let input = "len([1, 2, 3])";
    let expected = Rc::new(Object::Integer(3));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_builtin_first() {
    let input = "first([1, 2, 3])";
    let expected = Rc::new(Object::Integer(1));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_builtin_first_on_empty() {
    let input = "first([])";
    let expected = Rc::new(Object::Null);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_builtin_error_if_not_array() {
    let input = "first(\"hello\")";
    let expected_error = EvalError::IncompatibleTypes;
    let error = parse_and_eval(input).err().unwrap();
    assert_eq!(error, expected_error);
}

#[test]
fn test_eval_builtin_last() {
    let input = "last([1, 2, 3])";
    let expected = Rc::new(Object::Integer(3));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_builtin_last_on_empty() {
    let input = "last([])";
    let expected = Rc::new(Object::Null);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_builtin_last_if_not_array() {
    let input = "last(\"hello\")";
    let expected_error = EvalError::IncompatibleTypes;
    let error = parse_and_eval(input).err().unwrap();
    assert_eq!(error, expected_error);
}

#[test]
fn test_eval_builtin_rest() {
    let input = "rest([1, 2, 3])";
    let expected = Rc::new(Object::Array(vec![
        Rc::new(Object::Integer(2)),
        Rc::new(Object::Integer(3)),
    ]));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_builtin_rest_on_empty() {
    let input = "rest([])";
    let expected = Rc::new(Object::Null);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_builtin_rest_if_not_array() {
    let input = "rest(\"hello\")";
    let expected_error = EvalError::IncompatibleTypes;
    let error = parse_and_eval(input).err().unwrap();
    assert_eq!(error, expected_error);
}

#[test]
fn test_eval_builtin_push() {
    let input = "push([1, 2, 3], 4)";
    let expected = Rc::new(Object::Array(vec![
        Rc::new(Object::Integer(1)),
        Rc::new(Object::Integer(2)),
        Rc::new(Object::Integer(3)),
        Rc::new(Object::Integer(4)),
    ]));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_builtin_push_on_empty() {
    let input = "push([], 1)";
    let expected = Rc::new(Object::Array(vec![Rc::new(Object::Integer(1))]));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_builtin_push_error_if_not_array() {
    let input = "push(\"hello\", 1)";
    let expected_error = EvalError::IncompatibleTypes;
    let error = parse_and_eval(input).err().unwrap();
    assert_eq!(error, expected_error);
}

#[test]
fn test_eval_builtin_push_error_if_wrong_number_of_args() {
    let input = "push([1, 2, 3])";
    let expected_error = EvalError::IncorrectNumberOfArgs;
    let error = parse_and_eval(input).err().unwrap();
    assert_eq!(error, expected_error);
}

#[test]
fn test_eval_hash_literal() {
    let input = "
let two = \"two\";
{
    \"one\": 10 - 9,
    two: 1 + 1,
    \"three\": 3,
    2 + 2: 4,
    true: 5,
    false: 6
}";
    let expected = Rc::new(Object::Hash(HashMap::from([
        (
            Hashable::String(String::from("one")),
            Rc::new(Object::Integer(1)),
        ),
        (
            Hashable::String(String::from("two")),
            Rc::new(Object::Integer(2)),
        ),
        (
            Hashable::String(String::from("three")),
            Rc::new(Object::Integer(3)),
        ),
        (Hashable::Integer(4), Rc::new(Object::Integer(4))),
        (Hashable::Boolean(true), Rc::new(Object::Integer(5))),
        (Hashable::Boolean(false), Rc::new(Object::Integer(6))),
    ])));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_hash_index_expression_one() {
    let input = "{\"one\": 1}[\"one\"]";
    let expected = Rc::new(Object::Integer(1));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_hash_index_expression_two() {
    let input = "{1 + 1: 1}[2]";
    let expected = Rc::new(Object::Integer(1));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_hash_index_expression_three() {
    let input = "{true: 1}[true]";
    let expected = Rc::new(Object::Integer(1));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_hash_index_expression_four() {
    let input = "let map = {\"one\": 1}; map[\"one\"]";
    let expected = Rc::new(Object::Integer(1));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_hash_index_expression_five() {
    let input = "{0: 1}[3]";
    let expected = Rc::new(Object::Null);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_or_operator() {
    let input = "false || true";
    let expected = Rc::new(Object::Boolean(true));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_and_operator() {
    let input = "false && true";
    let expected = Rc::new(Object::Boolean(false));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_or_operator_with_truthy_and_falsy_values() {
    let input = "1 || 0";
    let expected = Rc::new(Object::Boolean(true));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_and_operator_with_truthy_and_falsy_values() {
    let input = "1 && 0";
    let expected = Rc::new(Object::Boolean(false));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_while_loop() {
    let input = "
let i = 0;
while (i < 3) {
    let i = i + 1;
}
return i;
";
    let expected = Rc::new(Object::Integer(3));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_while_loop_early_return() {
    let input = "
let i = 0;
while (true) {
    let i = i + 1;
    if (i == 3) {
        return i;
    }
}
";
    let expected = Rc::new(Object::Integer(3));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_assignment_expression() {
    let input = "
let i = 0;
i = i + 1;
i
";
    let expected = Rc::new(Object::Integer(1));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_assignment_expression_closure() {
    let input = "
let count = 0;
let counter = fn() { count = count + 1; count };
counter();
counter();
";
    let expected = Rc::new(Object::Integer(2));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_assignment_expression_closure_and_hof() {
    let input = "
let makeCounter = fn() { let count = 0; fn() { count = count + 1; count } };
let counter = makeCounter();
counter();
counter();
counter();
";
    let expected = Rc::new(Object::Integer(3));
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}
