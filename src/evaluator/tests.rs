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
fn test_eval_work_for_integer_expression() {
    let input = "5";
    let expected = Object::Integer(5);
    let result = parse_and_eval(input).ok().unwrap();
    assert_eq!(result, expected);
}
