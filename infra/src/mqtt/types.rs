use async_trait::async_trait;
use bytes::Bytes;
use log::error;
#[cfg(test)]
use mockall::automock;
use opentelemetry::Context;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::errors::MqttError;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum IoTServiceKind {
    Temp,
    GPS,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum MetadataKind {
    IoT(IoTServiceKind),
    Health,
    Log,
}

impl Display for MetadataKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MessageMetadata {
    pub kind: MetadataKind,
    pub topic: String,
}

impl Display for MessageMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind.to_string())
    }
}

impl MessageMetadata {
    pub fn from_topic(topic: String) -> Result<MessageMetadata, MqttError> {
        let splitted = topic.split("/").collect::<Vec<&str>>();
        if splitted.len() < 3 && splitted[0] != "iot" {
            error!("unformatted topic");
            return Err(MqttError::UnformattedTopicError {});
        }

        match splitted[2] {
            "temp" => {
                if splitted.len() < 4 {
                    error!("wrong temp topic");
                    return Err(MqttError::UnformattedTopicError {});
                }

                Ok(MessageMetadata {
                    kind: MetadataKind::IoT(IoTServiceKind::Temp),
                    topic: topic.clone(),
                })
            }
            _ => {
                error!("unknown message kind");
                return Err(MqttError::UnknownMessageKindError {});
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TempMessage {
    pub temp: f32,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Message {
    Temp(TempMessage),
}

impl Message {
    pub fn from_payload(kind: &MetadataKind, payload: &Bytes) -> Result<Message, MqttError> {
        match kind {
            MetadataKind::IoT(IoTServiceKind::Temp) => {
                let msg = serde_json::from_slice::<TempMessage>(payload);
                if msg.is_err() {
                    error!("msg conversion error - {:?}", msg);
                    return Err(MqttError::InternalError {});
                }

                Ok(Message::Temp(msg.unwrap()))
            }
            _ => {
                error!("unknown message kind");
                return Err(MqttError::UnknownMessageKindError {});
            }
        }
    }
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait Controller {
    async fn exec(
        &self,
        ctx: &Context,
        meta: &MessageMetadata,
        msg: &Message,
    ) -> Result<(), MqttError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_metadata_successfully() {
        let res = MessageMetadata::from_topic("iot/data/temp/device_id/location".to_owned());
        assert!(res.is_ok());

        let res = res.unwrap();
        let kind = res.kind;
        assert_eq!(kind, MetadataKind::IoT(IoTServiceKind::Temp));
    }

    #[test]
    fn should_get_metadata_err() {
        let res = MessageMetadata::from_topic("iot/data/temp".to_owned());
        assert!(res.is_err());

        let res = MessageMetadata::from_topic("wrong/data/temp".to_owned());
        assert!(res.is_err());

        let res = MessageMetadata::from_topic("iot/data/unknown/device_id/location".to_owned());
        assert!(res.is_err());
    }

    #[test]
    fn should_get_message_successfully() {
        let res = Message::from_payload(
            &MetadataKind::IoT(IoTServiceKind::Temp),
            &Bytes::try_from("{\"temp\": 39.9, \"time\": 99999999}").unwrap(),
        );
        assert!(res.is_ok());
    }

    #[test]
    fn should_get_message_err() {
        let res = Message::from_payload(
            &MetadataKind::IoT(IoTServiceKind::Temp),
            &Bytes::try_from("").unwrap(),
        );
        assert!(res.is_err());

        let res = Message::from_payload(
            &MetadataKind::IoT(IoTServiceKind::GPS),
            &Bytes::try_from("{\"temp\": 39.9, \"time\": 99999999}").unwrap(),
        );
        assert!(res.is_err());
    }
}
