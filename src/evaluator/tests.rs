#![cfg(test)]

use crate::evaluator::object::Object;
use crate::evaluator::{eval, EvalError};
use crate::evaluator::environment::Environment;
use crate::parser::Parser;

fn parse_and_eval(input: &str) -> Result<Object, EvalError> {
    // assume only parsabale strings are provided
    let program = Parser::parse_program(input).unwrap();
    let mut env = Environment::new();
    eval(program, &mut env)
}

#[test]
fn test_eval_for_integer_expression() {
    let input = "5";
    let expected = Object::Integer(5);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_for_boolean_expression() {
    let input = "true";
    let expected = Object::Boolean(true);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_bang_operator_one() {
    let input = "!true";
    let expected = Object::Boolean(false);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_bang_operator_two() {
    let input = "!false";
    let expected = Object::Boolean(true);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_bang_operator_three() {
    let input = "!5";
    let expected = Object::Boolean(false);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_bang_operator_four() {
    let input = "!!true";
    let expected = Object::Boolean(true);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_bang_operator_five() {
    let input = "!!false";
    let expected = Object::Boolean(false);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_bang_operator_six() {
    let input = "!!5";
    let expected = Object::Boolean(true);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_bang_operator_seven() {
    let input = "!0";
    let expected = Object::Boolean(true);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_minus_operator_one() {
    let input = "-5";
    let expected = Object::Integer(-5);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_minus_operator_two() {
    let input = "-10";
    let expected = Object::Integer(-10);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_minus_operator_three() {
    let input = "--10";
    let expected = Object::Integer(10);
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
    let expected = Object::Integer(10);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_infix_operators_two() {
    let input = "2 * 2 * 2 * 2 * 2";
    let expected = Object::Integer(32);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_infix_operators_three() {
    let input = "5 + 2 * 10";
    let expected = Object::Integer(25);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_infix_operators_four() {
    let input = "(5 + 2) * 10";
    let expected = Object::Integer(70);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_infix_operators_five() {
    let input = "15 - 5 * -4";
    let expected = Object::Integer(35);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_infix_operators_six() {
    let input = "(5 + 10 * 2 + 10 / 2) * 4 + -2";
    let expected = Object::Integer(118);
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
    let expected = Object::Integer(10);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_if_expression_two() {
    let input = "if (false) { 10 }";
    let expected = Object::Null;
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_if_expression_three() {
    let input = "if (1) { 10 }";
    let expected = Object::Integer(10);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_if_expression_four() {
    let input = "if (1 < 2) { 10 }";
    let expected = Object::Integer(10);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_if_expression_five() {
    let input = "if (1 < 2) { 10 } else { 20 }";
    let expected = Object::Integer(10);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_if_expression_six() {
    let input = "if (1 > 2) { 10 } else { 20 }";
    let expected = Object::Integer(20);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_return_statement_one() {
    let input = "return 10;";
    let expected = Object::Integer(10);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_return_statement_two() {
    let input = "return 10; 9";
    let expected = Object::Integer(10);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_return_statement_three() {
    let input = "2 + 5; return 10; 9";
    let expected = Object::Integer(10);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_return_statement_four() {
    let input = "
if (10 > 1) {
    if (10 > 1) {
        return 10;
    }
    return 1;
}";
    let expected = Object::Integer(10);
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
    let expected = Object::Integer(10);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_let_statement_binding_one() {
    let input = "let a = 5; a;";
    let expected = Object::Integer(5);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_let_statement_binding_two() {
    let input = "let a = 2 * 5; a;";
    let expected = Object::Integer(10);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_let_statement_binding_three() {
    let input = "let a = 5; let b = a; b;";
    let expected = Object::Integer(5);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_let_statement_binding_four() {
    let input = "let a = 5; let b = a; let c = a + b; c;";
    let expected = Object::Integer(10);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_eval_let_statement_error_if_identifier_unbound() {
    let input = "foo";
    let expected_error = EvalError::UnrecognisedVariable;
    let error = parse_and_eval(input).err().unwrap();
    assert_eq!(error, expected_error);
}