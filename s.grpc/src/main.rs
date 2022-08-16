mod services;

use app::ExampleServiceImpl;
use infra::{
    database, env::Config, logging, otel, repositories::iot_repository::IoTRepositoryImpl,
};
use log::debug;
use protos::iot::iot_data_server::IotDataServer;
use services::iot;
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

    let pool = database::conn(&cfg).await;

    let repository = IoTRepositoryImpl::new(Arc::new(pool));
    let service = ExampleServiceImpl::new(repository);
    let iot_service = iot::IoTGrpcService::new(service);

    debug!("starting server...");
    Server::builder()
        .add_service(IotDataServer::new(iot_service))
        .serve(cfg.app_addr().parse()?)
        .await?;

    Ok(())
}
