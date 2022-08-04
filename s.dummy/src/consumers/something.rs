use async_trait::async_trait;
use infra::{amqp::topology::ConsumerHandler, errors::AmqpError};
use opentelemetry::Context;
use std::sync::Arc;

pub struct SomethingConsumer {
    msg: String,
}

#[async_trait]
impl ConsumerHandler for SomethingConsumer {
    async fn exec(&self, _ctx: &Context, _data: &[u8]) -> Result<(), AmqpError> {
        println!("{}", self.msg);

        Ok(())
    }
}

impl SomethingConsumer {
    pub fn new(msg: &str) -> Arc<dyn ConsumerHandler + Send + Sync> {
        Arc::new(SomethingConsumer {
            msg: msg.to_owned(),
        })
    }
}
