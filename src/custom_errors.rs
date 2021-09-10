use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomError {
    #[error("Attempting to add component to an entity without creating component first")]
    CreateComponentNeverCalled,

    #[error("Attempted to reference a component that wasn't registered")]
    ComponentNotRegistered,

    #[error("Attempted to reference an entity that doesn't exist")]
    EntityDoesNotExist,
}
