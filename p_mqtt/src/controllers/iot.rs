use infra::mqtt::types::{Handler, Message, MessageMetadata};

pub struct IoTController {}

impl IoTController {
    pub fn new() -> Self {
        IoTController {}
    }

    pub fn iot_temp_controller(&self) -> Handler {
        |meta: &MessageMetadata, msg: &Message| {
            println!("Controller");
            println!("metadata: {:?}", meta);
            println!("message: {:?}", msg);
        }
    }
}
