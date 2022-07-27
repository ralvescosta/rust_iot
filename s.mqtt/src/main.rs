mod controllers;

use app::DeliveryIoTMessageService;
use infra::{
    env::Config,
    logging,
    mqtt::{
        types::{IoTServiceKind, MetadataKind},
        MQTT,
    },
    tracing,
};

use log::error;

use std::error::Error;

#[tokio::main(worker_threads = 1)]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::new();
    logging::setup(&cfg)?;
    tracing::setup()?;

    let delivery_service = DeliveryIoTMessageService::new();

    let mut mqtt = MQTT::new(cfg);
    let mut eventloop = mqtt.connect();

    mqtt.subscriber(
        "iot/data/temp/#",
        rumqttc::QoS::AtLeastOnce,
        MetadataKind::IoT(IoTServiceKind::Temp),
        controllers::IoTController::new(delivery_service.clone()),
    )
    .await?;

    loop {
        match eventloop.poll().await {
            Ok(event) => {
                mqtt.handle_event(&event);
            }
            Err(err) => error!("{:?}", err),
        }
    }
}
