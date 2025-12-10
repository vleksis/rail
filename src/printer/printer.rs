use ptree::{TreeBuilder, item::StringItem, print_tree};

use crate::grammar::*;

pub struct TreePrinter<'s> {
    syntax: &'s Syntax,
    builder: TreeBuilder,
}

impl<'s> TreePrinter<'s> {
    pub fn new(syntax: &'s Syntax) -> Self {
        Self {
            syntax,
            builder: TreeBuilder::new("ROOT".to_owned()),
        }
    }

    pub fn print(mut self) {
        let root = self.syntax.arena.get_root();
        self.add_statement(root);
        let tree = self.builder.build();
        print_tree(&tree).unwrap();
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

    fn add_expression(&mut self, id: expression::Id) {
        use expression::Kind::*;

        let kind = &self.syntax.arena.get_expression(id).kind;
        let label = TreePrinter::get_label(kind);
        self.builder.begin_child(label);

        match kind {
            Int64(_) | Uint64(_) | Float64(_) | Bool(_) | Unit => (),

            Infix { lhs, rhs, op: _ } => {
                self.add_expression(*lhs);
                self.add_expression(*rhs);
            }

            Prefix { exp, op: _ } => {
                self.add_expression(*exp);
            }
        };

        self.builder.end_child();
    }

    fn add_statement(&mut self, id: statement::Id) {
        use statement::Kind::*;

        let kind = &self.syntax.arena.get_statement(id).kind;
        self.builder.begin_child("statement".to_owned());

        match kind {
            Expression(exp) => self.add_expression(*exp),
            _ => unimplemented!(),
        };

        self.builder.end_child();
    }
}
