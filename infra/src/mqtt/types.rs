use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum IoTServiceKind {
    Temp,
    GPS,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum MetadataKind {
    IoT(IoTServiceKind),
    Health,
    Log,
}

#[derive(Clone)]
pub struct MessageMetadata {
    pub kind: MetadataKind,
    pub topic: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TempMessage {
    pub temp: f32,
    pub time: u64,
}

pub enum Message {
    Temp(TempMessage),
}

pub type Handler = fn(meta: &MessageMetadata, msg: &Message);
