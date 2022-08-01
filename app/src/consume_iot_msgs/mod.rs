use async_trait::async_trait;
use infra::{
    amqp::{
        client::IAmqp,
        types::{PublishData, PublishPayload},
    },
    repositories::iot_repository::IoTRepository,
};
use opentelemetry::Context;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[async_trait]
pub trait ConsumeIotMessageService {
    async fn consume(&self, ctx: &Context) -> Result<(), ()>;
}

pub struct ConsumeIoTMessageServiceImpl {
    repository: Arc<dyn IoTRepository + Send + Sync>,
    amqp: Arc<dyn IAmqp + Send + Sync>,
}

impl ConsumeIoTMessageServiceImpl {
    pub fn new(
        repo: Arc<dyn IoTRepository + Send + Sync>,
        amqp: Arc<dyn IAmqp + Send + Sync>,
    ) -> Arc<dyn ConsumeIotMessageService + Send + Sync> {
        Arc::new(ConsumeIoTMessageServiceImpl {
            repository: repo,
            amqp,
        })
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
impl ConsumeIotMessageService for ConsumeIoTMessageServiceImpl {
    async fn consume(&self, ctx: &Context) -> Result<(), ()> {
        self.repository.get(ctx).await.map_err(|_| ())?;

        self.repository.save(ctx).await.map_err(|_| ())?;

        let payload = SendToAmqp {};
        let data = PublishData::new(ctx, payload).unwrap();

        self.amqp
            .publish(ctx, "exchange_top_fanout", "", &data)
            .await
            .map_err(|_| ())?;

        Ok(())
    }
}
