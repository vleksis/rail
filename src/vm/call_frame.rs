use crate::bytecode::OpCode;
use crate::runtime::function::{self, Function};
use crate::runtime::object::ObjRef;
use crate::runtime::value::Value;
use crate::vm::error::{Result, VmError};

#[derive(Debug)]
pub struct CallFrame<'p> {
    function: &'p Function,
    ip: usize,
    stack_base: usize,
}

impl<'p> CallFrame<'p> {
    pub fn new(function: &'p Function, stack_base: usize) -> Self {
        Self {
            function,
            ip: 0,
            stack_base,
        }
    }

    /// # Precondition:
    /// ip is within chunk
    pub fn read_opcode(&mut self) -> Result<OpCode> {
        let byte = self.function.chunk.get_byte(self.ip);
        self.ip += 1;
        OpCode::from_byte(byte).ok_or(VmError::InvalidOpCode)
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        self.ip += 2;
        Ok(0u16)
    }

    pub fn get_const(&self, idx: u16) -> Value {
        self.function.chunk.get_const(idx as usize)
    }
}
