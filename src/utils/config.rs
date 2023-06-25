use std::env;

#[derive(Clone)]
pub struct Config {
    pub port: String,
    pub redis_connection_url: String,
    pub prefix: String,
    pub here_api_key: String,
    pub api_key: String,
    pub log_level: String,
}

pub fn init() -> Config {
    Config {
        port: env::var("PORT").expect("missing env var PORT"),
        prefix: env::var("PREFIX").expect("missing env var PREFIX"),
        redis_connection_url: env::var("REDIS_CONNECTION_URL")
            .expect("missing env var REDIS_CONNECTION_URL"),
        here_api_key: env::var("HERE_API_KEY").expect("missing env var HERE_API_KEY"),
        api_key: env::var("API_KEY").expect("missing env var API_TOKEN_SECRET"),
        log_level: env::var("LOG_LEVEL").unwrap_or(String::from("info")),
    }
}
