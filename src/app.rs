use crate::connectors::here::HereApi;
use crate::connectors::redis_fred::RedisCache;
use crate::utils::config::Config;

#[derive(Clone)]
pub struct Ctx {
    pub config: Config,
    pub cache: RedisCache,
    pub here_api: HereApi,
}

pub async fn init(config: &Config) -> Ctx {
    let cache = RedisCache::new(&config.redis_connection_url, &config.prefix).await;
    let here_api = HereApi::new(&config.here_api_key);

    Ctx {
        config: config.clone(),
        cache: cache,
        here_api: here_api,
    }
}
