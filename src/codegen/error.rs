use thiserror::Error;

#[derive(Debug, Error)]
#[error("Codegen error")]
pub struct Error {}

pub type Result<T> = std::result::Result<T, Error>;
