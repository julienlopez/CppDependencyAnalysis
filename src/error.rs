#[derive(Debug)]
pub enum Error {
    Redis(redis::RedisError),
    RedisGraph(redisgraph::RedisGraphError),
}

impl From<redis::RedisError> for Error {
    fn from(err: redis::RedisError) -> Error {
        Error::Redis(err)
    }
}

impl From<redisgraph::RedisGraphError> for Error {
    fn from(err: redisgraph::RedisGraphError) -> Error {
        Error::RedisGraph(err)
    }
}
