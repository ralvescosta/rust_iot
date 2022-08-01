use super::types::{Controller, Message, MessageMetadata, MetadataKind};
use crate::{env::Config, errors::MqttError, otel};
use async_trait::async_trait;
use log::{debug, error};
#[cfg(test)]
use mockall::predicate::*;
use opentelemetry::{
    global::{self, BoxedTracer},
    trace::FutureExt,
    Context,
};
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
        controller: Arc<dyn Controller + Sync + Send>,
    ) -> Result<(), MqttError>;
    async fn publish(
        &self,
        ctx: &Context,
        topic: &str,
        qos: QoS,
        retain: bool,
        payload: &[u8],
    ) -> Result<(), MqttError>;
    async fn handle_event(&self, event: &Event) -> Result<(), MqttError>;
}

pub struct MQTT {
    cfg: Box<Config>,
    client: Option<AsyncClient>,
    dispatchers: HashMap<MetadataKind, Arc<dyn Controller + Sync + Send>>,
    tracer: BoxedTracer,
}

impl MQTT {
    pub fn new(cfg: Box<Config>) -> Box<dyn IMQTT + Send + Sync> {
        Box::new(MQTT {
            cfg,
            client: None,
            dispatchers: HashMap::default(),
            tracer: global::tracer("mqtt"),
        })
    }

