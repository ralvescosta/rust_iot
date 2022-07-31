use async_trait::async_trait;
use infra::repositories::iot_repository::IoTRepository;
use opentelemetry::Context;
use std::sync::Arc;

#[async_trait]
pub trait ConsumeIotMessageService {
    async fn consume(&self, ctx: &Context) -> Result<(), ()>;
}

pub struct ConsumeIoTMessageServiceImpl {
    repository: Arc<dyn IoTRepository + Send + Sync>,
}

impl ConsumeIoTMessageServiceImpl {
    pub fn new(
        repo: Arc<dyn IoTRepository + Send + Sync>,
    ) -> Arc<dyn ConsumeIotMessageService + Send + Sync> {
        Arc::new(ConsumeIoTMessageServiceImpl { repository: repo })
    }
}

#[async_trait]
impl ConsumeIotMessageService for ConsumeIoTMessageServiceImpl {
    async fn consume(&self, ctx: &Context) -> Result<(), ()> {
        self.repository.get(ctx).await.map_err(|_| ())?;

        self.repository.save(ctx).await.map_err(|_| ())?;

        Ok(())
    }
}
