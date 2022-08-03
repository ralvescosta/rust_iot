mod controllers;
mod errors;
mod middlewares;
mod viewmodels;

use actix_web::{middleware as actix_middleware, web, App, HttpServer};
use controllers::iot_controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
