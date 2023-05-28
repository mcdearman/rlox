use crate::vm::{chunk::Chunk, opcode::OpCode, VM};

mod vm;

fn main() {
    let mut chunk = Chunk::new();
    let c1 = chunk.add_constant(1.2);
    chunk.write(OpCode::Const as u8, 0);
    chunk.write(c1 as u8, 0);
    chunk.write(OpCode::Neg as u8, 0);
    chunk.write(OpCode::Return as u8, 0);
    println!("{:?}", chunk);
    env_logger::init();
    let mut vm = VM::new(chunk);
    vm.run();
}
