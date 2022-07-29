use super::types::{
    IController, IoTServiceKind, Message, MessageMetadata, MetadataKind, TempMessage,
};
use crate::env::Config;
use crate::errors::MqttError;
use async_trait::async_trait;
use bytes::Bytes;
use log::{debug, error};
#[cfg(test)]
use mockall::predicate::*;
use opentelemetry::global;
use opentelemetry::trace::{Span, SpanKind, StatusCode, Tracer};
use rumqttc::{AsyncClient, Event, EventLoop, MqttOptions, Packet, QoS};
use std::{collections::HashMap, sync::Arc, time::Duration};

#[async_trait]
pub trait IMQTT {
    fn connect(&mut self) -> EventLoop;
    async fn subscriber(
        &mut self,
        topic: &str,
        qos: QoS,
        kind: MetadataKind,
        controller: Arc<dyn IController + Sync + Send>,
    ) -> Result<(), MqttError>;
    async fn publish(
        &self,
        topic: &str,
        qos: QoS,
        retain: bool,
        payload: &[u8],
    ) -> Result<(), MqttError>;
    fn get_metadata(&self, topic: String) -> Result<MessageMetadata, MqttError>;
    fn get_message(&self, kind: &MetadataKind, payload: &Bytes) -> Result<Message, MqttError>;
    fn handle_event(&self, event: &Event);
}

#[derive(Clone)]
pub struct MQTT {
    cfg: Box<Config>,
    client: Option<AsyncClient>,
    dispatchers: HashMap<MetadataKind, Arc<dyn IController + Sync + Send>>,
}

impl MQTT {
    pub fn new(cfg: Box<Config>) -> Box<dyn IMQTT + Send + Sync> {
        Box::new(MQTT {
            cfg,
            client: None,
            dispatchers: HashMap::default(),
        })
    }

