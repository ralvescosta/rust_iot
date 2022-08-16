use crate::{errors::RepositoriesError, otel};
use async_trait::async_trait;
use opentelemetry::{
    global::{self, BoxedTracer},
    trace::FutureExt,
    Context,
};
use sqlx::{Pool, Postgres};
use std::{sync::Arc, time::Duration};

#[async_trait]
pub trait IoTRepository {
    async fn get(&self, ctx: &Context) -> Result<(), RepositoriesError>;
    async fn save(&self, ctx: &Context) -> Result<(), RepositoriesError>;
    async fn find(&self, ctx: &Context) -> Result<(), RepositoriesError>;
}

pub struct IoTRepositoryImpl {
    tracer: BoxedTracer,
    pool: Arc<Pool<Postgres>>,
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
        let cx = otel::tracing::ctx_from_ctx(&self.tracer, ctx, "sql find");

        let mut conn = self
            .pool
            .acquire()
            .await
            .map_err(|_| RepositoriesError::InternalError {})?;

        sqlx::query("SELECT * FROM iot")
            .execute(&mut *conn)
            .with_context(cx)
            .await
            .map_err(|_| RepositoriesError::InternalError {})?;

        Ok(())
    }
}

impl IoTRepositoryImpl {
    pub fn new(pool: Arc<Pool<Postgres>>) -> Arc<dyn IoTRepository + Send + Sync> {
        Arc::new(IoTRepositoryImpl {
            tracer: global::tracer("iot_repository"),
            pool,
        })
    }
}
