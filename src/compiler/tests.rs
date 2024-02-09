#![cfg(test)]

use crate::code::{make, OpCode};
use crate::compiler::{ByteCode, Compiler, CompilerError};
use crate::object::{CompiledFunction, Object};
use crate::parser::Parser;
use std::rc::Rc;

fn parse_and_compile(input: &str) -> (Option<ByteCode>, Option<CompilerError>) {
    let mut byte_code = None;
    let mut error = None;
    let ast = Parser::parse_program(input).expect("got a parsing error");
    let mut compiler = Compiler::new();
    match compiler.compile(ast) {
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
            make(OpCode::JumpNotTruthy, &[16_u32]), // 0004
            make(OpCode::Constant, &[0_u32]),       // 0008
            make(OpCode::Jump, &[20_u32]),          // 0012
            make(OpCode::Null, &[]),                // 0016
            make(OpCode::Pop, &[]),                 // 0020
            make(OpCode::Constant, &[1_u32]),       // 0024
            make(OpCode::Pop, &[]),                 // 0028
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

#[test]
fn test_compile_global_let_statement_one() {
    let input = "
let one = 1;
let two = 2;
";
    let expected = ByteCode(
        vec![
            make(OpCode::Constant, &[0_u32]),
            make(OpCode::SetGlobal, &[0_u32]),
            make(OpCode::Constant, &[1_u32]),
            make(OpCode::SetGlobal, &[1_u32]),
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
fn test_compile_global_let_statement_two() {
    let input = "
let one = 1;
one;
";
    let expected = ByteCode(
        vec![
            make(OpCode::Constant, &[0_u32]),
            make(OpCode::SetGlobal, &[0_u32]),
            make(OpCode::GetGlobal, &[0_u32]),
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
fn test_compile_global_let_statement_three() {
    let input = "
let one = 1;
let two = one;
two;
";
    let expected = ByteCode(
        vec![
            make(OpCode::Constant, &[0_u32]),
            make(OpCode::SetGlobal, &[0_u32]),
            make(OpCode::GetGlobal, &[0_u32]),
            make(OpCode::SetGlobal, &[1_u32]),
            make(OpCode::GetGlobal, &[1_u32]),
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
fn test_compile_string_literal() {
    let input = "\"monkey\"";
    let expected = ByteCode(
        vec![make(OpCode::Constant, &[0_u32]), make(OpCode::Pop, &[])]
            .into_iter()
            .flatten()
            .collect::<Vec<u8>>(),
        vec![Rc::new(Object::String("monkey".to_string()))],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_compile_string_literal_addition() {
    let input = "\"mon\" + \"key\"";
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
        vec![
            Rc::new(Object::String("mon".to_string())),
            Rc::new(Object::String("key".to_string())),
        ],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_compile_array_literal_one() {
    let input = "[]";
    let expected = ByteCode(
        vec![make(OpCode::Array, &[0_u32]), make(OpCode::Pop, &[])]
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
fn test_compile_array_literal_two() {
    let input = "[1, 2, 3]";
    let expected = ByteCode(
        vec![
            make(OpCode::Constant, &[0_u32]),
            make(OpCode::Constant, &[1_u32]),
            make(OpCode::Constant, &[2_u32]),
            make(OpCode::Array, &[3_u32]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![
            Rc::new(Object::Integer(1)),
            Rc::new(Object::Integer(2)),
            Rc::new(Object::Integer(3)),
        ],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_compile_array_literal_three() {
    let input = "[1 + 2, 3 - 4, 5 * 6]";
    let expected = ByteCode(
        vec![
            make(OpCode::Constant, &[0_u32]),
            make(OpCode::Constant, &[1_u32]),
            make(OpCode::Add, &[]),
            make(OpCode::Constant, &[2_u32]),
            make(OpCode::Constant, &[3_u32]),
            make(OpCode::Subtract, &[]),
            make(OpCode::Constant, &[4_u32]),
            make(OpCode::Constant, &[5_u32]),
            make(OpCode::Multiply, &[]),
            make(OpCode::Array, &[3_u32]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![
            Rc::new(Object::Integer(1)),
            Rc::new(Object::Integer(2)),
            Rc::new(Object::Integer(3)),
            Rc::new(Object::Integer(4)),
            Rc::new(Object::Integer(5)),
            Rc::new(Object::Integer(6)),
        ],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_compile_hash_literal_one() {
    let input = "{}";
    let expected = ByteCode(
        vec![make(OpCode::Hash, &[0_u32]), make(OpCode::Pop, &[])]
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
fn test_compile_hash_literal_two() {
    let input = "{1: 2, 3: 4, 5: 6}";
    let expected = ByteCode(
        vec![
            make(OpCode::Constant, &[0_u32]),
            make(OpCode::Constant, &[1_u32]),
            make(OpCode::Constant, &[2_u32]),
            make(OpCode::Constant, &[3_u32]),
            make(OpCode::Constant, &[4_u32]),
            make(OpCode::Constant, &[5_u32]),
            make(OpCode::Hash, &[6_u32]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![
            Rc::new(Object::Integer(1)),
            Rc::new(Object::Integer(2)),
            Rc::new(Object::Integer(3)),
            Rc::new(Object::Integer(4)),
            Rc::new(Object::Integer(5)),
            Rc::new(Object::Integer(6)),
        ],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_compile_hash_literal_three() {
    let input = "{1: 2 + 3, 4: 5 * 6}";
    let expected = ByteCode(
        vec![
            make(OpCode::Constant, &[0_u32]),
            make(OpCode::Constant, &[1_u32]),
            make(OpCode::Constant, &[2_u32]),
            make(OpCode::Add, &[]),
            make(OpCode::Constant, &[3_u32]),
            make(OpCode::Constant, &[4_u32]),
            make(OpCode::Constant, &[5_u32]),
            make(OpCode::Multiply, &[]),
            make(OpCode::Hash, &[4_u32]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![
            Rc::new(Object::Integer(1)),
            Rc::new(Object::Integer(2)),
            Rc::new(Object::Integer(3)),
            Rc::new(Object::Integer(4)),
            Rc::new(Object::Integer(5)),
            Rc::new(Object::Integer(6)),
        ],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_compile_index_expression_one() {
    let input = "[1, 2, 3][1 + 1]";
    let expected = ByteCode(
        vec![
            make(OpCode::Constant, &[0_u32]),
            make(OpCode::Constant, &[1_u32]),
            make(OpCode::Constant, &[2_u32]),
            make(OpCode::Array, &[3_u32]),
            make(OpCode::Constant, &[3_u32]),
            make(OpCode::Constant, &[4_u32]),
            make(OpCode::Add, &[]),
            make(OpCode::Index, &[]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![
            Rc::new(Object::Integer(1)),
            Rc::new(Object::Integer(2)),
            Rc::new(Object::Integer(3)),
            Rc::new(Object::Integer(1)),
            Rc::new(Object::Integer(1)),
        ],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_compile_index_expression_three() {
    let input = "{1: 2}[2 - 1]";
    let expected = ByteCode(
        vec![
            make(OpCode::Constant, &[0_u32]),
            make(OpCode::Constant, &[1_u32]),
            make(OpCode::Hash, &[2_u32]),
            make(OpCode::Constant, &[2_u32]),
            make(OpCode::Constant, &[3_u32]),
            make(OpCode::Subtract, &[]),
            make(OpCode::Index, &[]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![
            Rc::new(Object::Integer(1)),
            Rc::new(Object::Integer(2)),
            Rc::new(Object::Integer(2)),
            Rc::new(Object::Integer(1)),
        ],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_compile_function_one() {
    let input = "fn() { return 5 + 10; }";
    let expected = ByteCode(
        vec![
            make(OpCode::Closure, &[2_u32, 0_u32]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![
            Rc::new(Object::Integer(5)),
            Rc::new(Object::Integer(10)),
            Rc::new(Object::CompiledFunc(Rc::new(CompiledFunction::new(
                vec![
                    make(OpCode::Constant, &[0_u32]),
                    make(OpCode::Constant, &[1_u32]),
                    make(OpCode::Add, &[]),
                    make(OpCode::ReturnValue, &[]),
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<u8>>(),
                0,
                0,
            )))),
        ],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_compile_function_two() {
    let input = "fn() { 5 + 10 }";
    let expected = ByteCode(
        vec![
            make(OpCode::Closure, &[2_u32, 0_u32]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![
            Rc::new(Object::Integer(5)),
            Rc::new(Object::Integer(10)),
            Rc::new(Object::CompiledFunc(Rc::new(CompiledFunction::new(
                vec![
                    make(OpCode::Constant, &[0_u32]),
                    make(OpCode::Constant, &[1_u32]),
                    make(OpCode::Add, &[]),
                    make(OpCode::ReturnValue, &[]),
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<u8>>(),
                0,
                0,
            )))),
        ],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_compile_function_three() {
    let input = "fn() { 1; 2 }";
    let expected = ByteCode(
        vec![
            make(OpCode::Closure, &[2_u32, 0_u32]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![
            Rc::new(Object::Integer(1)),
            Rc::new(Object::Integer(2)),
            Rc::new(Object::CompiledFunc(Rc::new(CompiledFunction::new(
                vec![
                    make(OpCode::Constant, &[0_u32]),
                    make(OpCode::Pop, &[]),
                    make(OpCode::Constant, &[1_u32]),
                    make(OpCode::ReturnValue, &[]),
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<u8>>(),
                0,
                0,
            )))),
        ],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_compile_function_four() {
    let input = "fn() {}";
    let expected = ByteCode(
        vec![
            make(OpCode::Closure, &[0_u32, 0_u32]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![Rc::new(Object::CompiledFunc(Rc::new(
            CompiledFunction::new(make(OpCode::Return, &[]).to_vec(), 0, 0),
        )))],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_compile_function_call_one() {
    let input = "fn() { 24 }()";
    let expected = ByteCode(
        vec![
            make(OpCode::Closure, &[1_u32, 0_u32]),
            make(OpCode::Call, &[0_u32]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![
            Rc::new(Object::Integer(24)),
            Rc::new(Object::CompiledFunc(Rc::new(CompiledFunction::new(
                vec![
                    make(OpCode::Constant, &[0_u32]),
                    make(OpCode::ReturnValue, &[]),
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<u8>>(),
                0,
                0,
            )))),
        ],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_compile_function_call_two() {
    let input = "let noArg = fn() { 24 }; noArg();";
    let expected = ByteCode(
        vec![
            make(OpCode::Closure, &[1_u32, 0_u32]),
            make(OpCode::SetGlobal, &[0_u32]),
            make(OpCode::GetGlobal, &[0_u32]),
            make(OpCode::Call, &[0_u32]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![
            Rc::new(Object::Integer(24)),
            Rc::new(Object::CompiledFunc(Rc::new(CompiledFunction::new(
                vec![
                    make(OpCode::Constant, &[0_u32]),
                    make(OpCode::ReturnValue, &[]),
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<u8>>(),
                0,
                0,
            )))),
        ],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_let_statement_scope_one() {
    let input = "
let num = 55;
fn() { num };
";
    let expected = ByteCode(
        vec![
            make(OpCode::Constant, &[0_u32]),
            make(OpCode::SetGlobal, &[0_u32]),
            make(OpCode::Closure, &[1_u32, 0_u32]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![
            Rc::new(Object::Integer(55)),
            Rc::new(Object::CompiledFunc(Rc::new(CompiledFunction::new(
                vec![
                    make(OpCode::GetGlobal, &[0_u32]),
                    make(OpCode::ReturnValue, &[]),
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<u8>>(),
                0,
                0,
            )))),
        ],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_let_statement_scope_two() {
    let input = "
fn() { 
    let num = 77;
    num
};
";
    let expected = ByteCode(
        vec![
            make(OpCode::Closure, &[1_u32, 0_u32]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![
            Rc::new(Object::Integer(77)),
            Rc::new(Object::CompiledFunc(Rc::new(CompiledFunction::new(
                vec![
                    make(OpCode::Constant, &[0_u32]),
                    make(OpCode::SetLocal, &[0_u32]),
                    make(OpCode::GetLocal, &[0_u32]),
                    make(OpCode::ReturnValue, &[]),
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<u8>>(),
                1,
                0,
            )))),
        ],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_let_statement_scope_three() {
    let input = "
fn() { 
    let a = 77;
    let b = 55;
    a + b
}
";
    let expected = ByteCode(
        vec![
            make(OpCode::Closure, &[2_u32, 0_u32]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![
            Rc::new(Object::Integer(77)),
            Rc::new(Object::Integer(55)),
            Rc::new(Object::CompiledFunc(Rc::new(CompiledFunction::new(
                vec![
                    make(OpCode::Constant, &[0_u32]),
                    make(OpCode::SetLocal, &[0_u32]),
                    make(OpCode::Constant, &[1_u32]),
                    make(OpCode::SetLocal, &[1_u32]),
                    make(OpCode::GetLocal, &[0_u32]),
                    make(OpCode::GetLocal, &[1_u32]),
                    make(OpCode::Add, &[]),
                    make(OpCode::ReturnValue, &[]),
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<u8>>(),
                2,
                0,
            )))),
        ],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_arguments_in_function_calls_one() {
    let input = "
let arg = fn(a) { };
arg(24);
";
    let expected = ByteCode(
        vec![
            make(OpCode::Closure, &[0_u32, 0_u32]),
            make(OpCode::SetGlobal, &[0_u32]),
            make(OpCode::GetGlobal, &[0_u32]),
            make(OpCode::Constant, &[1_u32]),
            make(OpCode::Call, &[1_u32]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![
            Rc::new(Object::CompiledFunc(Rc::new(CompiledFunction::new(
                vec![make(OpCode::Return, &[])]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<u8>>(),
                1,
                1,
            )))),
            Rc::new(Object::Integer(24)),
        ],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_arguments_in_function_calls_two() {
    let input = "
let arg = fn(a, b, c) { };
arg(1, 2, 3);
";
    let expected = ByteCode(
        vec![
            make(OpCode::Closure, &[0_u32, 0_u32]),
            make(OpCode::SetGlobal, &[0_u32]),
            make(OpCode::GetGlobal, &[0_u32]),
            make(OpCode::Constant, &[1_u32]),
            make(OpCode::Constant, &[2_u32]),
            make(OpCode::Constant, &[3_u32]),
            make(OpCode::Call, &[3_u32]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![
            Rc::new(Object::CompiledFunc(Rc::new(CompiledFunction::new(
                vec![make(OpCode::Return, &[])]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<u8>>(),
                3,
                3,
            )))),
            Rc::new(Object::Integer(1)),
            Rc::new(Object::Integer(2)),
            Rc::new(Object::Integer(3)),
        ],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_arguments_in_function_calls_three() {
    let input = "
let arg = fn(a) { a };
arg(24);
";
    let expected = ByteCode(
        vec![
            make(OpCode::Closure, &[0_u32, 0_u32]),
            make(OpCode::SetGlobal, &[0_u32]),
            make(OpCode::GetGlobal, &[0_u32]),
            make(OpCode::Constant, &[1_u32]),
            make(OpCode::Call, &[1_u32]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![
            Rc::new(Object::CompiledFunc(Rc::new(CompiledFunction::new(
                vec![
                    make(OpCode::GetLocal, &[0_u32]),
                    make(OpCode::ReturnValue, &[]),
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<u8>>(),
                1,
                1,
            )))),
            Rc::new(Object::Integer(24)),
        ],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_arguments_in_function_calls_four() {
    let input = "
let arg = fn(a, b, c) { a; b; c };
arg(1, 2, 3);
";
    let expected = ByteCode(
        vec![
            make(OpCode::Closure, &[0_u32, 0_u32]),
            make(OpCode::SetGlobal, &[0_u32]),
            make(OpCode::GetGlobal, &[0_u32]),
            make(OpCode::Constant, &[1_u32]),
            make(OpCode::Constant, &[2_u32]),
            make(OpCode::Constant, &[3_u32]),
            make(OpCode::Call, &[3_u32]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![
            Rc::new(Object::CompiledFunc(Rc::new(CompiledFunction::new(
                vec![
                    make(OpCode::GetLocal, &[0_u32]),
                    make(OpCode::Pop, &[]),
                    make(OpCode::GetLocal, &[1_u32]),
                    make(OpCode::Pop, &[]),
                    make(OpCode::GetLocal, &[2_u32]),
                    make(OpCode::ReturnValue, &[]),
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<u8>>(),
                3,
                3,
            )))),
            Rc::new(Object::Integer(1)),
            Rc::new(Object::Integer(2)),
            Rc::new(Object::Integer(3)),
        ],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}

#[test]
fn test_builtin_one() {
    let input = "
len([]);
push([], 1);
";
    let expected = ByteCode(
        vec![
            make(OpCode::GetBuiltin, &[0_u32]),
            make(OpCode::Array, &[0_u32]),
            make(OpCode::Call, &[1_u32]),
            make(OpCode::Pop, &[]),
            make(OpCode::GetBuiltin, &[4_u32]),
            make(OpCode::Array, &[0_u32]),
            make(OpCode::Constant, &[0_u32]),
            make(OpCode::Call, &[2_u32]),
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
fn test_builtin_two() {
    let input = "
fn() { len([]) };
";
    let expected = ByteCode(
        vec![
            make(OpCode::Closure, &[0_u32, 0_u32]),
            make(OpCode::Pop, &[]),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>(),
        vec![Rc::new(Object::CompiledFunc(Rc::new(
            CompiledFunction::new(
                vec![
                    make(OpCode::GetBuiltin, &[0_u32]),
                    make(OpCode::Array, &[0_u32]),
                    make(OpCode::Call, &[1_u32]),
                    make(OpCode::ReturnValue, &[]),
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<u8>>(),
                0,
                0,
            ),
        )))],
    );
    let (byte_code, error) = parse_and_compile(input);
    assert_eq!(error, None);
    assert_eq!(byte_code, Some(expected));
}
