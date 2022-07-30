use lapin::types::FieldTable;
use serde::Serialize;

use crate::errors::AmqpError;

#[derive(Debug)]
pub struct Metadata {
    pub count: i64,
    pub traceparent: String,
}

impl Metadata {
    pub fn extract(header: &FieldTable) -> Metadata {
        let count = match header.inner().get("x-death") {
            Some(value) => match value.as_array() {
                Some(arr) => match arr.as_slice().get(0) {
                    Some(value) => match value.as_field_table() {
                        Some(table) => match table.inner().get("count") {
                            Some(value) => match value.as_long_long_int() {
                                Some(long) => long,
                                _ => 0,
                            },
                            _ => 0,
                        },
                        _ => 0,
                    },
                    _ => 0,
                },
                _ => 0,
            },
            _ => 0,
        };

        let traceparent = match header.inner().get("traceparent") {
            Some(value) => match value.as_long_string() {
                Some(st) => st.to_string(),
                _ => "".to_owned(),
            },
            _ => "".to_owned(),
        };

        Metadata { count, traceparent }
    }
}

pub trait PublishPayload {
    fn get_type(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct PublishData {
    pub payload: Box<[u8]>,
    pub msg_type: String,
    pub traceparent: String,
}

impl PublishData {
    pub fn new<T>(d: T, span: String) -> Result<Self, AmqpError>
    where
        T: PublishPayload + Serialize,
    {
        let payload = serde_json::to_vec::<T>(&d)
            .map_err(|_| AmqpError::ParsePayloadError {})?
            .into_boxed_slice();

        Ok(PublishData {
            msg_type: d.get_type(),
            payload,
            traceparent: span,
        })
    }
}
