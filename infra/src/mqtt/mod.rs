pub mod types;

use crate::env::Config;
use types::{Handler, IoTServiceKind, Message, MessageMetadata, MetadataKind, TempMessage};

use async_trait::async_trait;
use bytes::Bytes;
use log::error;
use rumqttc::{AsyncClient, Event, EventLoop, MqttOptions, Packet, QoS};

use std::{collections::HashMap, error::Error, sync::Arc, time::Duration};

#[async_trait]
pub trait IMQTT {
    fn connect(&mut self) -> EventLoop;
    async fn subscriber(
        &mut self,
        topic: &str,
        qos: QoS,
        kind: MetadataKind,
        handler: Handler,
    ) -> Result<(), Box<dyn Error>>;
    async fn publish(
        &self,
        topic: &str,
        qos: QoS,
        retain: bool,
        payload: &[u8],
    ) -> Result<(), Box<dyn Error>>;
    fn get_metadata(&self, topic: String) -> Result<MessageMetadata, ()>;
    fn get_message(&self, kind: &MetadataKind, payload: &Bytes) -> Result<Message, ()>;
    fn handle_event(&self, event: &Event);
}

#[derive(Clone)]
pub struct MQTT {
    cfg: Box<Config>,
    client: Option<Arc<AsyncClient>>,
    dispatchers: HashMap<MetadataKind, Handler>,
}

impl MQTT {
    pub fn new(cfg: Box<Config>) -> Box<dyn IMQTT> {
        Box::new(MQTT {
            cfg,
            client: None,
            dispatchers: HashMap::default(),
        })
    }
}

#[async_trait]
impl IMQTT for MQTT {
    fn connect(&mut self) -> EventLoop {
        let mut mqtt_options =
            MqttOptions::new(self.cfg.app_name, self.cfg.mqtt_host, self.cfg.mqtt_port);

        mqtt_options
            .set_credentials(self.cfg.mqtt_user, self.cfg.mqtt_password)
            .set_keep_alive(Duration::from_secs(5));

        let (client, eventloop) = AsyncClient::new(mqtt_options, 50);

        self.client = Some(Arc::new(client));

        eventloop
    }

    async fn subscriber(
        &mut self,
        topic: &str,
        qos: QoS,
        kind: MetadataKind,
        handler: Handler,
    ) -> Result<(), Box<dyn Error>> {
        self.client.clone().unwrap().subscribe(topic, qos).await?;

        self.dispatchers.insert(kind, handler);

        Ok(())
    }

    async fn publish(
        &self,
        topic: &str,
        qos: QoS,
        retain: bool,
        payload: &[u8],
    ) -> Result<(), Box<dyn Error>> {
        self.client
            .clone()
            .unwrap()
            .publish(topic, qos, retain, payload)
            .await?;

        Ok(())
    }

    fn get_metadata(&self, topic: String) -> Result<MessageMetadata, ()> {
        let splitted = topic.split("/").collect::<Vec<&str>>();
        if splitted.len() < 3 && splitted[0] != "iot" {
            error!("unformatted topic");
            return Err(());
        }

        match splitted[2] {
            "temp" => {
                if splitted.len() < 4 {
                    return Err(());
                }

                Ok(MessageMetadata {
                    kind: MetadataKind::IoT(IoTServiceKind::Temp),
                    topic: topic.clone(),
                })
            }
            _ => {
                error!("unknown message kind");
                Err(())
            }
        }
    }

    fn get_message(&self, kind: &MetadataKind, payload: &Bytes) -> Result<Message, ()> {
        match kind {
            MetadataKind::IoT(IoTServiceKind::Temp) => {
                let msg = serde_json::from_slice::<TempMessage>(payload);
                if msg.is_err() {
                    return Err(());
                }

                Ok(Message::Temp(msg.unwrap()))
            }
            _ => {
                error!("unknown message kind");
                Err(())
            }
        }
    }

    fn handle_event(&self, event: &Event) {
        if let Event::Incoming(Packet::Publish(msg)) = event.to_owned() {
            let metadata = self.get_metadata(msg.topic);
            if metadata.is_err() {
                return;
            }
            let metadata = metadata.unwrap();

            let data = self.get_message(&metadata.kind, &msg.payload);
            if data.is_err() {
                return;
            }
            let data = data.unwrap();

            let handler = self.dispatchers.get(&metadata.kind);
            if handler.is_none() {
                return;
            }

            handler.unwrap()(&metadata, &data);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect() {
        let mut mq = MQTT::new(Config::new());
        mq.connect();
    }
}
