mod controllers;
mod viewmodels;

use viewmodels::{IoTData, IoTMessageKind, IoTTempViewModel, IoTTopicInfoViewModel};

use bytes::Bytes;

use rumqttc::{self, AsyncClient, Event, MqttOptions, Packet};
use std::error::Error;
use std::time::Duration;

#[tokio::main(worker_threads = 1)]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut mqtt_options = MqttOptions::new("my-app-name", "localhost", 1883);
    mqtt_options
        .set_credentials("mqtt_user", "password")
        .set_keep_alive(Duration::from_secs(5));

    let (_client, mut eventloop) = AsyncClient::new(mqtt_options, 50);

    loop {
        match eventloop.poll().await {
            Ok(event) => event_handler(&event),
            Err(err) => println!("{:?}", err),
        }
    }
}

fn topic_extractor(topic: String) -> Result<IoTTopicInfoViewModel, ()> {
    let splitted = topic.split("/").collect::<Vec<&str>>();
    if splitted.len() < 3 {
        return Err(());
    }

    match splitted[0] {
        "temp" => Ok(IoTTopicInfoViewModel {
            kind: IoTMessageKind::Temp,
            topic: splitted[0].to_owned(),
            device: splitted[1].to_owned(),
            location: splitted[2].to_owned(),
        }),

        _ => Err(()),
    }
}

fn deserializer(kind: IoTMessageKind, payload: Bytes) -> Result<IoTData, ()> {
    match kind {
        IoTMessageKind::Temp => {
            let data = serde_json::from_slice::<IoTTempViewModel>(&payload);
            if let Err(_) = data {
                return Err(());
            }

            Ok(IoTData::Temp(data.unwrap()))
        }
        _ => Err(()),
    }
}

fn event_handler(event: &Event) {
    if let Event::Incoming(Packet::Publish(publish)) = event.to_owned() {
        println!("Event::Incoming::Packet::Publish");
        println!("{:?}", publish);

        let topic = topic_extractor(publish.topic);
        if let Err(_) = topic {
            return;
        }
        let topic = topic.unwrap();

        let data = deserializer(topic.kind, publish.payload);
        if let Err(_) = data {
            return;
        }
        let data = data.unwrap();

        controller_delegate(&topic, &data);
    }
}

fn controller_delegate(info: &IoTTopicInfoViewModel, payload: &IoTData) {
    match info.topic.as_str() {
        "/temp/#" => {
            if let IoTData::Temp(data) = payload {
                controllers::iot_temp_controller(info, data);
            }
        }
        _ => println!("event with no controller"),
    }
}
