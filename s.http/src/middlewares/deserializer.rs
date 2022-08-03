use actix_web::{
    error::{InternalError, JsonPayloadError},
    web::JsonConfig,
    HttpRequest, HttpResponse,
};

use crate::viewmodels::http_error::HttpError;

pub fn handler() -> JsonConfig {
    JsonConfig::default().error_handler(|err: JsonPayloadError, _req: &HttpRequest| {
        InternalError::from_response(
            format!("JSON error: {:?}", err),
            HttpResponse::BadRequest().json(HttpError {
                status_code: 400,
                message: String::from("Wrong body format"),
                details: format!("{}", err),
            }),
        )
        .into()
    })
}
