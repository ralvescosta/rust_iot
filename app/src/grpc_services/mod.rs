use async_trait::async_trait;
use infra::repositories::iot_repository::IoTRepository;
use opentelemetry::Context;
use std::sync::Arc;

#[async_trait]
pub trait ExampleService {
    async fn get(&self, ctx: &Context) -> Result<(), ()>;
}

pub struct ExampleServiceImpl {
    repository: Arc<dyn IoTRepository + Send + Sync>,
}

impl ExampleServiceImpl {
    pub fn new(
        repo: Arc<dyn IoTRepository + Send + Sync>,
    ) -> Arc<dyn ExampleService + Send + Sync> {
        Arc::new(ExampleServiceImpl { repository: repo })
    }
}

#[async_trait]
impl ExampleService for ExampleServiceImpl {
    async fn get(&self, ctx: &Context) -> Result<(), ()> {
        self.repository.find(ctx).await.map_err(|_| ())?;

        Ok(())
    }
}
