pub mod error;
pub mod redis_graph_db;

use redis_graph_db::RedisGraphDb;

fn main() {
    println!("Hello, world!");

    let redis = RedisGraphDb::new("http://127.0.0.1", "MySoftware".to_string());
}
