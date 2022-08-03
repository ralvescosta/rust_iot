use crate::{errors::iot::HttpError, viewmodels::iot_viewmodel::MyObj};
use actix_web::{get, web, HttpResponse, Result};

#[get("/iot")]
pub async fn get(body: web::Json<MyObj>) -> Result<HttpResponse, HttpError> {
    Ok(HttpResponse::Ok().json(body.0))
}
