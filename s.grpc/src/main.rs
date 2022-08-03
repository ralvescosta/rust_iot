mod protos;
mod services;

use protos::iot::iot_data_server::IotDataServer;
use services::iot;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let iot_service = iot::IoTGrpcService::default();

    println!("starting server...");
    Server::builder()
        .add_service(IotDataServer::new(iot_service))
        .serve(addr)
        .await?;

    Ok(())
}
