use crate::error::Error;
use actix::prelude::*;
use redis::{aio::ConnectionManager, Client};
use crate::messages::{GetStream, AddToStream};
use tracing::info;

pub struct RedisBackend {
    connection: ConnectionManager,
}

impl RedisBackend {
    pub async fn new() -> Result<RedisBackend, Error> {
        Self::builder().build().await
    }

    /// Creates new RedisBackend builder with default settings.
    pub fn builder() -> RedisBackendBuilder {
        RedisBackendBuilder::default()
    }
}

/// Part of builder pattern implementation for RedisBackend actor.
pub struct RedisBackendBuilder {
    connection_info: String,
}

impl Default for RedisBackendBuilder {
    fn default() -> Self {
        Self {
            connection_info: "redis://127.0.0.1/".to_owned(),
        }
    }
}

impl RedisBackendBuilder {
    /// Set connection info (host, port, database, etc.) for RedisBackend actor.
    pub fn server(mut self, connection_info: String) -> Self {
        self.connection_info = connection_info;
        self
    }

    /// Create new instance of Redis backend with passed settings.
    pub async fn build(&self) -> Result<RedisBackend, Error> {
        let client = Client::open(self.connection_info.as_str())?;
        let connection = client.get_tokio_connection_manager().await?;
        Ok(RedisBackend { connection })
    }
}

/// Implementation actix Actor trait for Redis backend.
impl Actor for RedisBackend {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        info!("Redis actor started");
    }
}

pub trait Apply<T> {
    fn apply(value: T) -> String;
}

impl Apply<redis::Value> for String {
    fn apply(value: redis::Value) -> String {
        match value {
            | redis::Value::Nil
            | redis::Value::Okay
            | redis::Value::Status(_)
            | redis::Value::Int(_) => String::new(),
            redis::Value::Data(value) => String::from_utf8(value).unwrap(),
            redis::Value::Bulk(mut values) => {
                if let Some(redis::Value::Bulk(mut value)) = values.pop() {
                    if let Some(redis::Value::Data(value)) = value.pop() {
                        String::from_utf8(value).unwrap()
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                }
            },
        }
    }
}

/// Implementation of Redis Actor handler for GetStream message.
impl Handler<GetStream> for RedisBackend {
    type Result = ResponseFuture<Result<Vec<String>, Error>>;

    fn handle(&mut self, msg: GetStream, _: &mut Self::Context) -> Self::Result {
        let mut con = self.connection.clone();
        let fut = async move {
            redis::cmd("XRANGE")
                .arg(msg.stream)
                .arg("-")
                .arg("+")
                .query_async(&mut con)
                .await
                .map(|values: Vec<redis::Value>| values
                    .into_iter()
                    .map(String::apply)
                    .collect::<Vec<_>>()
                )
                .map_err(Error::from)
                .map_err(Error::from)
        };
        Box::pin(fut)
    }
}

/// Implementation of Redis Actor handler for AddToStream message.
impl Handler<AddToStream> for RedisBackend {
    type Result = ResponseFuture<Result<String, Error>>;

    fn handle(&mut self, msg: AddToStream, _: &mut Self::Context) -> Self::Result {
        let mut con = self.connection.clone();
        Box::pin(async move {
            let mut request = redis::cmd("XADD");
            request
                .arg(msg.stream)
                .arg("*")
                .arg("_")
                .arg(msg.value)
                .query_async(&mut con)
                .await
                .map_err(Error::from)
                .map_err(Error::from)
        })
    }
}
