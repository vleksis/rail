use crate::{bytecode::OpCode, runtime::value::Value};

#[derive(Debug)]
pub struct Chunk {
    bytes: Vec<u8>,
    lines: Vec<usize>,
    consts: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            bytes: Vec::new(),
            lines: Vec::new(),
            consts: Vec::new(),
        }
    }

    /// Low-level helper: append a single byte with its source line.
    fn push_byte(&mut self, byte: u8, line: usize) {
        self.bytes.push(byte);
        self.lines.push(line);
    }
    /// Append a u16 operand in big-endian form.
    fn push_u16(&mut self, value: u16, line: usize) {
        let [hi, lo] = value.to_be_bytes();
        self.push_byte(hi, line);
        self.push_byte(lo, line);
    }

    pub fn add_instruction(&mut self, op: OpCode, line: usize) {
        self.bytes.push(op as u8);
        self.lines.push(line);
    }

    pub fn add_const(&mut self, value: Value, line: usize) -> u16 {
        let idx: u16 = self.consts.len().try_into().expect("Too many constants");
        self.consts.push(value);
        self.add_instruction(OpCode::Const, line);
        self.push_u16(idx, line);
        idx
    }
    pub fn add_int64(&mut self, i: i64, line: usize) -> u16 {
        self.add_const(Value::Int64(i), line)
    }

    /// Number of bytes in the code stream
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn get_line(&self, idx: usize) -> usize {
        self.lines[idx]
    }

    pub fn get_byte(&self, idx: usize) -> u8 {
        self.bytes[idx]
    }

    pub fn get_const(&self, idx: usize) -> Value {
        self.consts[idx]
    }
}
