type Instruction = [u8; 4];

pub struct Instructions(pub Vec<Instruction>);

#[derive(PartialOrd, PartialEq)]
enum OpCode {
    Constant,
}

fn make(op: OpCode, operands: &[u32]) -> Instruction {
    let mut instruction: Instruction = [0x00; 4];
    match op {
        OpCode::Constant => {
            instruction[0] = op as u8;
            let operand = (operands[0] as u16).to_be_bytes();
            instruction[1] = operand[0];
            instruction[2] = operand[1];
        }
    }
    instruction
}

mod tests {
    #![cfg(test)]

    use crate::compiler::code::{make, OpCode};

    #[test]
    fn test_make() {
        let (op, operands) = (OpCode::Constant, [0xFFFE_u32]);
        let expected: [u8; 4] = [0x00, 0xFF, 0xFE, 0x00];
        let result = make(op, &operands);
        assert_eq!(result, expected);
    }
}
