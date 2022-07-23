use infra::mqtt::types::{Message, MessageMetadata};

pub fn iot_temp_controller(meta: &MessageMetadata, msg: &Message) {
    println!("Controller")
}
