use super::*;
use crate::bytecode::OpCode;
use crate::runtime::*;

#[derive(Debug)]
pub struct Vm<'p> {
    program: &'p Program,
    frames: Vec<CallFrame<'p>>,
    stack: Vec<Value>,
    // memory: Vec<Object>,
}

impl<'p> Vm<'p> {
    pub fn from(program: &'p Program) -> Self {
        Self {
            program,
            frames: Vec::new(),
            stack: Vec::new(),
            // memory: Vec::new(),
        }
    }

    fn current_frame_mut(&mut self) -> Result<&mut CallFrame<'p>> {
        self.frames.last_mut().ok_or(Error::StackUnderflow)
    }

    pub fn pop(&mut self) -> Result<Value> {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }
    pub fn pop_bool(&mut self) -> Result<bool> {
        let value = self.pop()?;
        match value {
            Value::Bool(b) => Ok(b),
            _ => Err(Error::TypeMismatch("Expected bool")),
        }
    }
    pub fn pop_int64(&mut self) -> Result<i64> {
        let value = self.pop()?;
        match value {
            Value::Int64(i) => Ok(i),
            _ => Err(Error::TypeMismatch("Expected int64")),
        }
    }
    pub fn pop_uint64(&mut self) -> Result<u64> {
        let value = self.pop()?;
        match value {
            Value::Uint64(u) => Ok(u),
            _ => Err(Error::TypeMismatch("Expected uint64")),
        }
    }
    pub fn pop_float64(&mut self) -> Result<f64> {
        let value = self.pop()?;
        match value {
            Value::Float64(f) => Ok(f),
            _ => Err(Error::TypeMismatch("Expected float64")),
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
            None => Err(Error::StackUnderflow),
        }
    }
    fn push_frame(&mut self, function: &'p Function) -> Result<()> {
        let frame = CallFrame::new(function, self.stack.len() - function.arity as usize);
        self.frames.push(frame);
        Ok(())
    }

    pub fn run(&mut self) -> Result<i64> {
        use OpCode::*;

        let entry_function = &self.program.functions[self.program.entry];
        let _ = self.push_frame(entry_function);
        self.trace_call_enter(entry_function);

        while let Ok(frame) = self.current_frame_mut() {
            let op = frame.read_opcode()?;

            let _ = match op {
                Const => {
                    let idx = frame.read_u16()?;
                    let value = frame.get_const(idx);
                    self.trace_op_u16(Const, idx);
                    self.push(value)
                }

                I64Add => {
                    let rhs = self.pop_int64()?;
                    let lhs = self.pop_int64()?;
                    self.trace_op(I64Add);
                    self.push_int64(lhs + rhs)
                }
                I64Sub => {
                    let rhs = self.pop_int64()?;
                    let lhs = self.pop_int64()?;
                    self.trace_op(I64Sub);
                    self.push_int64(lhs - rhs)
                }
                I64Mul => {
                    let rhs = self.pop_int64()?;
                    let lhs = self.pop_int64()?;
                    self.trace_op(I64Mul);
                    self.push_int64(lhs * rhs)
                }
                I64Div => {
                    let rhs = self.pop_int64()?;
                    let lhs = self.pop_int64()?;
                    self.trace_op(I64Div);
                    self.push_int64(lhs / rhs)
                }
                I64Equal => {
                    let rhs = self.pop_int64()?;
                    let lhs = self.pop_int64()?;
                    self.trace_op(I64Equal);
                    self.push_bool(lhs == rhs)
                }
                I64NotEqual => {
                    let rhs = self.pop_int64()?;
                    let lhs = self.pop_int64()?;
                    self.trace_op(I64NotEqual);
                    self.push_bool(lhs != rhs)
                }
                I64Less => {
                    let rhs = self.pop_int64()?;
                    let lhs = self.pop_int64()?;
                    self.trace_op(I64Less);
                    self.push_bool(lhs < rhs)
                }
                I64LessEqual => {
                    let rhs = self.pop_int64()?;
                    let lhs = self.pop_int64()?;
                    self.trace_op(I64LessEqual);
                    self.push_bool(lhs <= rhs)
                }
                I64Greater => {
                    let rhs = self.pop_int64()?;
                    let lhs = self.pop_int64()?;
                    self.trace_op(I64Greater);
                    self.push_bool(lhs > rhs)
                }
                I64GreaterEqual => {
                    let rhs = self.pop_int64()?;
                    let lhs = self.pop_int64()?;
                    self.trace_op(I64GreaterEqual);
                    self.push_bool(lhs >= rhs)
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
                U64Equal => {
                    let rhs = self.pop_uint64()?;
                    let lhs = self.pop_uint64()?;
                    self.trace_op(U64Equal);
                    self.push_bool(lhs == rhs)
                }
                U64NotEqual => {
                    let rhs = self.pop_uint64()?;
                    let lhs = self.pop_uint64()?;
                    self.trace_op(U64NotEqual);
                    self.push_bool(lhs != rhs)
                }
                U64Less => {
                    let rhs = self.pop_uint64()?;
                    let lhs = self.pop_uint64()?;
                    self.trace_op(U64Less);
                    self.push_bool(lhs < rhs)
                }
                U64LessEqual => {
                    let rhs = self.pop_uint64()?;
                    let lhs = self.pop_uint64()?;
                    self.trace_op(U64LessEqual);
                    self.push_bool(lhs <= rhs)
                }
                U64Greater => {
                    let rhs = self.pop_uint64()?;
                    let lhs = self.pop_uint64()?;
                    self.trace_op(U64Greater);
                    self.push_bool(lhs > rhs)
                }
                U64GreaterEqual => {
                    let rhs = self.pop_uint64()?;
                    let lhs = self.pop_uint64()?;
                    self.trace_op(U64GreaterEqual);
                    self.push_bool(lhs >= rhs)
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
                F64Equal => {
                    let rhs = self.pop_float64()?;
                    let lhs = self.pop_float64()?;
                    self.trace_op(F64Equal);
                    self.push_bool(lhs == rhs)
                }
                F64NotEqual => {
                    let rhs = self.pop_float64()?;
                    let lhs = self.pop_float64()?;
                    self.trace_op(F64NotEqual);
                    self.push_bool(lhs != rhs)
                }
                F64Less => {
                    let rhs = self.pop_float64()?;
                    let lhs = self.pop_float64()?;
                    self.trace_op(F64Less);
                    self.push_bool(lhs < rhs)
                }
                F64LessEqual => {
                    let rhs = self.pop_float64()?;
                    let lhs = self.pop_float64()?;
                    self.trace_op(F64LessEqual);
                    self.push_bool(lhs <= rhs)
                }
                F64Greater => {
                    let rhs = self.pop_float64()?;
                    let lhs = self.pop_float64()?;
                    self.trace_op(F64Greater);
                    self.push_bool(lhs > rhs)
                }
                F64GreaterEqual => {
                    let rhs = self.pop_float64()?;
                    let lhs = self.pop_float64()?;
                    self.trace_op(F64GreaterEqual);
                    self.push_bool(lhs >= rhs)
                }

                BoolNot => {
                    let b = self.pop_bool()?;
                    self.trace_op(BoolNot);
                    self.push_bool(!b)
                }

                Pop => {
                    self.trace_op(Pop);
                    dbg!(self.pop()).map(|_| {})
                }
                Return => {
                    let func = frame.function;
                    self.trace_call_exit(func);
                    self.pop_frame()
                }
                Call => {
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
        println!("Leaving main");
        let value = dbg!(self.pop())?;
        match value {
            Value::Int64(i) => Ok(i),
            _ => Err(Error::TypeMismatch("Expected int64")),
        }
    }
}

impl<'p> Vm<'p> {
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
    use crate::bytecode::Chunk;

    use super::*;

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

        let mut vm = Vm::from(&program);
        let result = vm.run().expect("vm run failed");

        assert_eq!(result, 11);
    }
}
