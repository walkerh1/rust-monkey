#![cfg(test)]

use crate::code::{disassemble, make, Instructions, OpCode};

#[test]
fn test_make_op_constant() {
    let (op, operands) = (OpCode::Constant, [0xFFFE_u32]);
    let expected: [u8; 4] = [0x00, 0xFF, 0xFE, 0x00];
    let result = make(op, &operands);
    assert_eq!(result, expected);
}

#[test]
fn test_make_op_add() {
    let (op, operands) = (OpCode::Add, []);
    let expected: [u8; 4] = [0x01, 0x00, 0x00, 0x00];
    let result = make(op, &operands);
    assert_eq!(result, expected);
}

#[test]
fn test_make_op_pop() {
    let (op, operands) = (OpCode::Pop, []);
    let expected: [u8; 4] = [0x02, 0x00, 0x00, 0x00];
    let result = make(op, &operands);
    assert_eq!(result, expected);
}

#[test]
fn test_make_op_subtract() {
    let (op, operands) = (OpCode::Subtract, []);
    let expected: [u8; 4] = [0x03, 0x00, 0x00, 0x00];
    let result = make(op, &operands);
    assert_eq!(result, expected);
}

#[test]
fn test_make_op_multiply() {
    let (op, operands) = (OpCode::Multiply, []);
    let expected: [u8; 4] = [0x04, 0x00, 0x00, 0x00];
    let result = make(op, &operands);
    assert_eq!(result, expected);
}

#[test]
fn test_make_op_divide() {
    let (op, operands) = (OpCode::Divide, []);
    let expected: [u8; 4] = [0x05, 0x00, 0x00, 0x00];
    let result = make(op, &operands);
    assert_eq!(result, expected);
}

#[test]
fn test_make_op_true() {
    let (op, operands) = (OpCode::True, []);
    let expected: [u8; 4] = [0x06, 0x00, 0x00, 0x00];
    let result = make(op, &operands);
    assert_eq!(result, expected);
}

#[test]
fn test_make_op_false() {
    let (op, operands) = (OpCode::False, []);
    let expected: [u8; 4] = [0x07, 0x00, 0x00, 0x00];
    let result = make(op, &operands);
    assert_eq!(result, expected);
}

#[test]
fn test_disassemble() {
    let input: Instructions = vec![
        0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFE, 0x00, 0x02, 0x00, 0x00,
        0x00, 0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x06, 0x00,
        0x00, 0x00, 0x07, 0x00, 0x00, 0x00,
    ];
    let expected = String::from(
        "\
0000 OpConstant 1\n\
0004 OpAdd\n\
0008 OpConstant 65534\n\
000c OpPop\n\
0010 OpSubtract\n\
0014 OpMultiply\n\
0018 OpDivide\n\
001c OpTrue\n\
0020 OpFalse\n\
",
    );
    let result = disassemble(&input);
    assert_eq!(result, expected);
}
