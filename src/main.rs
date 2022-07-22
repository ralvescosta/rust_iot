use tokio::{task, time};

use rumqttc::{self, AsyncClient, Event, MqttOptions, Packet, QoS};
use std::error::Error;
use std::time::Duration;

#[tokio::main(worker_threads = 1)]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    // color_backtrace::install();

    let mut mqtt_options = MqttOptions::new("test-1", "localhost", 1883);
    mqtt_options
        .set_credentials("mqtt_user", "password")
        .set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqtt_options, 50);
    task::spawn(async move {
        requests(client).await;
        time::sleep(Duration::from_secs(10)).await;
    });

    loop {
        if let Ok(e) = eventloop.poll().await {
            println!("Event::");
            if let Event::Incoming(inc) = e {
                println!("Incoming::");
                if let Packet::Publish(res) = inc {
                    println!("Publish::");
                    println!("{:?}", res)
                } else {
                    println!("{:?}\n", inc.clone());
                }
            } else {
                println!("Outgoing::");
                println!("{:?}\n", e.clone());
            }
        }
    }
}

async fn requests(client: AsyncClient) {
    client
        .subscribe("hello/world", QoS::AtMostOnce)
        .await
        .unwrap();

    for i in 1..=10 {
        client
            .publish("hello/world", QoS::AtMostOnce, false, vec![1; i])
            .await
            .unwrap();

        time::sleep(Duration::from_secs(1)).await;
    }

    time::sleep(Duration::from_secs(120)).await;
}
