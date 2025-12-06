use crate::ast::op::{InfixOperator, PostfixOperator, PrefixOperator};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ExpressionId(usize);

#[derive(Debug)]
pub struct ExpressionNode {
    pub(crate) uid: ExpressionId,
    pub(crate) kind: ExpressionKind,
}

#[derive(Debug)]
pub(crate) enum ExpressionKind {
    Int64(i64),
    Uint64(u64),
    Float64(f64),
    Bool(bool),
    Unit,

    Infix(Infix),

    Prefix(Prefix),
}

#[derive(Debug)]
pub(crate) struct Infix {
    pub(crate) lhs: ExpressionId,
    pub(crate) rhs: ExpressionId,
    pub(crate) op: InfixOperator,
}

#[derive(Debug)]
pub(crate) struct Prefix {
    pub(crate) exp: ExpressionId,
    pub(crate) op: PrefixOperator,
}

#[derive(Debug, Default)]
pub struct ExprArena {
    nodes: Vec<ExpressionNode>,
}

impl ExprArena {
    pub fn get(&self, id: ExpressionId) -> &ExpressionNode {
        &self.nodes[id.0]
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

#[derive(Debug, Default)]
pub struct ASTBuilder {
    pub arena: ExprArena,
}

impl ASTBuilder {
    fn push(&mut self, kind: ExpressionKind) -> ExpressionId {
        let uid = ExpressionId(self.arena.len());
        let node = ExpressionNode { kind, uid };
        self.arena.nodes.push(node);
        uid
    }

    pub(crate) fn make_int64(&mut self, i: i64) -> ExpressionId {
        let kind = ExpressionKind::Int64(i);
        self.push(kind)
    }

    pub(crate) fn make_uint64(&mut self, u: u64) -> ExpressionId {
        let kind = ExpressionKind::Uint64(u);
        self.push(kind)
    }

    pub(crate) fn make_float64(&mut self, f: f64) -> ExpressionId {
        let kind = ExpressionKind::Float64(f);
        self.push(kind)
    }

    pub(crate) fn make_infix(
        &mut self,
        op: InfixOperator,
        lhs: ExpressionId,
        rhs: ExpressionId,
    ) -> ExpressionId {
        let kind = Infix { lhs, rhs, op };
        let kind = ExpressionKind::Infix(kind);
        self.push(kind)
    }

    pub(crate) fn make_prefix(&mut self, op: PrefixOperator, exp: ExpressionId) -> ExpressionId {
        let kind = Prefix { exp, op };
        let kind = ExpressionKind::Prefix(kind);
        self.push(kind)
    }

    pub(crate) fn make_postfix(&mut self, op: PostfixOperator, exp: ExpressionId) -> ExpressionId {
        let kind = match op {
            PostfixOperator::Nothing => unimplemented!(),
        };
        self.push(kind)
    }
}
