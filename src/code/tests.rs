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
fn test_make_op_equal() {
    let (op, operands) = (OpCode::Equal, []);
    let expected: [u8; 4] = [0x08, 0x00, 0x00, 0x00];
    let result = make(op, &operands);
    assert_eq!(result, expected);
}

#[test]
fn test_make_op_not_equal() {
    let (op, operands) = (OpCode::NotEqual, []);
    let expected: [u8; 4] = [0x09, 0x00, 0x00, 0x00];
    let result = make(op, &operands);
    assert_eq!(result, expected);
}

#[test]
fn test_make_op_greater_than() {
    let (op, operands) = (OpCode::GreaterThan, []);
    let expected: [u8; 4] = [0x0a, 0x00, 0x00, 0x00];
    let result = make(op, &operands);
    assert_eq!(result, expected);
}

#[test]
fn test_make_op_minus() {
    let (op, operands) = (OpCode::Minus, []);
    let expected: [u8; 4] = [0x0b, 0x00, 0x00, 0x00];
    let result = make(op, &operands);
    assert_eq!(result, expected);
}

#[test]
fn test_make_op_bang() {
    let (op, operands) = (OpCode::Bang, []);
    let expected: [u8; 4] = [0x0c, 0x00, 0x00, 0x00];
    let result = make(op, &operands);
    assert_eq!(result, expected);
}

#[test]
fn test_make_op_jump_not_truthy() {
    let (op, operands) = (OpCode::JumpNotTruthy, [0xFFFE_u32]);
    let expected: [u8; 4] = [0x0d, 0xFF, 0xFE, 0x00];
    let result = make(op, &operands);
    assert_eq!(result, expected);
}

#[test]
fn test_make_op_jump() {
    let (op, operands) = (OpCode::Jump, [0xFFFE_u32]);
    let expected: [u8; 4] = [0x0e, 0xFF, 0xFE, 0x00];
    let result = make(op, &operands);
    assert_eq!(result, expected);
}

#[test]
fn test_make_op_null() {
    let (op, operands) = (OpCode::Null, []);
    let expected: [u8; 4] = [0x0f, 0x00, 0x00, 0x00];
    let result = make(op, &operands);
    assert_eq!(result, expected);
}

#[test]
fn test_make_op_set_global() {
    let (op, operands) = (OpCode::SetGlobal, [0xFFFE_u32]);
    let expected: [u8; 4] = [0x10, 0xFF, 0xFE, 0x00];
    let result = make(op, &operands);
    assert_eq!(result, expected);
}

#[test]
fn test_make_op_get_global() {
    let (op, operands) = (OpCode::GetGlobal, [0xFFFE_u32]);
    let expected: [u8; 4] = [0x11, 0xFF, 0xFE, 0x00];
    let result = make(op, &operands);
    assert_eq!(result, expected);
}

#[test]
fn test_make_op_array() {
    let (op, operands) = (OpCode::Array, [0xFFFE_u32]);
    let expected: [u8; 4] = [0x12, 0xFF, 0xFE, 0x00];
    let result = make(op, &operands);
    assert_eq!(result, expected);
}

#[test]
fn test_make_op_hash() {
    let (op, operands) = (OpCode::Hash, [0xFFFE_u32]);
    let expected: [u8; 4] = [0x13, 0xFF, 0xFE, 0x00];
    let result = make(op, &operands);
    assert_eq!(result, expected);
}

#[test]
fn test_make_op_index() {
    let (op, operands) = (OpCode::Index, []);
    let expected: [u8; 4] = [0x14, 0x00, 0x00, 0x00];
    let result = make(op, &operands);
    assert_eq!(result, expected);
}

#[test]
fn test_make_op_call() {
    let (op, operands) = (OpCode::Call, [0xFE]);
    let expected: [u8; 4] = [0x15, 0xFE, 0x00, 0x00];
    let result = make(op, &operands);
    assert_eq!(result, expected);
}

#[test]
fn test_make_op_return_value() {
    let (op, operands) = (OpCode::ReturnValue, []);
    let expected: [u8; 4] = [0x16, 0x00, 0x00, 0x00];
    let result = make(op, &operands);
    assert_eq!(result, expected);
}

#[test]
fn test_make_op_return() {
    let (op, operands) = (OpCode::Return, []);
    let expected: [u8; 4] = [0x17, 0x00, 0x00, 0x00];
    let result = make(op, &operands);
    assert_eq!(result, expected);
}

#[test]
fn test_make_op_set_local() {
    let (op, operands) = (OpCode::SetLocal, [0xFE]);
    let expected: [u8; 4] = [0x18, 0xFE, 0x00, 0x00];
    let result = make(op, &operands);
    assert_eq!(result, expected);
}

#[test]
fn test_make_op_get_local() {
    let (op, operands) = (OpCode::GetLocal, [0xFE]);
    let expected: [u8; 4] = [0x19, 0xFE, 0x00, 0x00];
    let result = make(op, &operands);
    assert_eq!(result, expected);
}

#[test]
fn test_make_op_get_builtin() {
    let (op, operands) = (OpCode::GetBuiltin, [0xFE]);
    let expected: [u8; 4] = [0x1a, 0xFE, 0x00, 0x00];
    let result = make(op, &operands);
    assert_eq!(result, expected);
}

#[test]
fn test_make_op_closure() {
    let (op, operands) = (OpCode::Closure, [0xFFFE, 0xFE]);
    let expected: [u8; 4] = [0x1b, 0xFF, 0xFE, 0xFE];
    let result = make(op, &operands);
    assert_eq!(result, expected);
}

#[test]
fn test_disassemble() {
    let input: Instructions = vec![
        0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFE, 0x00, 0x02, 0x00, 0x00,
        0x00, 0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x06, 0x00,
        0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x0a,
        0x00, 0x00, 0x00, 0x0b, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x0d, 0xFF, 0xFE, 0x00,
        0x0e, 0xFF, 0xFE, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x10, 0xFF, 0xFE, 0x00, 0x11, 0xFF, 0xFE,
        0x00, 0x12, 0xFF, 0xFE, 0x00, 0x13, 0xFF, 0xFE, 0x00, 0x14, 0x00, 0x00, 0x00, 0x15, 0xFE,
        0x00, 0x00, 0x16, 0x00, 0x00, 0x00, 0x17, 0x00, 0x00, 0x00, 0x18, 0xFE, 0x00, 0x00, 0x19,
        0xFE, 0x00, 0x00, 0x1a, 0xFE, 0x00, 0x00, 0x1b, 0xFF, 0xFE, 0xFE,
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
0024 OpEqual\n\
0028 OpNotEqual\n\
002c OpGreaterThan\n\
0030 OpMinus\n\
0034 OpBang\n\
0038 OpJumpNotTruthy 65534\n\
003c OpJump 65534\n\
0040 OpNull\n\
0044 OpSetGlobal 65534\n\
0048 OpGetGlobal 65534\n\
004c OpArray 65534\n\
0050 OpHash 65534\n\
0054 OpIndex\n\
0058 OpCall 254\n\
005c OpReturnValue\n\
0060 OpReturn\n\
0064 OpSetLocal 254\n\
0068 OpGetLocal 254\n\
006c OpGetBuiltin 254\n\
0070 OpClosure 65534 254\n\
",
    );
    let result = disassemble(&input);
    assert_eq!(result, expected);
}
