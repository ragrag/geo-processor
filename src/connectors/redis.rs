use std::time::Duration;

use crate::utils::helpers::SerializeError;
use bb8_redis::{bb8::Pool, redis::AsyncCommands, RedisConnectionManager};

pub type BB8Pool = Pool<RedisConnectionManager>;

const TTL: usize = 60 * 5; // TODO externalize to env
const MAX_POOL_SIZE: u32 = 60; // TODO externalize to env
const CONNECTION_TIMEOUT: Duration = Duration::from_secs(10); // TODO externalize to env

#[derive(Clone)]
pub struct RedisCache {
    pool: BB8Pool,
    key_prefix: String,
    ttl: usize,
}

impl RedisCache {
    pub async fn new(connection_url: &String, key_prefix: &String) -> RedisCache {
        let pool = RedisCache::create_pool(connection_url).await.unwrap();
        RedisCache {
            pool: pool,
            key_prefix: key_prefix.clone(),
            ttl: TTL,
        }
    }

    async fn create_pool(host_addr: &String) -> Result<BB8Pool, SerializeError> {
        let manager = RedisConnectionManager::new(host_addr.to_string())
            .map_err(|e| SerializeError::new_string(e.to_string()))?;
        Pool::builder()
            .max_size(MAX_POOL_SIZE)
            .connection_timeout(CONNECTION_TIMEOUT)
            .build(manager)
            .await
            .map_err(|e| SerializeError::new_string(e.to_string()))
    }

    fn get_key(self: &Self, key: &str) -> String {
        format!("{}/{}", self.key_prefix, key)
    }

    pub async fn set(self: &Self, key: &str, value: &str) -> Result<(), SerializeError> {
        let mut con = self
            .pool
            .get()
            .await
            .map_err(|e| SerializeError::new_string(e.to_string()))?;
        let cache_key = self.get_key(key);
        println!("Trying to set {} with {}", cache_key, value);
        con.set_ex(cache_key, value, self.ttl)
            .await
            .map_err(|e| SerializeError::new_string(e.to_string()))
    }

    pub async fn get(self: &Self, key: &str) -> Result<String, SerializeError> {
        let mut con = self
            .pool
            .get()
            .await
            .map_err(|e| SerializeError::new_string(e.to_string()))?;
        let cache_key = self.get_key(key);
        println!("Trying to get {}", cache_key);
        con.get(cache_key)
            .await
            .map_err(|e| SerializeError::new_string(e.to_string()))
    }
}
