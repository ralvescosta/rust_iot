use app::ConsumeIotMessageService;
use async_trait::async_trait;
use infra::{amqp::topology::ConsumerHandler, errors::AmqpError};
use opentelemetry::Context;
use std::sync::Arc;

pub struct IoTConsumer {
    service: Arc<dyn ConsumeIotMessageService + Send + Sync>,
}

#[async_trait]
impl ConsumerHandler for IoTConsumer {
    async fn exec(&self, ctx: &Context) -> Result<(), AmqpError> {
        println!("Consumer");

        self.service
            .consume(ctx)
            .await
            .map_err(|_| AmqpError::PublishingError)?;

        Ok(())
    }
}

impl IoTConsumer {
    pub fn new(
        service: Arc<dyn ConsumeIotMessageService + Send + Sync>,
    ) -> Arc<dyn ConsumerHandler + Send + Sync> {
        Arc::new(IoTConsumer { service })
    }
}
