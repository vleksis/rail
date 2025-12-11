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

        self.compile_statement(
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

    fn compile_statement(
        &mut self,
        arena: &Arena,
        types: &HashMap<expression::Id, Type>,
        chunk: &mut Chunk,
        id: statement::Id,
    ) {
        use OpCode::*;
        use statement::Kind::*;

        let node = arena.get_statement(id);
        let kind = &node.kind;
        let line = 42;

        match kind {
            Expression(exp) => {
                self.compile_expr(arena, types, chunk, *exp);
                chunk.add_instruction(Pop, line);
            }
            Block(stmts) => {
                for stmt in stmts {
                    self.compile_statement(arena, types, chunk, *stmt);
                }
            }
            _ => unimplemented!(),
        };
    }

    fn compile_expr(
        &mut self,
        arena: &Arena,
        types: &HashMap<expression::Id, Type>,
        chunk: &mut Chunk,
        id: expression::Id,
    ) {
        use OpCode::*;
        use expression::Kind::*;

        let node = arena.get_expression(id);
        let kind = &node.kind;
        let ty = types.get(&id).unwrap();
        let line = 42;

        match kind {
            Int64(i) => chunk.add_int64(*i, line),
            Uint64(u) => chunk.add_uint64(*u, line),
            Float64(f) => chunk.add_float64(*f, line),
            Bool(b) => chunk.add_bool(*b, line),

            Infix { lhs, rhs, op } => {
                self.compile_expr(arena, types, chunk, *lhs);
                self.compile_expr(arena, types, chunk, *rhs);
                let lty = types.get(lhs).unwrap();
                let rty = types.get(rhs).unwrap();

                use Type::*;
                use operator::Infix::*;

                match (lty, op, rty) {
                    (Int64, Plus, Int64) => chunk.add_instruction(I64Add, line),
                    (Int64, Minus, Int64) => chunk.add_instruction(I64Sub, line),
                    (Int64, Mul, Int64) => chunk.add_instruction(I64Mul, line),
                    (Int64, Div, Int64) => chunk.add_instruction(I64Div, line),

                    (Int64, Equal, Int64) => chunk.add_instruction(I64Equal, line),
                    (Int64, NotEqual, Int64) => chunk.add_instruction(I64NotEqual, line),
                    (Int64, Less, Int64) => chunk.add_instruction(I64Less, line),
                    (Int64, LessEqual, Int64) => chunk.add_instruction(I64LessEqual, line),
                    (Int64, Greater, Int64) => chunk.add_instruction(I64Greater, line),
                    (Int64, GreaterEqual, Int64) => chunk.add_instruction(I64GreaterEqual, line),

                    (Uint64, Plus, Uint64) => chunk.add_instruction(U64Add, line),
                    (Uint64, Minus, Uint64) => chunk.add_instruction(U64Sub, line),
                    (Uint64, Mul, Uint64) => chunk.add_instruction(U64Mul, line),
                    (Uint64, Div, Uint64) => chunk.add_instruction(U64Div, line),

                    (Uint64, Equal, Uint64) => chunk.add_instruction(U64Equal, line),
                    (Uint64, NotEqual, Uint64) => chunk.add_instruction(U64NotEqual, line),
                    (Uint64, Less, Uint64) => chunk.add_instruction(U64Less, line),
                    (Uint64, LessEqual, Uint64) => chunk.add_instruction(U64LessEqual, line),
                    (Uint64, Greater, Uint64) => chunk.add_instruction(U64Greater, line),
                    (Uint64, GreaterEqual, Uint64) => chunk.add_instruction(U64GreaterEqual, line),

                    (Float64, Plus, Float64) => chunk.add_instruction(F64Add, line),
                    (Float64, Minus, Float64) => chunk.add_instruction(F64Sub, line),
                    (Float64, Mul, Float64) => chunk.add_instruction(F64Mul, line),
                    (Float64, Div, Float64) => chunk.add_instruction(F64Div, line),

                    (Float64, Equal, Float64) => chunk.add_instruction(F64Equal, line),
                    (Float64, NotEqual, Float64) => chunk.add_instruction(F64NotEqual, line),
                    (Float64, Less, Float64) => chunk.add_instruction(F64Less, line),
                    (Float64, LessEqual, Float64) => chunk.add_instruction(F64LessEqual, line),
                    (Float64, Greater, Float64) => chunk.add_instruction(F64Greater, line),
                    (Float64, GreaterEqual, Float64) => {
                        chunk.add_instruction(F64GreaterEqual, line)
                    }

                    _ => unimplemented!(
                        "no codegen for {:?} with operand types {:?} and {:?}",
                        op,
                        lty,
                        rty
                    ),
                }
            }

            Prefix { op, exp } => {
                self.compile_expr(arena, types, chunk, *exp);

                use OpCode::*;
                use Type::*;
                use operator::Prefix::*;

                match (op, ty) {
                    (Plus, _) => {
                        unreachable!("Prefix Plus is elided in ast building")
                    }
                    (Minus, Int64) => {
                        chunk.add_int64(-1, line);
                        chunk.add_instruction(I64Mul, line);
                    }
                    (Minus, Float64) => {
                        chunk.add_float64(-1.0, line);
                        chunk.add_instruction(F64Mul, line);
                    }
                    (Negate, Bool) => {
                        chunk.add_instruction(BoolNot, line);
                    }

                    _ => unimplemented!("no codegen for {:?} with type {:?}", op, ty),
                }
            }
            _ => unimplemented!(),
        };
    }
}
