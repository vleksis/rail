use super::*;
use crate::bytecode::OpCode;
use crate::runtime::*;

#[derive(Debug)]
pub struct CallFrame<'p> {
    pub(crate) function: &'p Function,
    pub(crate) ip: usize,
    pub(crate) stack_base: usize,
}

impl<'p> CallFrame<'p> {
    pub fn new(function: &'p Function, stack_base: usize) -> Self {
        Self {
            function,
            ip: 0,
            stack_base,
        }
    }

    /// Read OpCode at the current ip and advance ip.
    pub fn read_opcode(&mut self) -> Result<OpCode> {
        if self.ip >= self.function.chunk.len() {
            return Err(Error::InvalidJumpTarget);
        }
        let byte = self.function.chunk.get_byte(self.ip);
        self.ip += 1;
        OpCode::from_byte(byte).ok_or(Error::InvalidOpCode)
    }

    /// Read a big-endian u16 operand starting at the current ip and advance ip.
    pub fn read_u16(&mut self) -> Result<u16> {
        if self.ip + 1 >= self.function.chunk.len() {
            return Err(Error::InvalidJumpTarget);
        }
        let hi = self.function.chunk.get_byte(self.ip) as u16;
        let lo = self.function.chunk.get_byte(self.ip + 1) as u16;
        self.ip += 2;
        Ok((hi << 8) | lo)
    }

    pub fn get_const(&self, idx: u16) -> Value {
        self.function.chunk.get_const(idx as usize)
    }
}
