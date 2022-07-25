use log::info;
use std::{any::Any, error::Error, sync::Arc};

use crate::Injectable;

pub trait IDeliveryIoTMessageService: Injectable {
    fn delivery(&self, data: u8) -> Result<(), Box<dyn Error>>;
}

#[derive(Default, Clone)]
pub struct DeliveryIoTMessageService {}

impl DeliveryIoTMessageService {
    pub fn new() -> Arc<dyn IDeliveryIoTMessageService + Sync + Send + 'static> {
        Arc::new(DeliveryIoTMessageService {})
    }

    pub fn i() -> Arc<dyn Injectable + Send + Sync> {
        Arc::new(DeliveryIoTMessageService {})
    }

    pub fn mock() -> Option<Arc<dyn IDeliveryIoTMessageService + Sync + Send + 'static>> {
        Some(Arc::new(DeliveryIoTMessageService {}))
    }
}

impl IDeliveryIoTMessageService for DeliveryIoTMessageService {
    fn delivery(&self, _data: u8) -> Result<(), Box<dyn Error>> {
        info!("IDeliveryIoTMessageService");
        Ok(())
    }
}

impl Injectable for DeliveryIoTMessageService {
    fn name(&self) -> &'static str {
        ""
    }
}
