use crate::runtime::function::Function;

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
    // pub native: Vec<_>,
    pub entry: usize,
}

impl Program {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            entry: 0,
        }
    }
}
