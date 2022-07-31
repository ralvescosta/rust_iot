use app::IDeliveryIoTMessageService;
use async_trait::async_trait;
use infra::mqtt::types::{IController, Message, MessageMetadata};
use log::info;
use opentelemetry::{trace::SpanContext, Context};
use std::sync::Arc;

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

#[async_trait]
impl IController for IoTController {
    async fn exec(&self, ctx: &Context, _meta: &MessageMetadata, _msg: &Message) -> Result<(), ()> {
        info!("IoTController");

        self.service.delivery(ctx, 10).await;

        Ok(())
    }
}
