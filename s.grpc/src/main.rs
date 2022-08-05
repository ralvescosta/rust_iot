mod services;

use app::ExampleServiceImpl;
use infra::{env::Config, logging, otel, repositories::iot_repository::IoTRepositoryImpl};
use log::debug;
use protos::iot::iot_data_server::IotDataServer;
use services::iot;
use sqlx::postgres::PgPoolOptions;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cfg = Config::new();
    cfg.app_name = "ggrpc";
    cfg.app_host = "[::1]";
    cfg.app_port = 50051;

    logging::setup(&cfg)?;
    otel::tracing::setup(&cfg)?;

    let pool = Box::new(
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&cfg.pg_uri())
            .await?,
    );

    let repository = IoTRepositoryImpl::new(Box::leak(pool));
    let service = ExampleServiceImpl::new(repository);
    let iot_service = iot::IoTGrpcService::new(service);

    debug!("starting server...");
    Server::builder()
        .add_service(IotDataServer::new(iot_service))
        .serve(cfg.app_addr().parse()?)
        .await?;

    Ok(())
}
