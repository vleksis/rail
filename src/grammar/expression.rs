use super::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub(crate) struct Id(pub(crate) usize);

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
