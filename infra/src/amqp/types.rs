use crate::{errors::AmqpError, otel};
use lapin::types::FieldTable;
use opentelemetry::Context;
use serde::Serialize;

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
    ///traceparent is compos from {trace-version}-{trace-id}-{parent-id}-{trace-flags}
    pub traceparent: String,
}

impl PublishData {
    pub fn new<T>(ctx: &Context, payload: T) -> Result<Self, AmqpError>
    where
        T: PublishPayload + Serialize,
    {
        let serialized = serde_json::to_vec::<T>(&payload)
            .map_err(|_| AmqpError::ParsePayloadError {})?
            .into_boxed_slice();

        Ok(PublishData {
            msg_type: payload.get_type(),
            payload: serialized,
            traceparent: otel::amqp::Traceparent::string_from_ctx(ctx),
        })
    }
}
