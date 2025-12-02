use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
#[error("lexing error")]
pub struct LexError {}

pub type Result<T> = std::result::Result<T, LexError>;
