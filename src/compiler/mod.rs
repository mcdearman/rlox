use crate::{compiler::token::TokenKind, vm::chunk};
use logos::Logos;

pub mod token;

pub fn compile<'src>(src: &'src str, chunk: &mut chunk::Chunk) -> bool {
    let lexer = TokenKind::lexer(src);
    todo!()
}
