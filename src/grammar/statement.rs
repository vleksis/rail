use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub(crate) struct Id(pub(super) usize);

#[derive(Debug)]
pub(crate) enum Kind {
    Expression(expression::Id),
    Block(Vec<Id>),
    Let { name: String, init: expression::Id },
}

#[derive(Debug)]
pub(crate) struct Node {
    // line
    // byte
    pub(crate) kind: Kind,
}
