use actix::Message;
use hitbox_actix::prelude::*;
use crate::error::Error;
use serde::Serialize;

/// Actix message requests stream values by stream name.
#[derive(Message, Debug, Clone, PartialEq, Cacheable, Serialize)]
#[rtype(result = "Result<Vec<String>, Error>")]
pub struct GetStream {
    pub stream: String,
}

impl GetStream {
    pub fn new(stream: String) -> Self {
        Self { stream }
    }
}


/// Actix message writes stream values by stream name.
#[derive(Message, Debug, Clone, PartialEq)]
#[rtype(result = "Result<String, Error>")]
pub struct AddToStream {
    pub stream: String,
    pub value: String,
}

impl AddToStream {
    pub fn new(stream: String, value: String) -> Self {
        Self { stream, value }
    }
}
