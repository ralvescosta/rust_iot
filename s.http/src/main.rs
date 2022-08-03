mod controllers;
mod viewmodels;

use actix_web::{App, HttpServer};
use controllers::iot_controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(iot_controller::get))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
