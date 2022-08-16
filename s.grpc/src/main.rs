mod services;

use app::ExampleServiceImpl;
use infra::{env::Config, logging, otel, repositories::iot_repository::IoTRepositoryImpl};
use log::debug;
use protos::iot::iot_data_server::IotDataServer;
use services::iot;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::sync::Arc;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cfg = Config::new();
    cfg.app_name = "ggrpc";
    cfg.app_host = "[::1]";
    cfg.app_port = 50051;

    logging::setup(&cfg)?;
    otel::tracing::setup(&cfg)?;

    // info!("{}", cfg.pg_uri());
    let opts = PgConnectOptions::new()
        .application_name(cfg.app_name)
        .host(cfg.db_host)
        .port(cfg.db_port)
        .username(cfg.db_name)
        .password(cfg.db_password);

    let pool = Arc::new(
        PgPoolOptions::new()
            .max_connections(5)
            .connect_with(opts)
            .await?,
    );

    let repository = IoTRepositoryImpl::new(pool);
    let service = ExampleServiceImpl::new(repository);
    let iot_service = iot::IoTGrpcService::new(service);

    debug!("starting server...");
    Server::builder()
        .add_service(IotDataServer::new(iot_service))
        .serve(cfg.app_addr().parse()?)
        .await?;

    Ok(())
}
