use actix::Message;
use crate::error::Error;

/// Actix message requests stream values by stream name.
#[derive(Message, Debug, Clone, PartialEq)]
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
