use rail::bytecode::OpCode;
use rail::bytecode::chunk::Chunk;
use rail::bytecode::disassembler::disassemble;

fn main() {
    let mut chunk = Chunk::new();
    chunk.add_instruction(OpCode::Const, 0);
    chunk.add_instruction(OpCode::Return, 3);
    chunk.add_instruction(OpCode::Pop, 42);

    let dis = disassemble(&chunk);
    dis.iter().for_each(|string| println!("{string}"));
}
