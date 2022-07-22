use rumqttc::{self, Client, LastWill, MqttOptions, QoS};
use std::thread;
use std::time::Duration;

fn main() {
    pretty_env_logger::init();

    let mut mqtt_options = MqttOptions::new("test-1", "localhost", 1883);
    mqtt_options
        .set_credentials("mqtt_user", "password")
        .set_keep_alive(Duration::from_secs(5));

    let (client, mut connection) = Client::new(mqtt_options, 10);

    //
    thread::spawn(move || publish(client));

    //
    for (i, notification) in connection.iter().enumerate() {
        println!("{}. Notification = {:?}", i, notification);
    }

    println!("Done with the stream!!");
}

fn publish(mut client: Client) {
    client.subscribe("hello/+/world", QoS::AtMostOnce).unwrap();
    for i in 0..10_usize {
        let payload = vec![1; i];
        let topic = format!("hello/{}/world", i);
        let qos = QoS::AtLeastOnce;

        client.publish(topic, qos, false, payload).unwrap();
    }

    thread::sleep(Duration::from_secs(1));
}
