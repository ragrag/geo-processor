use crate::connectors::here::GeocodeParsedResult;
use crate::{app::Ctx, services::common::Hashable, utils::errors::SerializeError};
use sha2::{Digest, Sha256};

pub async fn reverse_geocode(
    coords: Coords,
    ctx: &Ctx,
) -> Result<GeocodeParsedResult, SerializeError> {
    let hash = coords.sha_hash();
    let cache_result = ctx.cache.get(&hash).await;
    match cache_result {
        Ok(r) if r != "nil" => Ok(serde_json::from_str(&r).unwrap()),
        _ => {
            let here_result = ctx.here_api.reverse_geocode(&coords).await;
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

pub struct Coords {
    pub lat: String,
    pub lng: String,
}

impl Coords {
    pub fn new(lat: &f64, lng: &f64) -> Self {
        Self {
            lat: format!("{:.7}", lat.clone()),
            lng: format!("{:.7}", lng.clone()),
        }
    }
}

impl Hashable for Coords {
    fn sha_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.lat.as_bytes());
        hasher.update(self.lng.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
