use ptree::{TreeBuilder, item::StringItem, print_tree};

use crate::ast::{
    node::{ExprArena, ExpressionId, ExpressionKind},
    op::{InfixOperator, PrefixOperator},
};

pub struct TreePrinter<'a> {
    arena: &'a ExprArena,
}

impl<'a> TreePrinter<'a> {
    pub fn new(arena: &'a ExprArena) -> Self {
        Self { arena }
    }

    pub fn print(&self, root: ExpressionId) {
        let tree = self.build_tree(root);
        print_tree(&tree).unwrap()
    }

    fn build_tree(&self, root: ExpressionId) -> StringItem {
        let root_node = self.arena.get(root);
        let kind = &root_node.kind;
        let label = Self::get_label(kind);
        let mut builder = TreeBuilder::new(label);
        self.add_child(&mut builder, root);
        builder.build()
    }

    fn get_label(kind: &ExpressionKind) -> String {
        match kind {
            ExpressionKind::Int64(i) => format!("Int64({i})"),
            ExpressionKind::Uint64(u) => format!("Uint64({u})"),
            ExpressionKind::Float64(f) => format!("Float64({f})"),
            ExpressionKind::Bool(b) => format!("Bool({b})"),
            ExpressionKind::Unit => "Unit".to_owned(),

            ExpressionKind::Infix(inf) => match inf.op {
                InfixOperator::Plus => "Addition".to_owned(),
                InfixOperator::Minus => "Subtraction".to_owned(),
                InfixOperator::Mul => "Multiplication".to_owned(),
                InfixOperator::Div => "Division".to_owned(),
            },

            ExpressionKind::Prefix(pref) => match pref.op {
                PrefixOperator::Plus => unreachable!("Prefix plus is stripped during AST creation"),
                PrefixOperator::Minus => "IntegralNegation".to_owned(),
                PrefixOperator::Negate => "BooleanNegation".to_owned(),
            },
        }
    }

    fn add_child(&self, builder: &mut TreeBuilder, id: ExpressionId) {
        let kind = &self.arena.get(id).kind;
        let label = TreePrinter::get_label(kind);
        builder.begin_child(label);

        match kind {
            ExpressionKind::Int64(_)
            | ExpressionKind::Uint64(_)
            | ExpressionKind::Float64(_)
            | ExpressionKind::Bool(_)
            | ExpressionKind::Unit => (),

            ExpressionKind::Infix(node) => {
                self.add_child(builder, node.lhs);
                self.add_child(builder, node.rhs);
            }

            ExpressionKind::Prefix(node) => {
                self.add_child(builder, node.exp);
            }
        };

        builder.end_child();
    }
}
