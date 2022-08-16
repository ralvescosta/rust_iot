use crate::{errors::RepositoriesError, otel};
use async_trait::async_trait;
use deadpool_postgres::Pool;
use opentelemetry::{
    global::{self, BoxedTracer},
    trace::FutureExt,
    Context,
};
use std::{sync::Arc, time::Duration};

#[async_trait]
pub trait IoTRepository {
    async fn get(&self, ctx: &Context) -> Result<(), RepositoriesError>;
    async fn save(&self, ctx: &Context) -> Result<(), RepositoriesError>;
    async fn find(&self, ctx: &Context) -> Result<(), RepositoriesError>;
}

pub struct IoTRepositoryImpl {
    tracer: BoxedTracer,
    pool: Arc<Pool>,
}

#[async_trait]
impl IoTRepository for IoTRepositoryImpl {
    async fn get(&self, ctx: &Context) -> Result<(), RepositoriesError> {
        let cx = otel::tracing::ctx_from_ctx(&self.tracer, ctx, "sql get");

        tokio::time::sleep(Duration::from_millis(50))
            .with_context(cx)
            .await;
        Ok(())
    }

    async fn save(&self, ctx: &Context) -> Result<(), RepositoriesError> {
        let cx = otel::tracing::ctx_from_ctx(&self.tracer, ctx, "sql save");

        tokio::time::sleep(Duration::from_millis(100))
            .with_context(cx)
            .await;

        Ok(())
    }

    async fn find(&self, ctx: &Context) -> Result<(), RepositoriesError> {
        let _cx = otel::tracing::ctx_from_ctx(&self.tracer, ctx, "sql find");

        let mut _client = self.pool.get().await.unwrap();

        // let statement = client
        //     .prepare("SELECT * FROM foo WHERE bar = $1")
        //     .await
        //     .unwrap();
        // let rows = client.query(&statement, &[&""]).await.unwrap();

        Ok(())
    }
}

impl IoTRepositoryImpl {
    pub fn new(pool: Arc<Pool>) -> Arc<dyn IoTRepository + Send + Sync> {
        Arc::new(IoTRepositoryImpl {
            tracer: global::tracer("iot_repository"),
            pool,
        })
    }
}
