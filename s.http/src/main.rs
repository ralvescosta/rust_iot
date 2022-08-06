mod controllers;
mod errors;
mod middlewares;
mod viewmodels;

use actix_web::{middleware as actix_middleware, web, App, HttpServer};
use controllers::iot_controller;
use infra::{env::Config, logging, otel};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut cfg = Config::new();
    cfg.app_name = "api";
    cfg.app_host = "0.0.0.0";
    cfg.app_port = 3333;

    logging::setup(&cfg);
    otel::tracing::setup(&cfg);

    HttpServer::new(|| {
        App::new()
            //
            // usually register this first
            //
            .wrap(actix_middleware::Compress::default())
            .wrap(middlewares::headers::config())
            .wrap(middlewares::cors::config())
            .app_data(middlewares::deserializer::handler())
            //
            // always register Actix Web Logger middleware last
            //
            .wrap(actix_middleware::Logger::default())
            .service(iot_controller::get)
            //
            // always register default handler the last handler
            //
            .default_service(web::to(middlewares::default::handler))
    })
    .bind(cfg.app_addr())?
    .workers(2)
    .run()
    .await
}
