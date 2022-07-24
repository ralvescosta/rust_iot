use std::{error::Error, sync::Arc};

pub trait IDeliveryIoTMessageService {
    fn delivery(&self, data: u8) -> Result<(), Box<dyn Error>>;
}

pub struct DeliveryIoTMessageService {}

impl DeliveryIoTMessageService {
    pub fn new() -> Arc<dyn IDeliveryIoTMessageService> {
        Arc::new(DeliveryIoTMessageService {})
    }
}

impl IDeliveryIoTMessageService for DeliveryIoTMessageService {
    fn delivery(&self, data: u8) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
