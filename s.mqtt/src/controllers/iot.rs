use app::DeliveryIoTMessageService;
use async_trait::async_trait;
use infra::{
    errors::MqttError,
    mqtt::types::{Controller, Message, MessageMetadata},
};
use log::info;
use opentelemetry::Context;
use std::sync::Arc;

pub struct IoTController {
    service: Arc<dyn DeliveryIoTMessageService + Send + Sync>,
}

impl IoTController {
    pub fn new(
        service: Arc<dyn DeliveryIoTMessageService + Send + Sync>,
    ) -> Arc<dyn Controller + Send + Sync> {
        Arc::new(IoTController { service })
    }
}

#[async_trait]
impl Controller for IoTController {
    async fn exec(
        &self,
        ctx: &Context,
        _meta: &MessageMetadata,
        _msg: &Message,
    ) -> Result<(), MqttError> {
        info!("IoTController");

        self.service
            .delivery(ctx, 10)
            .await
            .map_err(|_| MqttError::InternalError {})?;

        Ok(())
    }
}
