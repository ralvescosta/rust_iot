use async_trait::async_trait;
use infra::amqp::{
    client::IAmqp,
    types::{PublishData, PublishPayload},
};
use log::info;
use opentelemetry::Context;
use serde::{Deserialize, Serialize};
use std::{error::Error, sync::Arc};

#[async_trait]
pub trait DeliveryIoTMessageService {
    async fn delivery(&self, ctx: &Context, data: u8) -> Result<(), Box<dyn Error>>;
}

#[derive(Clone)]
pub struct DeliveryIoTMessageServiceImpl {
    amqp: Arc<dyn IAmqp + Send + Sync>,
}

impl DeliveryIoTMessageServiceImpl {
    pub fn new(
        amqp: Arc<dyn IAmqp + Send + Sync>,
    ) -> Arc<dyn DeliveryIoTMessageService + Sync + Send + 'static> {
        Arc::new(DeliveryIoTMessageServiceImpl { amqp })
    }

    pub fn mock(
        amqp: Arc<dyn IAmqp + Send + Sync>,
    ) -> Option<Arc<dyn DeliveryIoTMessageService + Sync + Send + 'static>> {
        Some(Arc::new(DeliveryIoTMessageServiceImpl { amqp }))
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SendToAmqp {}

impl PublishPayload for SendToAmqp {
    fn get_type(&self) -> String {
        "SendToAmqp".to_owned()
    }
}

#[async_trait]
impl DeliveryIoTMessageService for DeliveryIoTMessageServiceImpl {
    async fn delivery(&self, ctx: &Context, _data: u8) -> Result<(), Box<dyn Error>> {
        info!("MQTT::IDeliveryIoTMessageService");

        let payload = SendToAmqp {};
        let data = PublishData::new(ctx, payload).unwrap();

        match self
            .amqp
            .publish(
                ctx,
                "exchange_top_test1",
                "exchange_top_test1_queue_top_test1",
                &data,
            )
            .await
        {
            Ok(_) => println!("mqtt success"),
            _ => println!("mqtt error"),
        };

        Ok(())
    }
}
