use crate::bytecode::chunk::Chunk;

pub fn disassemble(chunk: &Chunk) -> Vec<String> {
    let mut result = Vec::new();
    for i in 0..chunk.len() {
        let code = chunk.get_code(i);
        let line = chunk.get_line(i);
        let out = format!("{i:0>4}: line: {line:0>4} - {code}");
        result.push(out);
    }
    result
}