    #[cfg(test)]
    pub fn mock(
        cfg: Box<Config>,
        dispatchers: HashMap<MetadataKind, Arc<dyn IController + Sync + Send>>,
    ) -> Box<dyn IMQTT + Send + Sync> {
        Box::new(MQTT {
            cfg,
            client: None,
            dispatchers,
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

        self.client = Some(client);

        eventloop
    }

    async fn subscriber(
        &mut self,
        topic: &str,
        qos: QoS,
        kind: MetadataKind,
        controller: Arc<dyn IController + Sync + Send>,
    ) -> Result<(), MqttError> {
        debug!("subscribing in topic: {:?}...", topic);

        let res = self.client.clone().unwrap().subscribe(topic, qos).await;
        if res.is_err() {
            error!("subscribe error - {:?}", res);
            return Err(MqttError::InternalError {});
        }

        self.dispatchers.insert(kind, controller);

        debug!("subscribed");
        Ok(())
    }

    async fn publish(
        &self,
        topic: &str,
        qos: QoS,
        retain: bool,
        payload: &[u8],
    ) -> Result<(), MqttError> {
        debug!("publishing in a topic {:?}", topic);

        let res = self
            .client
            .clone()
            .unwrap()
            .publish(topic, qos, retain, payload)
            .await;
        if res.is_err() {
            error!("publish error - {:?}", res);
            return Err(MqttError::InternalError {});
        }

        debug!("message published");
        Ok(())
    }

    fn get_metadata(&self, topic: String) -> Result<MessageMetadata, MqttError> {
        let splitted = topic.split("/").collect::<Vec<&str>>();
        if splitted.len() < 3 && splitted[0] != "iot" {
            error!("unformatted topic");
            return Err(MqttError::UnformattedTopic {});
        }

        match splitted[2] {
            "temp" => {
                if splitted.len() < 4 {
                    error!("wrong temp topic");
                    return Err(MqttError::UnformattedTopic {});
                }

                Ok(MessageMetadata {
                    kind: MetadataKind::IoT(IoTServiceKind::Temp),
                    topic: topic.clone(),
                })
            }
            _ => {
                error!("unknown message kind");
                return Err(MqttError::UnknownMessageKind {});
            }
        }
    }

    fn get_message(&self, kind: &MetadataKind, payload: &Bytes) -> Result<Message, MqttError> {
        match kind {
            MetadataKind::IoT(IoTServiceKind::Temp) => {
                let msg = serde_json::from_slice::<TempMessage>(payload);
                if msg.is_err() {
                    error!("msg conversion error - {:?}", msg);
                    return Err(MqttError::InternalError {});
                }

                Ok(Message::Temp(msg.unwrap()))
            }
            _ => {
                error!("unknown message kind");
                return Err(MqttError::UnknownMessageKind {});
            }
        }
    }

    fn handle_event(&self, event: &Event) {
        if let Event::Incoming(Packet::Publish(msg)) = event.to_owned() {
            debug!("message received in a topic {:?}", msg.topic);

            let metadata = self.get_metadata(msg.topic);
            if metadata.is_err() {
                return;
            }
            let metadata = metadata.unwrap();

            let tracer = global::tracer("handle_event");
            let name = format!("mqtt::event::{}", metadata.kind);
            let name: &str = Box::leak(name.into_boxed_str());

            let mut span = tracer
                .span_builder(name)
                .with_kind(SpanKind::Consumer)
                .start(&tracer);

            let data = self.get_message(&metadata.kind, &msg.payload);
            if data.is_err() {
                span.set_status(StatusCode::Error, format!("ignored message"));
                return;
            }
            let data = data.unwrap();

            let controller = self.dispatchers.get(&metadata.kind);
            if controller.is_none() {
                span.set_status(StatusCode::Error, format!("ignored message"));
                return;
            }

            match controller.unwrap().exec(&metadata, &data) {
                Ok(_) => {
                    debug!("event processed successfully");
                    span.set_status(StatusCode::Ok, format!("event processed successfully"));
                }
                Err(e) => {
                    error!("failed to handle the event - {:?}", e);
                    span.set_status(StatusCode::Error, format!("failed to handle the event"));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rumqttc::Publish;

    use super::*;
    use crate::mqtt::types::MockIController;

    #[test]
    fn should_connect() {
        let mut mq = MQTT::new(Config::mock());
        mq.connect();
    }

    #[test]
    fn should_get_metadata_successfully() {
        let mq = MQTT::new(Config::mock());

        let res = mq.get_metadata("iot/data/temp/device_id/location".to_owned());
        assert!(res.is_ok());

        let res = res.unwrap();
        let kind = res.kind;
        assert_eq!(kind, MetadataKind::IoT(IoTServiceKind::Temp));
    }

    #[test]
    fn should_get_metadata_err() {
        let mq = MQTT::new(Config::mock());

        let res = mq.get_metadata("iot/data/temp".to_owned());
        assert!(res.is_err());

        let res = mq.get_metadata("wrong/data/temp".to_owned());
        assert!(res.is_err());

        let res = mq.get_metadata("iot/data/unknown/device_id/location".to_owned());
        assert!(res.is_err());
    }

    #[test]
    fn should_get_message_successfully() {
        let mq = MQTT::new(Config::mock());

        let res = mq.get_message(
            &MetadataKind::IoT(IoTServiceKind::Temp),
            &Bytes::try_from("{\"temp\": 39.9, \"time\": 99999999}").unwrap(),
        );
        assert!(res.is_ok());
    }

    #[test]
    fn should_get_message_err() {
        let mq = MQTT::new(Config::mock());

        let res = mq.get_message(
            &MetadataKind::IoT(IoTServiceKind::Temp),
            &Bytes::try_from("").unwrap(),
        );
        assert!(res.is_err());

        let res = mq.get_message(
            &MetadataKind::IoT(IoTServiceKind::GPS),
            &Bytes::try_from("{\"temp\": 39.9, \"time\": 99999999}").unwrap(),
        );
        assert!(res.is_err());
    }

    #[test]
    fn should_handle_event_successfully() {
        let mut mocked_controller = MockIController::new();

        mocked_controller
            .expect_exec()
            .with(
                eq(MessageMetadata {
                    kind: MetadataKind::IoT(IoTServiceKind::Temp),
                    topic: "iot/data/temp/device_id/location".to_owned(),
                }),
                eq(Message::Temp(TempMessage {
                    temp: 39.9,
                    time: 99999999,
                })),
            )
            .times(1)
            .returning(|_msg, _meta| Ok(()));

        let mut map: HashMap<MetadataKind, Arc<dyn IController + Sync + Send>> = HashMap::default();
        map.insert(
            MetadataKind::IoT(IoTServiceKind::Temp),
            Arc::new(mocked_controller),
        );

        let mq = MQTT::mock(Config::mock(), map);

        let event = Event::Incoming(Packet::Publish(Publish {
            dup: true,
            payload: Bytes::try_from("{\"temp\": 39.9, \"time\": 99999999}").unwrap(),
            pkid: 10,
            qos: QoS::AtMostOnce,
            retain: false,
            topic: "iot/data/temp/device_id/location".to_owned(),
        }));

        mq.handle_event(&event);
    }

    #[test]
    fn should_handle_event_err() {
        let map = HashMap::default();

        let mq = MQTT::mock(Config::mock(), map);

        let mut publish = Publish {
            dup: true,
            payload: Bytes::try_from("").unwrap(),
            pkid: 10,
            qos: QoS::AtMostOnce,
            retain: false,
            topic: "".to_owned(),
        };
        mq.handle_event(&Event::Incoming(Packet::Publish(publish.clone())));

        publish.topic = "iot/data/temp/device_id/location".to_owned();
        mq.handle_event(&Event::Incoming(Packet::Publish(publish.clone())));

        publish.payload = Bytes::try_from("{\"temp\": 39.9, \"time\": 99999999}").unwrap();
        mq.handle_event(&Event::Incoming(Packet::Publish(publish.clone())));
    }
}
