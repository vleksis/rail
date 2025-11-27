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

    pub fn run(&mut self) -> Result<i64> {
        let entry_function = &self.program.functions[self.program.entry];
        let _ = self.push_frame(entry_function);
        self.trace_call_enter(entry_function);
        while let Ok(frame) = self.current_frame_mut() {
            let op = frame.read_opcode()?;

            let _ = match op {
                OpCode::Const => {
                    let idx = frame.read_u16()?;
                    let value = frame.get_const(idx);
                    self.trace_op_u16(OpCode::Const, idx);
                    self.push(value)
                }

                OpCode::I64Add => {
                    let rhs = self.pop_int64()?;
                    let lhs = self.pop_int64()?;
                    self.trace_op(OpCode::I64Add);
                    self.push_int64(lhs + rhs)
                }
                OpCode::I64Sub => {
                    let rhs = self.pop_int64()?;
                    let lhs = self.pop_int64()?;
                    self.trace_op(OpCode::I64Sub);
                    self.push_int64(lhs - rhs)
                }
                OpCode::I64Mul => {
                    let rhs = self.pop_int64()?;
                    let lhs = self.pop_int64()?;
                    self.trace_op(OpCode::I64Mul);
                    self.push_int64(lhs * rhs)
                }
                OpCode::I64Div => {
                    let rhs = self.pop_int64()?;
                    let lhs = self.pop_int64()?;
                    self.trace_op(OpCode::I64Div);
                    self.push_int64(lhs / rhs)
                }
                OpCode::I64Eq => {
                    let rhs = self.pop_int64()?;
                    let lhs = self.pop_int64()?;
                    self.trace_op(OpCode::I64Eq);
                    self.push_bool(lhs == rhs)
                }
                OpCode::I64Lt => {
                    let rhs = self.pop_int64()?;
                    let lhs = self.pop_int64()?;
                    self.trace_op(OpCode::I64Lt);
                    self.push_bool(lhs < rhs)
                }
                OpCode::I64Gt => {
                    let rhs = self.pop_int64()?;
                    let lhs = self.pop_int64()?;
                    self.trace_op(OpCode::I64Gt);
                    self.push_bool(lhs > rhs)
                }

                OpCode::U64Add => {
                    let rhs = self.pop_uint64()?;
                    let lhs = self.pop_uint64()?;
                    self.trace_op(OpCode::U64Add);
                    self.push_uint64(lhs + rhs)
                }
                OpCode::U64Sub => {
                    let rhs = self.pop_uint64()?;
                    let lhs = self.pop_uint64()?;
                    self.trace_op(OpCode::U64Sub);
                    self.push_uint64(lhs - rhs)
                }
                OpCode::U64Mul => {
                    let rhs = self.pop_uint64()?;
                    let lhs = self.pop_uint64()?;
                    self.trace_op(OpCode::U64Mul);
                    self.push_uint64(lhs * rhs)
                }
                OpCode::U64Div => {
                    let rhs = self.pop_uint64()?;
                    let lhs = self.pop_uint64()?;
                    self.trace_op(OpCode::U64Div);
                    self.push_uint64(lhs / rhs)
                }

                OpCode::F64Add => {
                    let rhs = self.pop_float64()?;
                    let lhs = self.pop_float64()?;
                    self.trace_op(OpCode::F64Add);
                    self.push_float64(lhs + rhs)
                }
                OpCode::F64Sub => {
                    let rhs = self.pop_float64()?;
                    let lhs = self.pop_float64()?;
                    self.trace_op(OpCode::F64Sub);
                    self.push_float64(lhs - rhs)
                }
                OpCode::F64Mul => {
                    let rhs = self.pop_float64()?;
                    let lhs = self.pop_float64()?;
                    self.trace_op(OpCode::F64Mul);
                    self.push_float64(lhs * rhs)
                }
                OpCode::F64Div => {
                    let rhs = self.pop_float64()?;
                    let lhs = self.pop_float64()?;
                    self.trace_op(OpCode::F64Div);
                    self.push_float64(lhs / rhs)
                }
                OpCode::F64Eq => {
                    let rhs = self.pop_float64()?;
                    let lhs = self.pop_float64()?;
                    self.trace_op(OpCode::F64Eq);
                    self.push_bool(lhs == rhs)
                }
                OpCode::F64Lt => {
                    let rhs = self.pop_float64()?;
                    let lhs = self.pop_float64()?;
                    self.trace_op(OpCode::F64Lt);
                    self.push_bool(lhs < rhs)
                }
                OpCode::F64Gt => {
                    let rhs = self.pop_float64()?;
                    let lhs = self.pop_float64()?;
                    self.trace_op(OpCode::F64Gt);
                    self.push_bool(lhs > rhs)
                }

                OpCode::BoolNot => {
                    let b = self.pop_bool()?;
                    self.trace_op(OpCode::BoolNot);
                    self.push_bool(!b)
                }

                OpCode::Return => {
                    let func = frame.function;
                    self.trace_call_exit(func);
                    self.pop_frame()
                }
                OpCode::Call => {
                    let idx = frame.read_u16()?;
                    let func = self.program.get_function(idx);
                    self.trace_call_enter(func);
                    self.push_frame(func)
                }

                _ => {
                    unimplemented!()
                }
            };
        }

        // main always return int64
        self.pop_int64()
    }
}

impl<'p> VM<'p> {
    fn trace_op(&self, _op: OpCode) {
        #[cfg(feature = "trace_vm")]
        {
            println!("{_op}");
        }
    }

    fn trace_op_u16(&self, _op: OpCode, _operand: u16) {
        #[cfg(feature = "trace_vm")]
        {
            println!("{_op} {_operand:#08x}");
        }
    }

    fn trace_call_enter(&self, _func: &'p Function) {
        #[cfg(feature = "trace_vm")]
        {
            println!("Called {_func:?}");
        }
    }

    fn trace_call_exit(&self, _func: &'p Function) {
        #[cfg(feature = "trace_vm")]
        {
            println!("Exited {_func:?}\n");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bytecode::chunk::Chunk;
    use crate::bytecode::opcode::OpCode;
    use crate::runtime::function::Function;
    use crate::runtime::program::Program;
    use crate::vm::vm::VM;

    #[test]
    fn i64_add_two_consts() {
        let mut chunk = Chunk::new();
        chunk.add_int64(4, 0);
        chunk.add_int64(7, 0);
        chunk.add_instruction(OpCode::I64Add, 0);
        chunk.add_instruction(OpCode::Return, 1);

        let main_fn = Function {
            name: "main".to_string(),
            chunk,
            arity: 0,
        };

        let mut program = Program::new();
        program.functions.push(main_fn);
        program.entry = 0;

        let mut vm = VM::from(&program);
        let result = vm.run().expect("vm run failed");

        assert_eq!(result, 11);
    }
}
