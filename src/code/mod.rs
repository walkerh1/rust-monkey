mod tests;

use std::fmt::{Display, Formatter};

pub const WORD_SIZE: usize = 4;

pub type Instructions = Vec<u8>;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum OpCode {
    Constant = 0,
    Add,
    Pop,
    Subtract,
    Multiply,
    Divide,
    True,
    False,
    Equal,
    NotEqual,
    GreaterThan,
    Minus,
    Bang,
    JumpNotTruthy,
    Jump,
    Null,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OpCode::Constant => "OpConstant",
                OpCode::Add => "OpAdd",
                OpCode::Pop => "OpPop",
                OpCode::Subtract => "OpSubtract",
                OpCode::Multiply => "OpMultiply",
                OpCode::Divide => "OpDivide",
                OpCode::True => "OpTrue",
                OpCode::False => "OpFalse",
                OpCode::Equal => "OpEqual",
                OpCode::NotEqual => "OpNotEqual",
                OpCode::GreaterThan => "OpGreaterThan",
                OpCode::Minus => "OpMinus",
                OpCode::Bang => "OpBang",
                OpCode::JumpNotTruthy => "OpJumpNotTruthy",
                OpCode::Jump => "OpJump",
                OpCode::Null => "OpNull",
            }
        )
    }
}

impl TryFrom<u8> for OpCode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(OpCode::Constant),
            0x01 => Ok(OpCode::Add),
            0x02 => Ok(OpCode::Pop),
            0x03 => Ok(OpCode::Subtract),
            0x04 => Ok(OpCode::Multiply),
            0x05 => Ok(OpCode::Divide),
            0x06 => Ok(OpCode::True),
            0x07 => Ok(OpCode::False),
            0x08 => Ok(OpCode::Equal),
            0x09 => Ok(OpCode::NotEqual),
            0x0a => Ok(OpCode::GreaterThan),
            0x0b => Ok(OpCode::Minus),
            0x0c => Ok(OpCode::Bang),
            0x0d => Ok(OpCode::JumpNotTruthy),
            0x0e => Ok(OpCode::Jump),
            0x0f => Ok(OpCode::Null),
            _ => Err("Invalid OpCode"),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(value: OpCode) -> Self {
        match value {
            OpCode::Constant => 0x00,
            OpCode::Add => 0x01,
            OpCode::Pop => 0x02,
            OpCode::Subtract => 0x03,
            OpCode::Multiply => 0x04,
            OpCode::Divide => 0x05,
            OpCode::True => 0x06,
            OpCode::False => 0x07,
            OpCode::Equal => 0x08,
            OpCode::NotEqual => 0x09,
            OpCode::GreaterThan => 0x0a,
            OpCode::Minus => 0x0b,
            OpCode::Bang => 0x0c,
            OpCode::JumpNotTruthy => 0x0d,
            OpCode::Jump => 0x0e,
            OpCode::Null => 0x0f,
        }
    }
}

pub fn make(op: OpCode, operands: &[u32]) -> [u8; 4] {
    let mut instruction = [0x00; 4];
    match op {
        OpCode::Constant | OpCode::JumpNotTruthy | OpCode::Jump => {
            instruction[0] = u8::from(op);
            let operand = (operands[0] as u16).to_be_bytes();
            instruction[1] = operand[0];
            instruction[2] = operand[1];
        }
        OpCode::Add
        | OpCode::Pop
        | OpCode::Subtract
        | OpCode::Multiply
        | OpCode::Divide
        | OpCode::True
        | OpCode::False
        | OpCode::Equal
        | OpCode::NotEqual
        | OpCode::GreaterThan
        | OpCode::Minus
        | OpCode::Bang
        | OpCode::Null => {
            instruction[0] = u8::from(op);
        }
    }
    instruction
}

#[allow(unused)]
pub fn disassemble(instructions: &Instructions) -> String {
    let mut assembly = String::from("");
    let mut address: u32 = 0;
    instructions.chunks_exact(WORD_SIZE).for_each(|word| {
        let op: OpCode = OpCode::try_from(word[0]).expect("Invalid OpCode");
        match op {
            OpCode::Constant | OpCode::JumpNotTruthy | OpCode::Jump => {
                let operand = read_u16(&word[1..=2]);
                assembly.push_str(&format!("{:04x} {} {}\n", address, op, operand))
            }
            OpCode::Add
            | OpCode::Pop
            | OpCode::Subtract
            | OpCode::Multiply
            | OpCode::Divide
            | OpCode::True
            | OpCode::False
            | OpCode::Equal
            | OpCode::NotEqual
            | OpCode::GreaterThan
            | OpCode::Minus
            | OpCode::Bang
            | OpCode::Null => assembly.push_str(&format!("{:04x} {}\n", address, op)),
        }
        address += 4;
    });
    assembly
}

pub fn read_u16(bytes: &[u8]) -> u16 {
    ((bytes[0] as u16) << 8) | bytes[1] as u16
}
