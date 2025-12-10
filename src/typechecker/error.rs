use thiserror::Error;

#[derive(Debug, Error)]
#[error("Type Error during ast construction")]
pub struct Error {}

pub type Result<T> = std::result::Result<T, Error>;
