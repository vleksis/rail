use std::collections::HashMap;

use crate::grammar::*;
use crate::semantic::*;

#[derive(Debug, Default)]
pub struct Module {
    pub(crate) syntax: Syntax,
    pub(crate) types: HashMap<expression::Id, Type>,
}
