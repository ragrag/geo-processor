use actix_web::{error::ErrorBadRequest, get, web, Responder, Result};
use serde::Deserialize;

use crate::app::Ctx;
use crate::services;

use services::{
    geocode::{geocode, Address},
    reverse_geocode::{reverse_geocode, Coords},
};

#[derive(Deserialize)]
pub struct GeocodeReqQueryParams {
    address: String,
}

#[get("/geocode")]
pub async fn geocode_handler(
    ctx: web::Data<Ctx>,
    query: web::Query<GeocodeReqQueryParams>,
) -> Result<impl Responder> {
    let geocode_result = geocode(Address::new(&query.address), &ctx).await;
    match geocode_result {
        Ok(r) => Ok(web::Json(r)),
        Err(e) => Err(ErrorBadRequest(e.msg.to_string())),
    }
}

#[derive(Deserialize)]
pub struct ReverseGeocodeReqQueryParams {
    lat: f64,
    lng: f64,
}

#[get("/reverse-geocode")]
pub async fn reverse_geocode_handler(
    ctx: web::Data<Ctx>,
    query: web::Query<ReverseGeocodeReqQueryParams>,
) -> Result<impl Responder> {
    let rev_geocode_result = reverse_geocode(Coords::new(&query.lat, &query.lng), &ctx).await;
    match rev_geocode_result {
        Ok(r) => Ok(web::Json(r)),
        Err(e) => Err(ErrorBadRequest(e.msg.to_string())),
    }
}
