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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MessageMetadata {
    pub kind: MetadataKind,
    pub topic: String,
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
    fn exec(&self, meta: &MessageMetadata, msg: &Message);
}
