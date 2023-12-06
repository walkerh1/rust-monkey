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
        vec![make(OpCode::Constant, &[0_u32])]
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
fn test_compile_integer_arithmetic() {
    let input = "1 + 2";
    let expected = ByteCode(
        vec![
            make(OpCode::Constant, &[0_u32]),
            make(OpCode::Constant, &[1_u32]),
            make(OpCode::Add, &[]),
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
