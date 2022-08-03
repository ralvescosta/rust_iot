use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("internal error")]
    InternalError,

    #[error("bad request")]
    BadRequest,

    #[error("timeout")]
    Timeout,
}

impl error::ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            HttpError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            HttpError::BadRequest => StatusCode::BAD_REQUEST,
            HttpError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}
