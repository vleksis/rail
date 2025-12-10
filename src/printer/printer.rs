use ptree::{TreeBuilder, item::StringItem, print_tree};

use crate::grammar::*;

pub struct TreePrinter<'s> {
    syntax: &'s Syntax,
}

impl<'s> TreePrinter<'s> {
    pub fn new(syntax: &'s Syntax) -> Self {
        Self { syntax }
    }

    pub fn print_tree(&self) {
        let root = self.syntax.arena.get_root();
        self.print_expr(root);
    }

    fn print_expr(&self, root: expression::Id) {
        let tree = self.build_tree(root);
        print_tree(&tree).unwrap()
    }

    fn build_tree(&self, root: expression::Id) -> StringItem {
        let mut builder = TreeBuilder::new("ROOT".to_string());
        self.add_child(&mut builder, root);
        builder.build()
    }

    fn get_label(kind: &expression::Kind) -> String {
        match kind {
            expression::Kind::Int64(i) => format!("Int64({i})"),
            expression::Kind::Uint64(u) => format!("Uint64({u})"),
            expression::Kind::Float64(f) => format!("Float64({f})"),
            expression::Kind::Bool(b) => format!("Bool({b})"),
            expression::Kind::Unit => "Unit".to_owned(),
            expression::Kind::Infix { op, lhs: _, rhs: _ } => op.to_string(),
            expression::Kind::Prefix { op, exp: _ } => op.to_string(),
        }
    }

    fn add_child(&self, builder: &mut TreeBuilder, id: expression::Id) {
        let kind = &self.syntax.arena.get(id).kind;
        let label = TreePrinter::get_label(kind);
        builder.begin_child(label);

        match kind {
            expression::Kind::Int64(_)
            | expression::Kind::Uint64(_)
            | expression::Kind::Float64(_)
            | expression::Kind::Bool(_)
            | expression::Kind::Unit => (),

            expression::Kind::Infix { lhs, rhs, op: _ } => {
                self.add_child(builder, *lhs);
                self.add_child(builder, *rhs);
            }

            expression::Kind::Prefix { exp, op: _ } => {
                self.add_child(builder, *exp);
            }
        };

        builder.end_child();
    }
}
