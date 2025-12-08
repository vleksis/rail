use std::{collections::HashMap, hash::Hash};

use thiserror::Error;

use crate::ast::{
    node::{ExprArena, ExpressionId, ExpressionKind},
    op::{InfixOperator, PrefixOperator},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type {
    Int64,
    Uint64,
    Float64,
    Bool,
    Unit,
}

#[derive(Debug)]
pub struct TypeEnv {
    infix: HashMap<(InfixOperator, Type, Type), Type>,
    prefix: HashMap<(PrefixOperator, Type), Type>,
}

impl TypeEnv {
    pub fn new() -> Self {
        Self {
            infix: Self::default_infix(),
            prefix: Self::default_prefix(),
        }
    }

    fn default_infix() -> HashMap<(InfixOperator, Type, Type), Type> {
        use InfixOperator::*;
        use Type::*;

        let mut infix = HashMap::new();

        let numeric = [Int64, Uint64, Float64];
        for &ty in &numeric {
            infix.insert((Plus, ty, ty), ty);
            infix.insert((Minus, ty, ty), ty);
            infix.insert((Mul, ty, ty), ty);
            infix.insert((Div, ty, ty), ty);
        }

        infix
    }

    fn default_prefix() -> HashMap<(PrefixOperator, Type), Type> {
        use PrefixOperator::*;
        use Type::*;

        let mut prefix = HashMap::new();

        prefix.insert((Minus, Int64), Int64);
        prefix.insert((Minus, Float64), Float64);

        prefix.insert((Negate, Bool), Bool);

        prefix
    }
}

impl TypeEnv {
    fn resolve_infix(&self, op: InfixOperator, lty: Type, rty: Type) -> Result<Type> {
        self.infix.get(&(op, lty, rty)).copied().ok_or(TypeError {})
    }

    fn resolve_prefix(&self, op: PrefixOperator, exp: Type) -> Result<Type> {
        self.prefix.get(&(op, exp)).copied().ok_or(TypeError {})
    }
}

#[derive(Debug)]
pub struct CompilationUnit {
    pub arena: ExprArena,                   // Syntax
    pub types: HashMap<ExpressionId, Type>, // Semantic
}

pub struct Typer<'e> {
    env: &'e TypeEnv,
}

impl<'e> Typer<'e> {
    pub fn new(env: &'e TypeEnv) -> Self {
        Self { env }
    }

    pub fn calculate_type(
        &self,
        arena: &ExprArena,
        types: &mut HashMap<ExpressionId, Type>,
        id: ExpressionId,
    ) -> Result<Type> {
        let kind = &arena.get(id).kind;
        let ty = match kind {
            ExpressionKind::Int64(_) => Type::Int64,
            ExpressionKind::Uint64(_) => Type::Uint64,
            ExpressionKind::Float64(_) => Type::Float64,
            ExpressionKind::Bool(_) => Type::Bool,
            ExpressionKind::Unit => Type::Unit,
            ExpressionKind::Infix(node) => {
                let lty = self.calculate_type(arena, types, node.lhs)?;
                let rty = self.calculate_type(arena, types, node.rhs)?;
                self.env.resolve_infix(node.op, lty, rty)?
            }
            ExpressionKind::Prefix(node) => {
                let ty = self.calculate_type(arena, types, node.exp)?;
                self.env.resolve_prefix(node.op, ty)?
            }
        };

        types.insert(id, ty);
        dbg!(ty);
        Ok(ty)
    }
}

#[derive(Debug, Error)]
#[error("Type Error during ast construction")]
pub struct TypeError {}

pub type Result<T> = std::result::Result<T, TypeError>;
