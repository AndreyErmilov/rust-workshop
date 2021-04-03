use redis::RedisError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Redis backend error: {0}")]
    Redis(RedisError),
}

impl From<RedisError> for Error {
    fn from(error: RedisError) -> Self {
        Error::Redis(error)
    }
}
