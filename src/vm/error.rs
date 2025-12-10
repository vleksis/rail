use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("stack underflow")]
    StackUnderflow,
    #[error("type mismatch: {0}")]
    TypeMismatch(&'static str),
    #[error("global not defined: {0}")]
    GlobalNotDefined(u16),
    #[error("invalid jump target")]
    InvalidJumpTarget,
    #[error("expected OpCode")]
    InvalidOpCode,
}

pub type Result<T> = std::result::Result<T, Error>;
