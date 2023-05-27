use crate::vm::{chunk::Chunk, opcode::OpCode};

mod vm;

fn main() {
    let mut c = Chunk::new();
    let constant = c.add_constant(1.2);
    c.write(OpCode::Const as u8);
    c.write(constant as u8);
    println!("{:?}", c);
}
