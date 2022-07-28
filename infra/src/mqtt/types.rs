use std::fmt::Display;

#[cfg(test)]
use mockall::automock;

use serde::{Deserialize, Serialize};

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
pub trait IController {
    fn exec(&self, meta: &MessageMetadata, msg: &Message) -> Result<(), ()>;
}
