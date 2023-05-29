use crate::vm::{chunk::Chunk, opcode::OpCode, value::Value, VM};

mod compiler;
mod vm;

fn main() {
    env_logger::init();

    let mut chunk = Chunk::new();
    let c1 = chunk.add_constant(Value::Number(1.2));
    chunk.write(OpCode::Const as u8, 0);
    chunk.write(c1 as u8, 0);

    let c2 = chunk.add_constant(Value::Number(3.4));
    chunk.write(OpCode::Const as u8, 1);
    chunk.write(c2 as u8, 1);

    chunk.write(OpCode::Add as u8, 2);

    let c3 = chunk.add_constant(Value::Number(5.6));
    chunk.write(OpCode::Const as u8, 3);
    chunk.write(c3 as u8, 3);

    chunk.write(OpCode::Div as u8, 4);
    chunk.write(OpCode::Neg as u8, 5);

    chunk.write(OpCode::Return as u8, 0);

    println!("{:?}", chunk);
    let mut vm = VM::new(chunk);
    vm.run();
}
