use crate::bytecode::{OpCode, chunk::Chunk};

pub fn disassemble(chunk: &Chunk) -> Vec<String> {
    let mut result = Vec::new();
    for i in 0..chunk.len() {
        let byte = chunk.get_byte(i);
        let code = if let Some(op) = OpCode::from_byte(byte) {
            op.to_string()
        } else {
            format!("{:#04x} byte", byte)
        };
        let line = chunk.get_line(i);
        let out = format!("{i:0>4}: line: {line:0>4} - {code}");
        result.push(out);
    }
    result
}
