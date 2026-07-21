use thiserror::Error;

use crate::core::entity::entity::EntityId;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Entity not found")]
    EntityNotFound(EntityId),
}

pub type Result<T> = std::result::Result<T, CoreError>;
