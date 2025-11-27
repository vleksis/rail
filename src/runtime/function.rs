use crate::bytecode::chunk::Chunk;

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub chunk: Chunk,
    pub arity: u8,
}