    #[cfg(test)]
    pub fn mock(
        cfg: Box<Config>,
        dispatchers: HashMap<MetadataKind, Arc<dyn Controller + Sync + Send>>,
    ) -> Box<dyn IMQTT + Send + Sync> {
        Box::new(MQTT {
            cfg,
            client: None,
            dispatchers,
            tracer: global::tracer("mqtt"),
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
        controller: Arc<dyn Controller + Sync + Send>,
    ) -> Result<(), MqttError> {
        debug!("subscribing in topic: {:?}...", topic);

        self.client
            .clone()
            .unwrap()
            .subscribe(topic, qos)
            .await
            .map_err(|_| MqttError::SubscribeError {})?;

        self.dispatchers.insert(kind, controller);

        debug!("subscribed");
        Ok(())
    }

    async fn publish(
        &self,
        ctx: &Context,
        topic: &str,
        qos: QoS,
        retain: bool,
        payload: &[u8],
    ) -> Result<(), MqttError> {
        debug!("publishing in a topic {:?}", topic);

        let cx = otel::tracing::ctx_from_ctx(&self.tracer, ctx, "mqtt publish");

        self.client
            .clone()
            .unwrap()
            .publish(topic, qos, retain, payload)
            .with_context(cx)
            .await
            .map_err(|_| MqttError::PublishingError {})?;

        debug!("message published");
        Ok(())
    }

    async fn handle_event(&self, event: &Event) -> Result<(), MqttError> {
        if let Event::Incoming(Packet::Publish(msg)) = event.to_owned() {
            debug!("message received in a topic {:?}", msg.topic);

            let metadata = MessageMetadata::from_topic(msg.topic)?;

            let name = format!("mqtt::event::{:?}", metadata.kind);
            let ctx = otel::tracing::new_ctx(&self.tracer, Box::leak(name.into_boxed_str()));

            let data = Message::from_payload(&metadata.kind, &msg.payload)?;

            let controller = self.dispatchers.get(&metadata.kind);
            if controller.is_none() {
                return Err(MqttError::InternalError {});
            }

            return match controller.unwrap().exec(&ctx, &metadata, &data).await {
                Ok(_) => {
                    debug!("event processed successfully");
                    // span.set_status(StatusCode::Ok, format!("event processed successfully"));
                    Ok(())
                }
                Err(e) => {
                    error!("failed to handle the event - {:?}", e);
                    // span.set_status(StatusCode::Error, format!("failed to handle the event"));
                    Err(e)
                }
            };
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rumqttc::Publish;

    use super::*;
    use crate::mqtt::types::MockController;

    #[test]
    fn should_connect() {
        let mut mq = MQTT::new(Config::mock());
        mq.connect();
    }

    #[test]
    fn should_get_metadata_successfully() {
        // let mq = MQTT::new(Config::mock());

        // let res = mq.get_metadata("iot/data/temp/device_id/location".to_owned());
        // assert!(res.is_ok());

        // let res = res.unwrap();
        // let kind = res.kind;
        // assert_eq!(kind, MetadataKind::IoT(IoTServiceKind::Temp));
    }

    #[test]
    fn should_get_metadata_err() {
        // let mq = MQTT::new(Config::mock());

        // let res = mq.get_metadata("iot/data/temp".to_owned());
        // assert!(res.is_err());

        // let res = mq.get_metadata("wrong/data/temp".to_owned());
        // assert!(res.is_err());

        // let res = mq.get_metadata("iot/data/unknown/device_id/location".to_owned());
        // assert!(res.is_err());
    }

    #[test]
    fn should_get_message_successfully() {
        // let mq = MQTT::new(Config::mock());

        // let res = mq.get_message(
        //     &MetadataKind::IoT(IoTServiceKind::Temp),
        //     &Bytes::try_from("{\"temp\": 39.9, \"time\": 99999999}").unwrap(),
        // );
        // assert!(res.is_ok());
    }

    #[test]
    fn should_get_message_err() {
        // let mq = MQTT::new(Config::mock());

        // let res = mq.get_message(
        //     &MetadataKind::IoT(IoTServiceKind::Temp),
        //     &Bytes::try_from("").unwrap(),
        // );
        // assert!(res.is_err());

        // let res = mq.get_message(
        //     &MetadataKind::IoT(IoTServiceKind::GPS),
        //     &Bytes::try_from("{\"temp\": 39.9, \"time\": 99999999}").unwrap(),
        // );
        // assert!(res.is_err());
    }

    #[test]
    fn should_handle_event_successfully() {
        // let mut mocked_controller = MockIController::new();

        // mocked_controller
        //     .expect_exec()
        //     .with(
        //         eq(MessageMetadata {
        //             kind: MetadataKind::IoT(IoTServiceKind::Temp),
        //             topic: "iot/data/temp/device_id/location".to_owned(),
        //         }),
        //         eq(Message::Temp(TempMessage {
        //             temp: 39.9,
        //             time: 99999999,
        //         })),
        //     )
        //     .times(1)
        //     .returning(|_msg, _meta| Ok(()));

        // let mut map: HashMap<MetadataKind, Arc<dyn IController + Sync + Send>> = HashMap::default();
        // map.insert(
        //     MetadataKind::IoT(IoTServiceKind::Temp),
        //     Arc::new(mocked_controller),
        // );

        // let mq = MQTT::mock(Config::mock(), map);

        // let event = Event::Incoming(Packet::Publish(Publish {
        //     dup: true,
        //     payload: Bytes::try_from("{\"temp\": 39.9, \"time\": 99999999}").unwrap(),
        //     pkid: 10,
        //     qos: QoS::AtMostOnce,
        //     retain: false,
        //     topic: "iot/data/temp/device_id/location".to_owned(),
        // }));

        // mq.handle_event(&event);
    }

    #[test]
    fn should_handle_event_err() {
        // let mut mocked_controller = MockIController::new();

        // mocked_controller
        //     .expect_exec()
        //     .with(
        //         eq(MessageMetadata {
        //             kind: MetadataKind::IoT(IoTServiceKind::Temp),
        //             topic: "iot/data/temp/device_id/location".to_owned(),
        //         }),
        //         eq(Message::Temp(TempMessage {
        //             temp: 39.9,
        //             time: 99999999,
        //         })),
        //     )
        //     .times(1)
        //     .returning(|_msg, _meta| Err(()));

        // let mut map: HashMap<MetadataKind, Arc<dyn IController + Sync + Send>> = HashMap::default();
        // map.insert(
        //     MetadataKind::IoT(IoTServiceKind::Temp),
        //     Arc::new(mocked_controller),
        // );

        // let mq = MQTT::mock(Config::mock(), map);

        // let mut publish = Publish {
        //     dup: true,
        //     payload: Bytes::new(),
        //     pkid: 10,
        //     qos: QoS::AtMostOnce,
        //     retain: false,
        //     topic: "".to_owned(),
        // };
        // mq.handle_event(&Event::Incoming(Packet::Publish(publish.clone())));

        // publish.topic = "iot/data/temp/device_id/location".to_owned();
        // mq.handle_event(&Event::Incoming(Packet::Publish(publish.clone())));

        // publish.payload = Bytes::try_from("{\"temp\": 39.9, \"time\": 99999999}").unwrap();
        // mq.handle_event(&Event::Incoming(Packet::Publish(publish.clone())));
    }
}
