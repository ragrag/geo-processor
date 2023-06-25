use crate::connectors::here::GeocodeParsedResult;
use crate::services::common::Hashable;
use crate::{app::Ctx, utils::errors::SerializeError};
use sha2::{Digest, Sha256};

pub async fn geocode(address: Address, ctx: &Ctx) -> Result<GeocodeParsedResult, SerializeError> {
    let hash = address.sha_hash();
    let cache_result = ctx.cache.get(&address.sha_hash()).await;
    println!("cache_result: {:?}", cache_result);
    match cache_result {
        Ok(r) if r != "nil" => Ok(serde_json::from_str(&r).unwrap()),
        _ => {
            let here_result = ctx.here_api.geocode(&address).await;
            match here_result {
                Ok(r) => {
                    ctx.cache
                        .set(&hash, &serde_json::to_string(&r).unwrap())
                        .await
                        .expect("Failed to set cache");
                    return Ok(r);
                }
                Err(e) => Err(e),
            }
        }
    }
}

pub struct Address {
    pub query: String,
}

impl Address {
    pub fn new(address: &String) -> Self {
        Self {
            query: address.clone(),
        }
    }
}

impl Hashable for Address {
    fn sha_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.query.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
