use super::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub(crate) struct Id(usize);

#[derive(Debug)]
pub(crate) enum Kind {
    Int64(i64),
    Uint64(u64),
    Float64(f64),
    Bool(bool),
    Unit,

    Infix {
        lhs: Id,
        rhs: Id,
        op: operator::Infix,
    },

    Prefix {
        exp: Id,
        op: operator::Prefix,
    },
}

#[derive(Debug)]
pub(crate) struct Node {
    // line
    // byte
    pub(crate) kind: Kind,
}

#[derive(Debug, Default)]
pub struct Arena {
    nodes: Vec<Node>,
    root: Id,
}

impl Arena {
    pub fn get(&self, id: Id) -> &Node {
        &self.nodes[id.0]
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn set_root(&mut self, root: Id) {
        self.root = root;
    }

    pub fn get_root(&self) -> Id {
        self.root
    }
}

#[derive(Debug, Default)]
pub struct ASTBuilder {
    pub arena: Arena,
}

impl ASTBuilder {
    fn push(&mut self, node: Node) -> Id {
        let uid = Id(self.arena.len());
        self.arena.nodes.push(node);
        uid
    }

    pub(crate) fn make_int64(&mut self, i: i64) -> Id {
        let kind = Kind::Int64(i);
        let node = Node { kind };
        self.push(node)
    }

    pub(crate) fn make_uint64(&mut self, u: u64) -> Id {
        let kind = Kind::Uint64(u);
        let node = Node { kind };
        self.push(node)
    }

    pub(crate) fn make_float64(&mut self, f: f64) -> Id {
        let kind = Kind::Float64(f);
        let node = Node { kind };
        self.push(node)
    }

    pub(crate) fn make_bool(&mut self, b: bool) -> Id {
        let kind = Kind::Bool(b);
        let node = Node { kind };
        self.push(node)
    }

    pub(crate) fn make_infix(&mut self, op: operator::Infix, lhs: Id, rhs: Id) -> Id {
        let kind = Kind::Infix { lhs, rhs, op };
        let node = Node { kind };
        self.push(node)
    }

    pub(crate) fn make_prefix(&mut self, op: operator::Prefix, exp: Id) -> Id {
        let kind = Kind::Prefix { exp, op };
        let node = Node { kind };
        self.push(node)
    }

    pub(crate) fn make_postfix(&mut self, _op: operator::Postfix, _exp: Id) -> Id {
        unimplemented!()
    }
}
