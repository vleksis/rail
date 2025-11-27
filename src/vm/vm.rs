use crate::bytecode::OpCode;
use crate::runtime::function::Function;
use crate::runtime::program::Program;
use crate::runtime::value::Value;
use crate::vm::call_frame::CallFrame;
use crate::vm::error::{Result, VmError};

#[derive(Debug)]
pub struct VM<'p> {
    program: &'p Program,
    frames: Vec<CallFrame<'p>>,
    stack: Vec<Value>,
    // memory: Vec<Object>,
}

impl<'p> VM<'p> {
    pub fn from(program: &'p Program) -> Self {
        Self {
            program,
            frames: Vec::new(),
            stack: Vec::new(),
            // memory: Vec::new(),
        }
    }

    fn current_frame(&self) -> Result<&CallFrame<'p>> {
        return self.frames.last().ok_or(VmError::StackUnderflow);
    }

    fn current_frame_mut(&mut self) -> Result<&mut CallFrame<'p>> {
        return self.frames.last_mut().ok_or(VmError::StackUnderflow);
    }

    pub fn pop(&mut self) -> Result<Value> {
        self.stack.pop().ok_or(VmError::StackUnderflow)
    }
    pub fn pop_bool(&mut self) -> Result<bool> {
        let value = self.pop()?;
        match value {
            Value::Bool(b) => Ok(b),
            _ => Err(VmError::TypeMismatch("Expected bool")),
        }
    }
    pub fn pop_int64(&mut self) -> Result<i64> {
        let value = self.pop()?;
        match value {
            Value::Int64(i) => Ok(i),
            _ => Err(VmError::TypeMismatch("Expected int64")),
        }
    }
    pub fn pop_uint64(&mut self) -> Result<u64> {
        let value = self.pop()?;
        match value {
            Value::Uint64(u) => Ok(u),
            _ => Err(VmError::TypeMismatch("Expected uint64")),
        }
    }
    pub fn pop_float64(&mut self) -> Result<f64> {
        let value = self.pop()?;
        match value {
            Value::Float64(f) => Ok(f),
            _ => Err(VmError::TypeMismatch("Expected float64")),
        }
    }

    fn push(&mut self, value: Value) -> Result<()> {
        self.stack.push(value);
        Ok(())
    }
    fn push_int64(&mut self, v: i64) -> Result<()> {
        self.push(Value::Int64(v))
    }
    fn push_uint64(&mut self, v: u64) -> Result<()> {
        self.push(Value::Uint64(v))
    }
    fn push_float64(&mut self, v: f64) -> Result<()> {
        self.push(Value::Float64(v))
    }
    fn push_bool(&mut self, v: bool) -> Result<()> {
        self.push(Value::Bool(v))
    }

    fn pop_frame(&mut self) -> Result<()> {
        match self.frames.pop() {
            Some(_) => Ok(()),
            None => Err(VmError::StackUnderflow),
        }
    }

    fn push_frame(&mut self, function: &'p Function) -> Result<()> {
        let frame = CallFrame::new(function, self.stack.len() - function.arity as usize);
        self.frames.push(frame);
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        let entry_function = &self.program.functions[self.program.entry];
        let _ = self.push_frame(entry_function);
        while let Ok(frame) = self.current_frame_mut() {
            let op = frame.read_opcode()?;

            #[cfg(debug_assertions)]
            {
                println!("{op:?}");
            }

            let noname = match op {
                OpCode::Const => {
                    let idx = frame.read_u16()?;
                    let value = frame.get_const(idx);
                    self.push(value)
                }
                OpCode::I64Add => {
                    let rhs = self.pop_int64()?;
                    let lhs = self.pop_int64()?;
                    self.push_int64(lhs + rhs)
                }
                OpCode::I64Sub => {
                    let rhs = self.pop_int64()?;
                    let lhs = self.pop_int64()?;
                    self.push_int64(lhs - rhs)
                }
                OpCode::I64Mul => {
                    let rhs = self.pop_int64()?;
                    let lhs = self.pop_int64()?;
                    self.push_int64(lhs * rhs)
                }
                OpCode::I64Div => {
                    let rhs = self.pop_int64()?;
                    let lhs = self.pop_int64()?;
                    self.push_int64(lhs / rhs)
                }
                OpCode::I64Eq => {
                    let rhs = self.pop_int64()?;
                    let lhs = self.pop_int64()?;
                    self.push_bool(lhs == rhs)
                }
                OpCode::I64Lt => {
                    let rhs = self.pop_int64()?;
                    let lhs = self.pop_int64()?;
                    self.push_bool(lhs < rhs)
                }
                OpCode::I64Gt => {
                    let rhs = self.pop_int64()?;
                    let lhs = self.pop_int64()?;
                    self.push_bool(lhs > rhs)
                }

                OpCode::U64Add => {
                    let rhs = self.pop_uint64()?;
                    let lhs = self.pop_uint64()?;
                    self.push_uint64(lhs + rhs)
                }
                OpCode::U64Sub => {
                    let rhs = self.pop_uint64()?;
                    let lhs = self.pop_uint64()?;
                    self.push_uint64(lhs - rhs)
                }
                OpCode::U64Mul => {
                    let rhs = self.pop_uint64()?;
                    let lhs = self.pop_uint64()?;
                    self.push_uint64(lhs * rhs)
                }
                OpCode::U64Div => {
                    let rhs = self.pop_uint64()?;
                    let lhs = self.pop_uint64()?;
                    self.push_uint64(lhs / rhs)
                }

                OpCode::F64Add => {
                    let rhs = self.pop_float64()?;
                    let lhs = self.pop_float64()?;
                    self.push_float64(lhs + rhs)
                }
                OpCode::F64Sub => {
                    let rhs = self.pop_float64()?;
                    let lhs = self.pop_float64()?;
                    self.push_float64(lhs - rhs)
                }
                OpCode::F64Mul => {
                    let rhs = self.pop_float64()?;
                    let lhs = self.pop_float64()?;
                    self.push_float64(lhs * rhs)
                }
                OpCode::F64Div => {
                    let rhs = self.pop_float64()?;
                    let lhs = self.pop_float64()?;
                    self.push_float64(lhs / rhs)
                }
                OpCode::F64Eq => {
                    let rhs = self.pop_float64()?;
                    let lhs = self.pop_float64()?;
                    self.push_bool(lhs == rhs)
                }
                OpCode::F64Lt => {
                    let rhs = self.pop_float64()?;
                    let lhs = self.pop_float64()?;
                    self.push_bool(lhs < rhs)
                }
                OpCode::F64Gt => {
                    let rhs = self.pop_float64()?;
                    let lhs = self.pop_float64()?;
                    self.push_bool(lhs > rhs)
                }

                OpCode::BoolNot => {
                    let b = self.pop_bool()?;
                    self.push_bool(!b)
                }

                OpCode::Return => self.pop_frame(),

                _ => {
                    unimplemented!()
                }
            };

            #[cfg(debug_assertions)]
            {
                println!("{:?}", self.frames);
                println!("{:?}", self.stack);
            }
        }

        Ok(())
    }
}
