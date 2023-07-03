use redis::Client;
use redisgraph::Graph;

pub struct RedisGraphDb {
    client: Client,
    graph: Graph,
}

impl RedisGraphDb {
    pub fn new(ip: &str, graph_name: String) -> Result<RedisGraphDb, crate::error::Error> {
        let client = Client::open(ip)?;
        let connection = client.get_connection()?;
        let graph = Graph::open(connection, graph_name)?;
        Ok(RedisGraphDb { client, graph })
    }
}
