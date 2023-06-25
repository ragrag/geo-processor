use crate::services;
use crate::services::reverse_geocode::Coords;
use crate::utils::errors::SerializeError;
use async_std::task;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use serde::{Deserialize, Serialize};
use services::geocode::Address;
use std::time::Duration;

const MAX_REQ_RETRIES: u32 = 2;

#[derive(Clone)]
pub struct HereApi {
    client: ClientWithMiddleware,
    here_api_key: String,
}

impl HereApi {
    pub fn new(here_api_key: &String) -> HereApi {
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(MAX_REQ_RETRIES);
        let client = ClientBuilder::new(reqwest::Client::new())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        HereApi {
            client: client,
            here_api_key: here_api_key.clone(),
        }
    }

    pub async fn geocode(
        self: &Self,
        address: &Address,
    ) -> Result<GeocodeParsedResult, SerializeError> {
        self.retryable_request(
            "https://geocode.search.hereapi.com/v1/geocode",
            &[("apiKey", &self.here_api_key), ("q", &address.query)],
        )
        .await
    }

    pub async fn reverse_geocode(
        self: &Self,
        coords: &Coords,
    ) -> Result<GeocodeParsedResult, SerializeError> {
        self.retryable_request(
            "https://geocode.search.hereapi.com/v1/revgeocode",
            &[
                ("apiKey", &self.here_api_key),
                ("at", &format!("{},{}", &coords.lat, &coords.lng)),
            ],
        )
        .await
    }

    // retries requests when retry-after header is supplied, otherwise falls back to ExponentialBackoff retry policy
    async fn retryable_request(
        self: &Self,
        url: &str,
        query: &[(&str, &String)],
    ) -> Result<GeocodeParsedResult, SerializeError> {
        let mut retry_count = 0;

        let result = loop {
            if retry_count >= MAX_REQ_RETRIES {
                break Err(SerializeError {
                    msg: "Maximum number of retries reached".to_string(),
                });
            }

            let resp = self.client.get(url).query(&query).send().await;

            match resp {
                Ok(r) => match r.status().is_success() {
                    true => {
                        let res_text = r.text().await.unwrap().to_string();
                        let res_parsed: GeocodeFullResult =
                            serde_json::from_str(&res_text).unwrap();

                        let items_with_geodata: Vec<GeocodeRawResult> = res_parsed
                            .items
                            .into_iter()
                            .filter(|it| {
                                if let (Some(_add), Some(_pos)) = (&it.address, &it.position) {
                                    return true;
                                } else {
                                    return false;
                                }
                            })
                            .collect();

                        if items_with_geodata.len() == 0 {
                            break Ok(self.get_result(None));
                        }

                        let max_qs_item = self.get_max_qs_item(items_with_geodata);
                        break Ok(self.get_result(Some(max_qs_item)));
                    }
                    false => {
                        if r.status().as_u16() == 429 && r.headers().contains_key("retry-after") {
                            let retry_after =
                                r.headers().get("retry-after").unwrap().to_str().unwrap();
                            let retry_after = retry_after.parse::<u64>().unwrap() * 1000;
                            task::sleep(Duration::from_secs(retry_after)).await;
                        } else {
                            break Err(SerializeError {
                                msg: r.text().await.unwrap(),
                            });
                        }
                    }
                },
                Err(e) => break Err(SerializeError { msg: e.to_string() }),
            }
            retry_count += 1;
        };

        return result;
    }

    fn get_max_qs_item(self: &Self, mut items: Vec<GeocodeRawResult>) -> GeocodeRawResult {
        let mut max_qs_idx: usize = 0;
        for (idx, item) in items.iter().enumerate() {
            let cur_score = item.scoring.as_ref().unwrap().queryScore;
            let max_score = items[max_qs_idx].scoring.as_ref().unwrap().queryScore;
            if cur_score > max_score {
                max_qs_idx = idx;
            }
        }
        items.remove(max_qs_idx)
    }

    fn get_result(self: &Self, geocode_obj: Option<GeocodeRawResult>) -> GeocodeParsedResult {
        match geocode_obj {
            Some(v) => {
                let address = v.address.unwrap();
                let position = v.position.unwrap();
                GeocodeParsedResult {
                    quarter: address.district,
                    street: address.street,
                    houseNumber: address.houseNumber,
                    zipcode: address.postalCode,
                    latitude: position.lat,
                    longitude: position.lng,
                    city: address.city,
                    state: address.state,
                    county: address.county,
                    country: address.countryName,
                }
            }
            None => GeocodeParsedResult {
                quarter: None,
                street: None,
                houseNumber: None,
                zipcode: None,
                latitude: None,
                longitude: None,
                city: None,
                state: None,
                county: None,
                country: None,
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GeocodeParsedResult {
    quarter: Option<String>,
    street: Option<String>,
    houseNumber: Option<String>,
    zipcode: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    city: Option<String>,
    state: Option<String>,
    county: Option<String>,
    country: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct GeocodeFullResult {
    items: Vec<GeocodeRawResult>,
}

#[derive(Serialize, Deserialize)]
struct GeocodeRawResult {
    title: Option<String>,
    id: Option<String>,
    resultType: Option<String>,
    houseNumberType: Option<String>,
    address: Option<GeocodeRawAddressResult>,
    position: Option<GeocodeRawCoordsResult>,
    access: Option<Vec<GeocodeRawCoordsResult>>,
    mapView: Option<GeocodeRawMapViewResult>,
    scoring: Option<GeocodeRawScoringResult>,
}

#[derive(Serialize, Deserialize)]
struct GeocodeRawAddressResult {
    label: Option<String>,
    countryCode: Option<String>,
    countryName: Option<String>,
    stateCode: Option<String>,
    state: Option<String>,
    county: Option<String>,
    city: Option<String>,
    district: Option<String>,
    street: Option<String>,
    postalCode: Option<String>,
    houseNumber: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct GeocodeRawCoordsResult {
    lat: Option<f64>,
    lng: Option<f64>,
}

#[derive(Serialize, Deserialize)]
struct GeocodeRawMapViewResult {
    west: Option<f64>,
    south: Option<f64>,
    east: Option<f64>,
    north: Option<f64>,
}

#[derive(Serialize, Deserialize)]
struct GeocodeRawScoringResult {
    queryScore: Option<f64>,
    fieldScore: Option<GeocodeRawFieldScoringResult>,
    country: Option<f64>,
    city: Option<f64>,
    streets: Option<Vec<f64>>,
    houseNumber: Option<f64>,
    postalCode: Option<f64>,
}

#[derive(Serialize, Deserialize)]
struct GeocodeRawFieldScoringResult {
    country: Option<f64>,
    city: Option<f64>,
    streets: Option<Vec<f64>>,
    houseNumber: Option<f64>,
    postalCode: Option<f64>,
}
