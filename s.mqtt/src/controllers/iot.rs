use std::sync::Arc;

use app::IDeliveryIoTMessageService;
use infra::mqtt::types::{IController, Message, MessageMetadata};
use log::info;

pub struct IoTController {
    service: Arc<dyn IDeliveryIoTMessageService + Send + Sync>,
}

impl IoTController {
    pub fn new(
        service: Arc<dyn IDeliveryIoTMessageService + Send + Sync>,
    ) -> Arc<dyn IController + Send + Sync> {
        Arc::new(IoTController { service })
    }
}

impl IController for IoTController {
    fn exec(&self, _meta: &MessageMetadata, _msg: &Message) -> Result<(), ()> {
        info!("IoTController");

        self.service.delivery(10);

        Ok(())
    }
}
