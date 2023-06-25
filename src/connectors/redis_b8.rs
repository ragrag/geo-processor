use std::time::Duration;

use crate::utils::errors::SerializeError;
use bb8_redis::{
    bb8::{Pool, PooledConnection},
    redis::AsyncCommands,
    RedisConnectionManager,
};

pub type BB8Pool = Pool<RedisConnectionManager>;

const TTL: usize = 60 * 120; // TODO externalize to env
const MIN_POOL_IDLE_SIZE: u32 = 64; // TODO externalize to env
const MAX_POOL_SIZE: u32 = 64; // TODO externalize to env
const CONNECTION_TIMEOUT: Duration = Duration::from_secs(5); // TODO externalize to env

#[derive(Clone)]
pub struct RedisCache {
    pool: BB8Pool,
    key_prefix: String,
    ttl: usize,
}

impl RedisCache {
    pub async fn new(connection_url: &String, key_prefix: &String) -> RedisCache {
        let pool = RedisCache::create_pool(connection_url).await;
        RedisCache {
            pool: pool,
            key_prefix: key_prefix.clone(),
            ttl: TTL,
        }
    }

    async fn create_pool(host_addr: &String) -> BB8Pool {
        let manager = RedisConnectionManager::new(host_addr.to_string())
            .expect("unable to create connection manager");
        Pool::builder()
            .min_idle(Some(MIN_POOL_IDLE_SIZE))
            .max_size(MAX_POOL_SIZE)
            .connection_timeout(CONNECTION_TIMEOUT)
            .build(manager)
            .await
            .expect("Unable to establish connection to redis")
    }

    fn get_cache_key(self: &Self, key: &str) -> String {
        format!("{}/{}", self.key_prefix, key)
    }

    pub async fn set(self: &Self, key: &str, value: &str) -> Result<(), SerializeError> {
        let mut con = self.get_connection().await;
        let cache_key = self.get_cache_key(key);
        con.set(cache_key, value)
            .await
            .map_err(|e| SerializeError::new_string(e.to_string()))
    }

    pub async fn get(self: &Self, key: &str) -> Result<String, SerializeError> {
        let mut con = self.get_connection().await;
        let cache_key = self.get_cache_key(key);
        con.get(cache_key)
            .await
            .map_err(|e| SerializeError::new_string(e.to_string()))
    }

    async fn get_connection(self: &Self) -> PooledConnection<'_, RedisConnectionManager> {
        self.pool
            .get()
            .await
            .expect("Unable to get connection from redis pool")
    }
}
