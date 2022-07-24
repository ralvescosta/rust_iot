mod controllers;
use app::DeliveryIoTMessageService;
use infra::{
    env::Config,
    logging,
    mqtt::{
        types::{IoTServiceKind, MetadataKind},
        MQTT,
    },
};

use std::error::Error;

#[tokio::main(worker_threads = 1)]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::new();

    logging::setup(&cfg);

    let mut mqtt = MQTT::new(cfg);
    let mut eventloop = mqtt.connect();

    let delivery_iot_msgs_service = DeliveryIoTMessageService::new();
    let iot_controller = controllers::IoTController::new(delivery_iot_msgs_service);

    mqtt.subscriber(
        "iot/data/temp/#",
        rumqttc::QoS::AtLeastOnce,
        MetadataKind::IoT(IoTServiceKind::Temp),
        iot_controller.iot_temp_controller(),
    )
    .await?;

    loop {
        match eventloop.poll().await {
            Ok(event) => {
                mqtt.handle_event(&event);
            }
            Err(err) => println!("{:?}", err),
        }
    }
}
