mod controllers;

use infra::{
    env::Config,
    mqtt::{
        types::{IoTServiceKind, MetadataKind},
        MQTT,
    },
};
use std::error::Error;

#[tokio::main(worker_threads = 1)]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::new();

    let mut mqtt = MQTT::new(cfg);
    let mut eventloop = mqtt.connect();

    let iot_controller = controllers::IoTController::new();

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
