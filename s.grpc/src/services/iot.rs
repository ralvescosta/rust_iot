use crate::protos::iot::{
    iot_data_server::IotData, GetIoTDataRequest, GetIoTDataResponse, IoTDataMessage,
};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct IoTGrpcService {}

#[tonic::async_trait]
impl IotData for IoTGrpcService {
    async fn get_io_t_data(
        &self,
        request: Request<GetIoTDataRequest>,
    ) -> Result<Response<GetIoTDataResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = GetIoTDataResponse {
            data: vec![IoTDataMessage { time: 1000 }],
        };

        Ok(Response::new(reply))
    }
}
