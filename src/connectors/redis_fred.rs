use crate::utils::errors::SerializeError;
use fred::prelude::*;

const TTL: i64 = 60 * 60 * 24 * 14; // TODO externalize to env

#[derive(Clone)]
pub struct RedisCache {
    client: RedisClient,
    key_prefix: String,
    ttl: i64,
}

impl RedisCache {
    pub async fn new(connection_url: &String, key_prefix: &String) -> RedisCache {
        let config = RedisConfig::from_url(&connection_url).unwrap();
        let perf = PerformanceConfig::default();
        let policy = ReconnectPolicy::new_exponential(0, 100, 30_000, 2);
        let client = RedisClient::new(config, Some(perf), Some(policy));
        let _ = client.connect();
        let _ = client.wait_for_connect().await.unwrap();
        RedisCache {
            client: client,
            key_prefix: key_prefix.clone(),
            ttl: TTL,
        }
    }

    fn get_cache_key(self: &Self, key: &str) -> String {
        format!("{}/{}", self.key_prefix, key)
    }

    pub async fn set(self: &Self, key: &str, value: &str) -> Result<(), SerializeError> {
        let cache_key = self.get_cache_key(key);
        let _: () = self
            .client
            .set(
                cache_key,
                value,
                Some(Expiration::EX(self.ttl)),
                Some(SetOptions::NX),
                false,
            )
            .await
            .unwrap();

        Ok(())
    }

    pub async fn get(self: &Self, key: &str) -> Result<String, SerializeError> {
        let cache_key = self.get_cache_key(key);
        self.client
            .get(cache_key)
            .await
            .map_err(|e| SerializeError::new_string(e.to_string()))
    }
}
