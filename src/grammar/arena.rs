use super::*;

#[derive(Debug, Default)]
pub struct Arena {
    statements: Vec<statement::Node>,
    expressions: Vec<expression::Node>,
    root: statement::Id,
}

impl Arena {
    pub fn set_root(&mut self, root: statement::Id) {
        self.root = root;
    }

    pub fn get_root(&self) -> statement::Id {
        self.root
    }
}

impl Arena {
    pub fn get_expression(&self, id: expression::Id) -> &expression::Node {
        &self.expressions[id.0]
    }

    pub fn expression_count(&self) -> usize {
        self.expressions.len()
    }

    fn push_expression(&mut self, node: expression::Node) -> expression::Id {
        let uid = expression::Id(self.expression_count());
        self.expressions.push(node);
        uid
    }

    pub(crate) fn make_int64(&mut self, i: i64) -> expression::Id {
        let kind = expression::Kind::Int64(i);
        let node = expression::Node { kind };
        self.push_expression(node)
    }

    pub(crate) fn make_uint64(&mut self, u: u64) -> expression::Id {
        let kind = expression::Kind::Uint64(u);
        let node = expression::Node { kind };
        self.push_expression(node)
    }

    pub(crate) fn make_float64(&mut self, f: f64) -> expression::Id {
        let kind = expression::Kind::Float64(f);
        let node = expression::Node { kind };
        self.push_expression(node)
    }

    pub(crate) fn make_bool(&mut self, b: bool) -> expression::Id {
        let kind = expression::Kind::Bool(b);
        let node = expression::Node { kind };
        self.push_expression(node)
    }

    pub(crate) fn make_infix(
        &mut self,
        op: operator::Infix,
        lhs: expression::Id,
        rhs: expression::Id,
    ) -> expression::Id {
        let kind = expression::Kind::Infix { lhs, rhs, op };
        let node = expression::Node { kind };
        self.push_expression(node)
    }

    pub(crate) fn make_prefix(
        &mut self,
        op: operator::Prefix,
        exp: expression::Id,
    ) -> expression::Id {
        let kind = expression::Kind::Prefix { exp, op };
        let node = expression::Node { kind };
        self.push_expression(node)
    }

    pub(crate) fn make_postfix(
        &mut self,
        _op: operator::Postfix,
        _exp: expression::Id,
    ) -> expression::Id {
        unimplemented!()
    }
}

impl Arena {
    pub fn get_statement(&self, id: statement::Id) -> &statement::Node {
        &self.statements[id.0]
    }

    pub fn statement_count(&self) -> usize {
        self.statements.len()
    }

    fn push_statement(&mut self, node: statement::Node) -> statement::Id {
        let uid = statement::Id(self.statement_count());
        self.statements.push(node);
        uid
    }

    pub(crate) fn push_epxression_statement(&mut self, id: expression::Id) -> statement::Id {
        let kind = statement::Kind::Expression(id);
        let node = statement::Node { kind };
        self.push_statement(node)
    }
}
