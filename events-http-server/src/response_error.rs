use crate::error::ServiceError;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct ErrorPayload {
    pub error: String,
    pub message: Option<String>,
}

impl From<&ServiceError> for ErrorPayload {
    fn from(error: &ServiceError) -> Self {
        Self {
            error: error.to_string(),
            message: match error {
                ServiceError::BackendError(err) => Some(err.to_string()),
                ServiceError::MailboxError(err) => Some(err.to_string()),
                ServiceError::CacheError(err) => Some(err.to_string()),
            },
        }
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        let payload = ErrorPayload::from(self);
        match self {
            ServiceError::MailboxError(_)
            | ServiceError::BackendError(_)
            | ServiceError::CacheError(_)
                => HttpResponse::InternalServerError().json(payload),
        }
    }
}
