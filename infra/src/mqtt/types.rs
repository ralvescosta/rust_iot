use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
use opentelemetry::Context;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

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
        write!(f, "{}", self)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MessageMetadata {
    pub kind: MetadataKind,
    pub topic: String,
}

impl Display for MessageMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TempMessage {
    pub temp: f32,
    pub time: u64,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Message {
    Temp(TempMessage),
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait IController {
    async fn exec(&self, ctx: &Context, meta: &MessageMetadata, msg: &Message) -> Result<(), ()>;
}
