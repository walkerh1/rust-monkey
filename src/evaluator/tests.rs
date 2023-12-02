#![cfg(test)]

use crate::evaluator::environment::Environment;
use crate::evaluator::object::{Function, Object};
use crate::evaluator::{eval, EvalError};
use crate::parser::ast::{Expression, Infix, Statement};
use crate::parser::Parser;
use std::cell::RefCell;
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
    let expected_error = EvalError::UnrecognisedVariable;
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
