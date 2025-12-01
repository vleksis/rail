use thiserror::Error;

use crate::lexer::token::SourceSpan;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum LexErrorKind {
    UnexpectedChar(char),
    InvalidNumber,
}

#[derive(Debug, Clone, PartialEq, Error)]
#[error("lexing error")]
pub struct LexError {}

pub type Result<T> = std::result::Result<T, LexError>;
