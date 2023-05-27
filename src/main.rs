use crate::vm::{chunk::Chunk, opcode::OpCode, VM};

mod vm;

fn main() {
    let mut chunk = Chunk::new();
    let c1 = chunk.add_constant(1.2);
    let c2 = chunk.add_constant(2.3);
    chunk.write(OpCode::Const as u8, 0);
    chunk.write(c1 as u8, 0);
    chunk.write(OpCode::Const as u8, 1);
    chunk.write(c2 as u8, 1);
    chunk.write(OpCode::Return as u8, 0);
    println!("{:?}", chunk);
    let mut vm = VM::new(chunk);
    vm.run();
}
