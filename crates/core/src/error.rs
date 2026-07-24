use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("{0} not found")]
    NotFound(String),
}

pub type Result<T> = std::result::Result<T, CoreError>;
