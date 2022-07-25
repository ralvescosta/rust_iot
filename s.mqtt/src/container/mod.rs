use lazy_static::lazy_static;
use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use app::{DeliveryIoTMessageService, IDeliveryIoTMessageService};

lazy_static! {
    pub static ref SINGLETON: Mutex<Option<ServicesContainer>> = Mutex::new(None);
}

#[derive(Clone)]
pub struct ServicesContainer {
    delivery_service: Arc<dyn IDeliveryIoTMessageService + Send + Sync>,
}

impl ServicesContainer {
    pub fn new() -> Result<(), Box<dyn Error>> {
        let mut st = SINGLETON.lock().unwrap();
        if st.is_some() {
            return Ok(());
        }

        let config = ServicesContainer {
            delivery_service: DeliveryIoTMessageService::new(),
        };

        *st = Some(config);

        Ok(())
    }

    pub fn delivery_service() -> Arc<dyn IDeliveryIoTMessageService + Send + Sync> {
        SINGLETON.lock().unwrap().clone().unwrap().delivery_service
    }
}
