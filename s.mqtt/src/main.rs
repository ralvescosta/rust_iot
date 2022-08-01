mod controllers;

use app::DeliveryIoTMessageServiceImpl;
use infra::{
    amqp::client::Amqp,
    env::Config,
    logging,
    mqtt::{
        client::MQTT,
        types::{IoTServiceKind, MetadataKind},
    },
    otel,
};
use log::error;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut cfg = Config::new();
    cfg.app_name = "mqtt";
    cfg.otlp_service_type = "MQTT";

    logging::setup(&cfg)?;
    otel::tracing::setup(&cfg)?;
    let amqp = Amqp::new(&cfg).await?;

    let delivery_service = DeliveryIoTMessageServiceImpl::new(amqp.clone());

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
                match mqtt.handle_event(&event).await {
                    Ok(_) => {}
                    Err(err) => error!("{:?}", err),
                };
            }
            Err(err) => error!("{:?}", err),
        }
    }
}
