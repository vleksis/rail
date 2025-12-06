use ptree::TreeItem;

#[derive(Debug)]
pub enum ExpressionNode {
    Int64(i64),
    Uint64(u64),
    Float64(f64),
    Bool(bool),
    Unit,

    Addition(AdditionNode),
    Subtraction(SubtractionNode),
    Multiplication(MultiplicationNode),
    Division(DivisionNode),

    IntegralNegation(IntegralNegationNode),
    BooleanNegation(BooleanNegationNode),
}

#[derive(Debug)]
pub struct AdditionNode {
    pub(crate) lhs: Box<ExpressionNode>,
    pub(crate) rhs: Box<ExpressionNode>,
}

#[derive(Debug)]
pub struct SubtractionNode {
    pub(crate) lhs: Box<ExpressionNode>,
    pub(crate) rhs: Box<ExpressionNode>,
}

#[derive(Debug)]
pub struct MultiplicationNode {
    pub(crate) lhs: Box<ExpressionNode>,
    pub(crate) rhs: Box<ExpressionNode>,
}

#[derive(Debug)]
pub struct DivisionNode {
    pub(crate) lhs: Box<ExpressionNode>,
    pub(crate) rhs: Box<ExpressionNode>,
}

#[derive(Debug)]
pub struct IntegralNegationNode {
    pub(crate) exp: Box<ExpressionNode>,
}

#[derive(Debug)]
pub struct BooleanNegationNode {
    pub(crate) exp: Box<ExpressionNode>,
}

impl<'s> TreeItem for &'s ExpressionNode {
    type Child = &'s ExpressionNode;

    fn write_self<W: std::io::Write>(
        &self,
        f: &mut W,
        _style: &ptree::Style,
    ) -> std::io::Result<()> {
        use std::io::Write as _;
        let label = match self {
            ExpressionNode::Int64(v) => format!("{}", v),
            ExpressionNode::Uint64(v) => format!("{}", v),
            ExpressionNode::Float64(v) => format!("{}", v),
            ExpressionNode::Bool(b) => format!("{}", b),
            ExpressionNode::Unit => "()".to_string(),

            ExpressionNode::Addition(_) => "Addition".to_string(),
            ExpressionNode::Subtraction(_) => "Subtraction".to_string(),
            ExpressionNode::Multiplication(_) => "Multiplication".to_string(),
            ExpressionNode::Division(_) => "Division".to_string(),

            ExpressionNode::IntegralNegation(_) => "IntegralNegation".to_string(),
            ExpressionNode::BooleanNegation(_) => "BooleanNegation".to_string(),
        };
        write!(f, "{}", label)
    }

    fn children(&self) -> std::borrow::Cow<[Self::Child]> {
        use std::borrow::Cow;

        let v: Vec<&ExpressionNode> = match self {
            ExpressionNode::Addition(n) => vec![&n.lhs, &n.rhs],
            ExpressionNode::Subtraction(n) => vec![&n.lhs, &n.rhs],
            ExpressionNode::Multiplication(n) => vec![&n.lhs, &n.rhs],
            ExpressionNode::Division(n) => vec![&n.lhs, &n.rhs],

            ExpressionNode::IntegralNegation(n) => vec![&n.exp],
            ExpressionNode::BooleanNegation(n) => vec![&n.exp],

            ExpressionNode::Int64(_)
            | ExpressionNode::Uint64(_)
            | ExpressionNode::Float64(_)
            | ExpressionNode::Bool(_)
            | ExpressionNode::Unit => Vec::new(),
        };

        Cow::Owned(v)
    }
}
