#![cfg(test)]

use crate::evaluator::object::Object;
use crate::evaluator::{eval, EvalError};
use crate::parser::Parser;

fn parse_and_eval(input: &str) -> Result<Object, EvalError> {
    // assume only parsabale strings are provided
    let program = Parser::parse_program(input).unwrap();
    eval(program)
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
