use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum IoTService {
    Temp,
    GPS,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum IoTMessageKind {
    IoT(IoTService),
    Health,
    Log,
}

#[derive(Serialize, Deserialize)]
pub struct IoTTempViewModel {
    pub temp: f32,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct IoTTopicInfoViewModel {
    pub kind: IoTMessageKind,
    pub topic: String,
    pub device: String,
    pub location: String,
}

#[derive(Serialize, Deserialize)]
pub enum IoTData {
    Temp(IoTTempViewModel),
    GPS,
}
