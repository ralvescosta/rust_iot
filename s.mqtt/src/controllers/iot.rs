use crate::container::ServicesContainer;
use infra::mqtt::types::{Message, MessageMetadata};
use log::info;

pub fn iot_controller(_meta: &MessageMetadata, _msg: &Message) {
    info!("iot_controller");
    let service = ServicesContainer::delivery_service();
    service.delivery(10);
}
