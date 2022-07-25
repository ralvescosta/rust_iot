mod delivery_iot_msgs;
use std::sync::Arc;

pub use delivery_iot_msgs::{DeliveryIoTMessageService, IDeliveryIoTMessageService};

pub trait Injectable {
    fn name(&self) -> &'static str;
}
