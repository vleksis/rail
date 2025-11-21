use crate::bytecode::OpCode;

#[derive(Debug)]
pub struct Chunk {
    codes: Vec<OpCode>,
    lines: Vec<i32>,
    consts: Vec<()>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            codes: Vec::new(),
            lines: Vec::new(),
            consts: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, code: OpCode, line: i32) {
        self.codes.push(code);
        self.lines.push(line);
    }

    pub fn len(&self) -> usize {
        self.codes.len()
    }

    pub fn get_line(&self, idx: usize) -> i32 {
        self.lines[idx]
    }

    pub fn get_code(&self, idx: usize) -> OpCode {
        self.codes[idx]
    }
}
