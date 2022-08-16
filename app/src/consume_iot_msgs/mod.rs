use async_trait::async_trait;
use infra::amqp::{
    client::IAmqp,
    types::{AmqpMessageType, PublishData, PublishPayload},
};
use opentelemetry::Context;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[async_trait]
pub trait ConsumeIotMessageService {
    async fn consume(&self, ctx: &Context, msg: &[u8]) -> Result<(), ()>;
}

pub struct ConsumeIoTMessageServiceImpl {
    amqp: Arc<dyn IAmqp + Send + Sync>,
}

impl ConsumeIoTMessageServiceImpl {
    pub fn new(
        amqp: Arc<dyn IAmqp + Send + Sync>,
    ) -> Arc<dyn ConsumeIotMessageService + Send + Sync> {
        Arc::new(ConsumeIoTMessageServiceImpl { amqp })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SendToAmqp {}

impl PublishPayload for SendToAmqp {
    fn get_type(&self) -> AmqpMessageType {
        AmqpMessageType::Temp
    }
}

impl SendToAmqp {
    pub fn new() -> Result<PublishData, ()> {
        PublishData::new(SendToAmqp {}).map_err(|_| ())
    }
}

#[async_trait]
impl ConsumeIotMessageService for ConsumeIoTMessageServiceImpl {
    async fn consume(&self, ctx: &Context, _msg: &[u8]) -> Result<(), ()> {
        let data = SendToAmqp::new()?;

        self.amqp
            .publish(ctx, "exchange_top_fanout", "", &data)
            .await
            .map_err(|_| ())?;

        Ok(())
    }
}
