// REDIS SINGLE ASYNC CONNECTION IMPL

// use crate::utils::helpers::SerializeError;
// use redis::aio::Connection;
// use redis::{AsyncCommands, Client};

// const TTL: usize = 60 * 5; // TODO externalize to env

// #[derive(Clone)]
// pub struct RedisCacheRs {
//     client: Client,
//     key_prefix: String,
//     ttl: usize,
// }

// impl RedisCacheRs {
//     pub async fn new(connection_url: &String, key_prefix: &String) -> RedisCacheRs {
//         let client = RedisCacheRs::create_client(connection_url).unwrap();
//         RedisCacheRs {
//             client: client,
//             key_prefix: key_prefix.clone(),
//             ttl: TTL,
//         }
//     }

//     fn create_client(host_addr: &String) -> Result<Client, SerializeError> {
//         redis::Client::open(host_addr.to_string())
//             .map_err(|e| SerializeError::new_string(e.to_string()))
//     }

//     async fn create_connection(self: &Self) -> Result<Connection, SerializeError> {
//         self.client
//             .get_async_connection()
//             .await
//             .map_err(|e| SerializeError::new_string(e.to_string()))
//     }

//     fn get_key(self: &Self, key: &str) -> String {
//         format!("{}/{}", self.key_prefix, key)
//     }

//     pub async fn set(self: &Self, key: &str, value: &str) -> Result<(), SerializeError> {
//         let mut con = self.create_connection().await?;
//         let cache_key = self.get_key(key);
//         println!("Trying to set {} with {}", cache_key, value);
//         con.set_ex(cache_key, value, self.ttl)
//             .await
//             .map_err(|e| SerializeError::new_string(e.to_string()))
//     }

//     pub async fn get(self: &Self, key: &str) -> Result<String, SerializeError> {
//         let mut con = self.create_connection().await?;
//         let cache_key = self.get_key(key);
//         println!("Trying to get {}", cache_key);
//         con.get(cache_key)
//             .await
//             .map_err(|e| SerializeError::new_string(e.to_string()))
//     }
// }
