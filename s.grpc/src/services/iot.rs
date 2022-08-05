use app::ExampleService;
use opentelemetry::{trace::FutureExt, Context};
use protos::iot::{
    iot_data_server::IotData, GetIoTDataRequest, GetIoTDataResponse, IoTDataMessage,
};
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct IoTGrpcService {
    service: Arc<dyn ExampleService + Sync + Send>,
}

impl IoTGrpcService {
    pub fn new(service: Arc<dyn ExampleService + Sync + Send>) -> Self {
        IoTGrpcService { service }
    }
}

#[tonic::async_trait]
impl IotData for IoTGrpcService {
    async fn get_io_t_data(
        &self,
        _request: Request<GetIoTDataRequest>,
    ) -> Result<Response<GetIoTDataResponse>, Status> {
        let ctx = Context::new();

        match self.service.get(&ctx.clone()).with_context(ctx).await {
            Ok(_) => Ok(Response::new(GetIoTDataResponse {
                data: vec![IoTDataMessage { time: 1000 }],
            })),
            _ => Err(Status::internal("internal error")),
        }
    }
}
