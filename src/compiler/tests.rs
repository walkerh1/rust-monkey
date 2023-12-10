#![cfg(test)]

use crate::code::{make, OpCode};
use crate::compiler::{ByteCode, Compiler, CompilerError};
use crate::evaluator::object::Object;
use crate::parser::Parser;
use std::rc::Rc;

fn parse_and_compile(input: &str) -> (Option<ByteCode>, Option<CompilerError>) {
    let mut byte_code = None;
    let mut error = None;
    let ast = Parser::parse_program(input).expect("got a parsing error");
    match Compiler::compile(ast) {
        Ok(result) => byte_code = Some(result),
        Err(err) => error = Some(err),
    }
    (byte_code, error)
}

#[test]
fn test_compile_integer_object() {
    let input = "1096";
    let expected = ByteCode(
        vec![make(OpCode::Constant, &[0_u32]), make(OpCode::Pop, &[])]
            .into_iter()
            .flatten()
            .collect::<Vec<u8>>(),
        vec![Rc::new(Object::Integer(1096))],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_compile_integer_addition() {
    let input = "1 + 2";
    let expected = ByteCode(
        vec![
            make(OpCode::Constant, &[0_u32]),
            make(OpCode::Constant, &[1_u32]),
            make(OpCode::Add, &[]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![Rc::new(Object::Integer(1)), Rc::new(Object::Integer(2))],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_compile_integer_subtraction() {
    let input = "4 - 2";
    let expected = ByteCode(
        vec![
            make(OpCode::Constant, &[0_u32]),
            make(OpCode::Constant, &[1_u32]),
            make(OpCode::Subtract, &[]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![Rc::new(Object::Integer(4)), Rc::new(Object::Integer(2))],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_compile_integer_multiplication() {
    let input = "3 * 5";
    let expected = ByteCode(
        vec![
            make(OpCode::Constant, &[0_u32]),
            make(OpCode::Constant, &[1_u32]),
            make(OpCode::Multiply, &[]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![Rc::new(Object::Integer(3)), Rc::new(Object::Integer(5))],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_compile_integer_division() {
    let input = "10 / 2";
    let expected = ByteCode(
        vec![
            make(OpCode::Constant, &[0_u32]),
            make(OpCode::Constant, &[1_u32]),
            make(OpCode::Divide, &[]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![Rc::new(Object::Integer(10)), Rc::new(Object::Integer(2))],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_compile_nested_integer_expression() {
    let input = "(6 + 10) / 2";
    let expected = ByteCode(
        vec![
            make(OpCode::Constant, &[0_u32]),
            make(OpCode::Constant, &[1_u32]),
            make(OpCode::Add, &[]),
            make(OpCode::Constant, &[2_u32]),
            make(OpCode::Divide, &[]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![
            Rc::new(Object::Integer(6)),
            Rc::new(Object::Integer(10)),
            Rc::new(Object::Integer(2)),
        ],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_compile_boolean_literals() {
    let input = "true; false";
    let expected = ByteCode(
        vec![
            make(OpCode::True, &[]),
            make(OpCode::Pop, &[]),
            make(OpCode::False, &[]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_compile_boolean_expression_one() {
    let input = "1 > 2;";
    let expected = ByteCode(
        vec![
            make(OpCode::Constant, &[0_u32]),
            make(OpCode::Constant, &[1_u32]),
            make(OpCode::GreaterThan, &[]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![Rc::new(Object::Integer(1)), Rc::new(Object::Integer(2))],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_compile_boolean_expression_two() {
    let input = "1 < 2;";
    let expected = ByteCode(
        vec![
            make(OpCode::Constant, &[0_u32]),
            make(OpCode::Constant, &[1_u32]),
            make(OpCode::GreaterThan, &[]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![Rc::new(Object::Integer(2)), Rc::new(Object::Integer(1))],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_compile_boolean_expression() {
    let input = "1 == 2;";
    let expected = ByteCode(
        vec![
            make(OpCode::Constant, &[0_u32]),
            make(OpCode::Constant, &[1_u32]),
            make(OpCode::Equal, &[]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![Rc::new(Object::Integer(1)), Rc::new(Object::Integer(2))],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_compile_boolean_expression_three() {
    let input = "1 != 2;";
    let expected = ByteCode(
        vec![
            make(OpCode::Constant, &[0_u32]),
            make(OpCode::Constant, &[1_u32]),
            make(OpCode::NotEqual, &[]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![Rc::new(Object::Integer(1)), Rc::new(Object::Integer(2))],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_compile_boolean_expression_four() {
    let input = "true == false;";
    let expected = ByteCode(
        vec![
            make(OpCode::True, &[]),
            make(OpCode::False, &[]),
            make(OpCode::Equal, &[]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_compile_boolean_expression_five() {
    let input = "true != false;";
    let expected = ByteCode(
        vec![
            make(OpCode::True, &[]),
            make(OpCode::False, &[]),
            make(OpCode::NotEqual, &[]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_compile_minus_expressions() {
    let input = "-1";
    let expected = ByteCode(
        vec![
            make(OpCode::Constant, &[0_u32]),
            make(OpCode::Minus, &[]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![Rc::new(Object::Integer(1))],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_compile_bang_expression() {
    let input = "!true";
    let expected = ByteCode(
        vec![
            make(OpCode::True, &[]),
            make(OpCode::Bang, &[]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_compile_conditional_no_else() {
    let input = "if (true) { 10 }; 1024";
    let expected = ByteCode(
        vec![
            make(OpCode::True, &[]),                // 0000
            make(OpCode::JumpNotTruthy, &[12_u32]), // 0004
            make(OpCode::Constant, &[0_u32]),       // 0008
            make(OpCode::Pop, &[]),                 // 0012
            make(OpCode::Constant, &[1_u32]),       // 0016
            make(OpCode::Pop, &[]),                 // 0020
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![Rc::new(Object::Integer(10)), Rc::new(Object::Integer(1024))],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_compile_conditional_with_else() {
    let input = "if (true) { 10 } else { 20 }; 1024";
    let expected = ByteCode(
        vec![
            make(OpCode::True, &[]),                // 0000
            make(OpCode::JumpNotTruthy, &[16_u32]), // 0004
            make(OpCode::Constant, &[0_u32]),       // 0008
            make(OpCode::Jump, &[20_u32]),          // 0012
            make(OpCode::Constant, &[1_u32]),       // 0016
            make(OpCode::Pop, &[]),                 // 0020
            make(OpCode::Constant, &[2_u32]),       // 0024
            make(OpCode::Pop, &[]),                 // 0028
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![
            Rc::new(Object::Integer(10)),
            Rc::new(Object::Integer(20)),
            Rc::new(Object::Integer(1024)),
        ],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}
