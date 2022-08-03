use serde::Serialize;

#[derive(Serialize)]
pub struct HttpError {
    pub status_code: i32,
    pub message: String,
    pub details: String,
}
