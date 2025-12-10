use std::collections::HashMap;

use crate::bytecode::*;
use crate::grammar::*;
use crate::module::Module;
use crate::runtime::*;
use crate::semantic::*;

#[derive(Debug)]
pub struct CodeGen {}

impl CodeGen {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile(&mut self, module: Module) -> Program {
        let mut chunk = Chunk::new();

        self.compile_expr(
            &module.syntax.arena,
            &module.types,
            &mut chunk,
            module.syntax.arena.get_root(),
        );

        chunk.add_instruction(OpCode::Return, 100);

        let main_fn = Function {
            name: "main".to_string(),
            chunk,
            arity: 0,
        };

        let mut program = Program::new();
        program.functions.push(main_fn);

        program
    }

    fn compile_expr(
        &mut self,
        arena: &expression::Arena,
        types: &HashMap<expression::Id, Type>,
        chunk: &mut Chunk,
        id: expression::Id,
    ) {
        let node = arena.get(id);
        let kind = &node.kind;
        let ty = types.get(&id).unwrap();
        let line = 42;

        match kind {
            expression::Kind::Int64(i) => chunk.add_int64(*i, line),
            expression::Kind::Uint64(u) => chunk.add_uint64(*u, line),
            expression::Kind::Float64(f) => chunk.add_float64(*f, line),
            expression::Kind::Bool(b) => chunk.add_bool(*b, line),

            expression::Kind::Infix { lhs, rhs, op } => {
                self.compile_expr(arena, types, chunk, *lhs);
                self.compile_expr(arena, types, chunk, *rhs);

                match (op, ty) {
                    (operator::Infix::Plus, Type::Int64) => {
                        chunk.add_instruction(OpCode::I64Add, line)
                    }
                    (operator::Infix::Minus, Type::Int64) => {
                        chunk.add_instruction(OpCode::I64Sub, line)
                    }
                    (operator::Infix::Mul, Type::Int64) => {
                        chunk.add_instruction(OpCode::I64Mul, line)
                    }
                    (operator::Infix::Div, Type::Int64) => {
                        chunk.add_instruction(OpCode::I64Div, line)
                    }

                    (operator::Infix::Plus, Type::Uint64) => {
                        chunk.add_instruction(OpCode::U64Add, line)
                    }
                    (operator::Infix::Minus, Type::Uint64) => {
                        chunk.add_instruction(OpCode::U64Sub, line)
                    }
                    (operator::Infix::Mul, Type::Uint64) => {
                        chunk.add_instruction(OpCode::U64Mul, line)
                    }
                    (operator::Infix::Div, Type::Uint64) => {
                        chunk.add_instruction(OpCode::U64Div, line)
                    }

                    (operator::Infix::Plus, Type::Float64) => {
                        chunk.add_instruction(OpCode::F64Add, line)
                    }
                    (operator::Infix::Minus, Type::Float64) => {
                        chunk.add_instruction(OpCode::F64Sub, line)
                    }
                    (operator::Infix::Mul, Type::Float64) => {
                        chunk.add_instruction(OpCode::F64Mul, line)
                    }
                    (operator::Infix::Div, Type::Float64) => {
                        chunk.add_instruction(OpCode::F64Div, line)
                    }

                    _ => unimplemented!("no codegen for {:?} with type {:?}", op, ty),
                };
            }

            expression::Kind::Prefix { op, exp } => {
                self.compile_expr(arena, types, chunk, *exp);

                match (op, ty) {
                    (operator::Prefix::Plus, _) => {
                        unreachable!("Prefix Plus is elided in ast building")
                    }
                    (operator::Prefix::Minus, Type::Int64) => {
                        chunk.add_int64(-1, line);
                        chunk.add_instruction(OpCode::I64Mul, line);
                    }
                    (operator::Prefix::Minus, Type::Float64) => {
                        chunk.add_float64(-1.0, line);
                        chunk.add_instruction(OpCode::F64Mul, line);
                    }
                    (operator::Prefix::Negate, Type::Bool) => {
                        chunk.add_instruction(OpCode::BoolNot, line);
                    }

                    _ => unimplemented!("no codegen for {:?} with type {:?}", op, ty),
                }
            }
            _ => unimplemented!(),
        };
    }
}
