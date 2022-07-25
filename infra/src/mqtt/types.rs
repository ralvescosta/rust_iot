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

#[derive(Clone, Debug)]
pub struct MessageMetadata {
    pub kind: MetadataKind,
    pub topic: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TempMessage {
    pub temp: f32,
    pub time: u64,
}

#[derive(Clone, Debug)]
pub enum Message {
    Temp(TempMessage),
}

pub trait IController {}

pub type Handler = fn(meta: &MessageMetadata, msg: &Message);
