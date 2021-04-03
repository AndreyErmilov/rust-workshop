use actix::Message;
use crate::error::Error;

/// Actix message requests cache backend value by key.
#[derive(Message, Debug, Clone, PartialEq)]
#[rtype(result = "Result<Vec<String>, Error>")]
pub struct Get {
    pub stream: String,
}

impl Get {
    pub fn new(stream: String) -> Self {
        Self { stream }
    }
}


/// Actix message writes cache backend value by key.
#[derive(Message, Debug, Clone, PartialEq)]
#[rtype(result = "Result<String, Error>")]
pub struct Add {
    pub stream: String,
    pub key: String,
    pub value: String,
}

impl Add {
    pub fn new(stream: String, key: String, value: String) -> Self {
        Self { stream, key, value }
    }
}
