use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error(transparent)]
    MailboxError(#[from] actix::MailboxError),
    #[error(transparent)]
    BackendError(#[from] events_backend::error::Error),
}
