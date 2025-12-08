use std::collections::HashMap;

use crate::{
    ast::{
        node::{ExprArena, ExpressionId, ExpressionKind},
        op::{InfixOperator, PrefixOperator},
        types::Type,
    },
    bytecode::{OpCode, chunk::Chunk},
    runtime::function::Function,
};

#[derive(Debug)]
pub struct CodeGen {
    // pub prog: Program,
}

impl CodeGen {
    pub fn new() -> Self {
        Self {}
    }

    fn compile_function(&mut self) -> Function {
        unimplemented!()
    }

    pub fn compile_expr(
        &mut self,
        arena: &ExprArena,
        types: &HashMap<ExpressionId, Type>,
        chunk: &mut Chunk,
        id: ExpressionId,
    ) {
        let node = arena.get(id);
        let kind = &node.kind;
        let ty = types.get(&id).unwrap();
        let line = 42;

        match kind {
            ExpressionKind::Int64(i) => chunk.add_int64(*i, line),
            ExpressionKind::Uint64(u) => chunk.add_uint64(*u, line),
            ExpressionKind::Float64(f) => chunk.add_float64(*f, line),
            ExpressionKind::Bool(b) => chunk.add_bool(*b, line),

            ExpressionKind::Infix(infix) => {
                self.compile_expr(arena, types, chunk, infix.lhs);
                self.compile_expr(arena, types, chunk, infix.rhs);

                match (infix.op, ty) {
                    (InfixOperator::Plus, Type::Int64) => {
                        chunk.add_instruction(OpCode::I64Add, line)
                    }
                    (InfixOperator::Minus, Type::Int64) => {
                        chunk.add_instruction(OpCode::I64Sub, line)
                    }
                    (InfixOperator::Mul, Type::Int64) => {
                        chunk.add_instruction(OpCode::I64Mul, line)
                    }
                    (InfixOperator::Div, Type::Int64) => {
                        chunk.add_instruction(OpCode::I64Div, line)
                    }

                    (InfixOperator::Plus, Type::Uint64) => {
                        chunk.add_instruction(OpCode::U64Add, line)
                    }
                    (InfixOperator::Minus, Type::Uint64) => {
                        chunk.add_instruction(OpCode::U64Sub, line)
                    }
                    (InfixOperator::Mul, Type::Uint64) => {
                        chunk.add_instruction(OpCode::U64Mul, line)
                    }
                    (InfixOperator::Div, Type::Uint64) => {
                        chunk.add_instruction(OpCode::U64Div, line)
                    }

                    (InfixOperator::Plus, Type::Float64) => {
                        chunk.add_instruction(OpCode::F64Add, line)
                    }
                    (InfixOperator::Minus, Type::Float64) => {
                        chunk.add_instruction(OpCode::F64Sub, line)
                    }
                    (InfixOperator::Mul, Type::Float64) => {
                        chunk.add_instruction(OpCode::F64Mul, line)
                    }
                    (InfixOperator::Div, Type::Float64) => {
                        chunk.add_instruction(OpCode::F64Div, line)
                    }

                    _ => unimplemented!("no codegen for {:?} with type {:?}", infix.op, ty),
                };
            }

            ExpressionKind::Prefix(prefix) => {
                self.compile_expr(arena, types, chunk, prefix.exp);

                match (prefix.op, ty) {
                    (PrefixOperator::Plus, _) => {
                        unreachable!("Prefix Plus is elided in ast building")
                    }
                    (PrefixOperator::Minus, Type::Int64) => {
                        chunk.add_int64(-1, line);
                        chunk.add_instruction(OpCode::I64Mul, line);
                    }
                    (PrefixOperator::Minus, Type::Float64) => {
                        chunk.add_float64(-1.0, line);
                        chunk.add_instruction(OpCode::F64Mul, line);
                    }
                    (PrefixOperator::Negate, Type::Bool) => {
                        chunk.add_instruction(OpCode::BoolNot, line);
                    }

                    _ => unimplemented!("no codegen for {:?} with type {:?}", prefix.op, ty),
                }
            }
            _ => unimplemented!(),
        };
    }
}
