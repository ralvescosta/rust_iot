use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

use app::{DeliveryIoTMessageService, IDeliveryIoTMessageService};

pub trait Injectable {}

lazy_static! {
    pub static ref SINGLETON: Mutex<Option<Container>> = Mutex::new(None);
}

pub struct Container {
    delivery_service: Arc<dyn IDeliveryIoTMessageService + Send + Sync>,
}

impl Container {
    pub fn new() {
        let mut st = SINGLETON.lock().unwrap();
        if st.is_some() {
            return;
        }

        let config = Container {
            delivery_service: DeliveryIoTMessageService::new(),
        };

        *st = Some(config);
    }

    pub fn delivery_service() -> Arc<dyn IDeliveryIoTMessageService + Send + Sync> {
        SINGLETON
            .lock()
            .unwrap()
            .take()
            .unwrap()
            .delivery_service
            .clone()
    }
}
