#![cfg(test)]

use crate::evaluator::eval;
use crate::evaluator::object::Object;

#[test]
fn test_eval_work_for_integer_expression() {
    let input = "5";
    let expected = Object::Integer(5);
    let result = eval(input);
    assert_eq!(result, Ok(expected));
}
