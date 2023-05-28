#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    Const,
    Neg,
    Return,
}

impl From<u8> for OpCode {
    fn from(byte: u8) -> Self {
        match byte {
            0 => OpCode::Const,
            1 => OpCode::Neg,
            2 => OpCode::Return,
            _ => panic!("invalid opcode"),
        }
    }
}
