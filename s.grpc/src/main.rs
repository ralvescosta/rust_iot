mod services;

use app::ExampleServiceImpl;
use infra::{env::Config, repositories::iot_repository::IoTRepositoryImpl};
use protos::iot::iot_data_server::IotDataServer;
use services::iot;
use sqlx::postgres::PgPoolOptions;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let cfg = Config::new();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&cfg.pg_uri())
        .await?;

    let repository = IoTRepositoryImpl::new(pool);
    let service = ExampleServiceImpl::new(repository);
    let iot_service = iot::IoTGrpcService::new(service);

    println!("starting server...");
    Server::builder()
        .add_service(IotDataServer::new(iot_service))
        .serve(addr)
        .await?;

    Ok(())
}
