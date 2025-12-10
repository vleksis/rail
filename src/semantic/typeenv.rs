use std::collections::HashMap;

use super::Type;
use crate::grammar::*;

#[derive(Debug)]
pub struct TypeEnv {
    pub(crate) infix: HashMap<(operator::Infix, Type, Type), Type>,
    pub(crate) prefix: HashMap<(operator::Prefix, Type), Type>,
}

impl TypeEnv {
    pub fn new() -> Self {
        Self {
            infix: Self::default_infix(),
            prefix: Self::default_prefix(),
        }
    }

    fn default_infix() -> HashMap<(operator::Infix, Type, Type), Type> {
        use Type::*;
        use operator::Infix::*;

        let mut infix = HashMap::new();

        let numeric = [Int64, Uint64, Float64];
        for &ty in &numeric {
            infix.insert((Plus, ty, ty), ty);
            infix.insert((Minus, ty, ty), ty);
            infix.insert((Mul, ty, ty), ty);
            infix.insert((Div, ty, ty), ty);
        }

        for ty in [Int64, Uint64, Float64] {
            infix.insert((Equal, ty, ty), Bool);
            infix.insert((NotEqual, ty, ty), Bool);
            infix.insert((Less, ty, ty), Bool);
            infix.insert((LessEqual, ty, ty), Bool);
            infix.insert((Greater, ty, ty), Bool);
            infix.insert((GreaterEqual, ty, ty), Bool);
        }

        infix
    }

    fn default_prefix() -> HashMap<(operator::Prefix, Type), Type> {
        use Type::*;
        use operator::Prefix::*;

        let mut prefix = HashMap::new();

        prefix.insert((Minus, Int64), Int64);
        prefix.insert((Minus, Float64), Float64);

        prefix.insert((Negate, Bool), Bool);

        prefix
    }
}
