use app::IDeliveryIoTMessageService;
use infra::mqtt::types::{Handler, Message, MessageMetadata};
use log::info;
use std::sync::Arc;

pub struct IoTController {
    service: Arc<dyn IDeliveryIoTMessageService>,
}

impl IoTController {
    pub fn new(service: Arc<dyn IDeliveryIoTMessageService>) -> Self {
        IoTController { service }
    }
}

impl IoTController {
    pub fn iot_temp_controller(&self) -> Handler {
        |meta: &MessageMetadata, msg: &Message| {
            info!("Controller");
            info!("metadata: {:?}", meta);
            info!("message: {:?}", msg);
        }
    }
}
