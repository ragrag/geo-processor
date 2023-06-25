use actix_web::middleware::Logger;
use actix_web::{web, web::scope, App, HttpResponse, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;
use routes::{handlers::geocode_handler, handlers::reverse_geocode_handler, middlewares};
use utils::{config, logger};

mod app;
mod connectors;
mod routes;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = config::init();
    let ctx = app::init(&config).await;
    logger::init(config.log_level);

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(middlewares::auth_middleware);
        App::new()
            .app_data(web::Data::new(ctx.clone()))
            .wrap(Logger::default())
            .route("/healthz", web::get().to(|| async { HttpResponse::Ok() }))
            .service(
                scope("")
                    .wrap(auth)
                    .service(geocode_handler)
                    .service(reverse_geocode_handler),
            )
    })
    .bind(format!("0.0.0.0:{}", config.port))?
    .run()
    .await
}
