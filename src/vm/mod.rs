use self::{chunk::Chunk, value::Value};
use crate::vm::opcode::OpCode;

pub mod chunk;
pub mod opcode;
pub mod value;

// #[derive(Debug, Clone, PartialEq)]
// pub struct RuntimeError(pub String);

// impl RuntimeError {
//     pub fn new(msg: String) -> RuntimeError {
//         RuntimeError(msg)
//     }
// }

// impl From<&str> for RuntimeError {
//     fn from(msg: &str) -> RuntimeError {
//         RuntimeError::new(msg.to_string())
//     }
// }

// pub type Result<T> = std::result::Result<T, RuntimeError>;

#[derive(Debug, Clone, PartialEq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VM {
    chunk: Chunk,
    ip: usize,
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        Self { chunk, ip: 0 }
    }

    pub fn run(&mut self) -> InterpretResult {
        loop {
            let instr = self.read_instr();
            match instr {
                OpCode::Const => {
                    let constant = self.read_constant();
                    println!("{}", constant);
                    return InterpretResult::Ok;
                }
                OpCode::Return => return InterpretResult::Ok,
                _ => return InterpretResult::RuntimeError,
            }
        }
    }

    fn read_instr(&mut self) -> OpCode {
        let instr = self.chunk.code[self.ip];
        self.ip += 1;
        OpCode::from(instr)
    }

    fn read_constant(&mut self) -> Value {
        let op = self.read_instr();
        self.chunk.constants[op as usize]
    }
}
