use super::{opcode::OpCode, value::Value};
use core::result::Result;
use std::fmt::{Debug, Write};

#[derive(Clone, PartialEq)]
pub struct Chunk {
    code: Vec<u8>,
    constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: vec![],
            constants: vec![],
        }
    }

    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    fn disassemble<W: Write>(&self, out: &mut W) -> Result<(), std::fmt::Error> {
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instr(out, offset)?;
        }
        Ok(())
    }

    fn disassemble_instr<W: Write>(
        &self,
        out: &mut W,
        offset: usize,
    ) -> Result<usize, std::fmt::Error> {
        write!(out, "{:04} ", offset)?;
        match OpCode::from(self.code[offset]) {
            OpCode::Const => self.const_instr(out, offset),
            OpCode::Return => self.simple_instr(out, "RET", offset),
            _ => {
                write!(out, "Unknown opcode {}", self.code[offset])?;
                Ok(offset + 1)
            }
        }
    }

    fn const_instr<W: Write>(&self, out: &mut W, offset: usize) -> Result<usize, std::fmt::Error> {
        let constant = self.code[offset + 1];
        write!(out, "CONST {:04} ", constant)?;
        write!(out, "{}", self.constants[constant as usize])?;
        writeln!(out)?;
        Ok(offset + 2)
    }

    fn simple_instr<W: Write>(
        &self,
        out: &mut W,
        name: &str,
        offset: usize,
    ) -> Result<usize, std::fmt::Error> {
        writeln!(out, "{}", name)?;
        Ok(offset + 1)
    }
}

impl Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.disassemble(f)
    }
}
