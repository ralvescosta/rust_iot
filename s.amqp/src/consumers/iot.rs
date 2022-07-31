use async_trait::async_trait;
use infra::amqp::topology::ConsumerHandler;
use std::sync::Arc;

pub struct IoTConsumer {}

#[async_trait]
impl ConsumerHandler for IoTConsumer {
    async fn exec(&self) -> Result<(), infra::errors::AmqpError> {
        println!("Consumer");

        Ok(())
    }
}

impl IoTConsumer {
    pub fn new() -> Arc<dyn ConsumerHandler + Send + Sync> {
        Arc::new(IoTConsumer {})
    }
}
