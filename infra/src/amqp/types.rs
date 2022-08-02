use crate::errors::AmqpError;
use lapin::types::FieldTable;
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
}

impl PublishData {
    pub fn new<T>(payload: T) -> Result<Self, AmqpError>
    where
        T: PublishPayload + Serialize,
    {
        let serialized = serde_json::to_vec::<T>(&payload)
            .map_err(|_| AmqpError::ParsePayloadError {})?
            .into_boxed_slice();

        Ok(PublishData {
            msg_type: payload.get_type(),
            payload: serialized,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use lapin::types::{AMQPValue, FieldArray, FieldTable, LongLongInt, LongString, ShortString};

    #[test]
    fn test_metadata_extract_successfully() {
        let mut count = BTreeMap::new();
        count.insert(ShortString::from("count"), AMQPValue::LongLongInt(10));

        let mut metadata = BTreeMap::new();
        metadata.insert(
            ShortString::from("x-death"),
            AMQPValue::FieldArray(FieldArray::from(vec![AMQPValue::FieldTable(
                FieldTable::from(count),
            )])),
        );

        metadata.insert(
            ShortString::from("traceparent"),
            AMQPValue::LongString(LongString::from("traceparent")),
        );

        let re = Metadata::extract(&FieldTable::from(metadata));
        assert_eq!(re.count, 10);
        assert_eq!(re.traceparent, "traceparent");
    }

    #[test]
    fn test_metadata_extract_wrong() {
        let mut metadata = BTreeMap::new();

        let re = Metadata::extract(&FieldTable::from(metadata.clone()));
        assert_eq!(re.count, 0);
        assert_eq!(re.traceparent, "");

        let mut count = BTreeMap::new();
        count.insert(ShortString::from("c"), AMQPValue::LongLongInt(10));
        metadata.insert(
            ShortString::from("x-death"),
            AMQPValue::FieldArray(FieldArray::from(vec![AMQPValue::FieldTable(
                FieldTable::from(count),
            )])),
        );
        metadata.insert(
            ShortString::from("traceparent"),
            AMQPValue::LongLongInt(LongLongInt::from(10)),
        );
        let re = Metadata::extract(&FieldTable::from(metadata.clone()));
        assert_eq!(re.count, 0);
        assert_eq!(re.traceparent, "");
    }
}
