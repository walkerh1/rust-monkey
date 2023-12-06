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
fn test_disassemble() {
    let input: Instructions = vec![
        0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFE, 0x00, 0x02, 0x00, 0x00,
        0x00,
    ];
    let expected = String::from("0000 OpConstant 1\n0004 OpAdd\n0008 OpConstant 65534\n0012 OpPop");
    let result = disassemble(&input);
    assert_eq!(result, expected);
}
