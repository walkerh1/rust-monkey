mod tests;

use std::fmt::{Display, Formatter};

pub const WORD_SIZE: usize = 4;

pub type Instructions = Vec<u8>;

pub fn disassemble(instructions: &Instructions) -> String {
    let mut assembly = String::from("");
    let mut address: u32 = 0;
    instructions.chunks_exact(WORD_SIZE).for_each(|word| {
        let op: OpCode = OpCode::try_from(word[0]).expect("Invalid OpCode");
        match op {
            OpCode::Constant => {
                let operand = read_u16(&word[1..=2]);
                assembly.push_str(&format!("{:04x} {} {}\n", address, op, operand))
            }
            OpCode::Add => assembly.push_str(&format!("{:04x} {}\n", address, op)),
        }
        address += 4;
    });
    assembly
}

pub fn read_u16(bytes: &[u8]) -> u16 {
    ((bytes[0] as u16) << 8) | bytes[1] as u16
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum OpCode {
    Constant,
    Add,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OpCode::Constant => "OpConstant",
                OpCode::Add => "OpAdd",
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
            _ => Err("Invalid OpCode"),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(value: OpCode) -> Self {
        match value {
            OpCode::Constant => 0x00,
            OpCode::Add => 0x01,
        }
    }
}

pub fn make(op: OpCode, operands: &[u32]) -> [u8; 4] {
    let mut instruction = [0x00; 4];
    match op {
        OpCode::Constant => {
            instruction[0] = u8::from(op);
            let operand = (operands[0] as u16).to_be_bytes();
            instruction[1] = operand[0];
            instruction[2] = operand[1];
        }
        OpCode::Add => {
            instruction[0] = u8::from(op);
        }
    }
    instruction
}
