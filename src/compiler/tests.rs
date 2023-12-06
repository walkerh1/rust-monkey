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
