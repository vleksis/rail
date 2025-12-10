use std::collections::HashMap;

use super::*;
use crate::grammar::*;
use crate::module::Module;
use crate::semantic::*;

pub struct Typer<'e> {
    env: &'e TypeEnv,
}

impl<'e> Typer<'e> {
    pub fn new(env: &'e TypeEnv) -> Self {
        Self { env }
    }

    pub fn check(&self, syntax: Syntax) -> Result<Module> {
        let mut types = HashMap::new();
        let root = syntax.arena.get_root();
        self.calculate_type(&syntax.arena, &mut types, root)?;
        let module = Module { syntax, types };
        Ok(module)
    }

    fn calculate_type(
        &self,
        arena: &expression::Arena,
        types: &mut HashMap<expression::Id, Type>,
        id: expression::Id,
    ) -> Result<Type> {
        let kind = &arena.get(id).kind;
        let ty = match kind {
            expression::Kind::Int64(_) => Type::Int64,
            expression::Kind::Uint64(_) => Type::Uint64,
            expression::Kind::Float64(_) => Type::Float64,
            expression::Kind::Bool(_) => Type::Bool,
            expression::Kind::Unit => Type::Unit,
            expression::Kind::Infix { lhs, rhs, op } => {
                let lty = self.calculate_type(arena, types, *lhs)?;
                let rty = self.calculate_type(arena, types, *rhs)?;
                self.env.resolve_infix(*op, lty, rty)?
            }
            expression::Kind::Prefix { op, exp } => {
                let ty = self.calculate_type(arena, types, *exp)?;
                self.env.resolve_prefix(*op, ty)?
            }
        };

        types.insert(id, ty);
        dbg!(ty);
        Ok(ty)
    }
}

impl TypeEnv {
    fn resolve_infix(&self, op: operator::Infix, lty: Type, rty: Type) -> Result<Type> {
        self.infix.get(&(op, lty, rty)).copied().ok_or(Error {})
    }

    fn resolve_prefix(&self, op: operator::Prefix, exp: Type) -> Result<Type> {
        self.prefix.get(&(op, exp)).copied().ok_or(Error {})
    }
}
