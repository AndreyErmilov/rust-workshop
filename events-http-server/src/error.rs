use thiserror::Error;
use hitbox_actix::CacheError;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error(transparent)]
    MailboxError(#[from] actix::MailboxError),
    #[error(transparent)]
    BackendError(#[from] events_backend::error::Error),
    #[error(transparent)]
    CacheError(#[from] CacheError),
}
