use std::sync::Arc;

use infra::amqp::topology::ConsumerHandler;
pub struct IoTConsumer {}

impl ConsumerHandler for IoTConsumer {
    fn exec(&self) -> Result<(), infra::errors::AmqpError> {
        println!("Consumer");

        Ok(())
    }
}

impl IoTConsumer {
    pub fn new() -> Arc<dyn ConsumerHandler + Send + Sync> {
        Arc::new(IoTConsumer {})
    }
}
