use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IoTTempViewModel {
    pub temp: f32,
    pub time: u64,
}

#[derive(Serialize, Deserialize)]
pub enum IoTMessageKind {
    Temp,
}

#[derive(Serialize, Deserialize)]
pub struct IoTTopicInfoViewModel {
    pub kind: IoTMessageKind,
    pub topic: String,
    pub device: String,
    pub location: String,
}

pub enum IoTData {
    Temp(IoTTempViewModel),
}
