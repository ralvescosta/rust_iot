use async_trait::async_trait;
use infra::{
    amqp::{
        client::IAmqp,
        types::{AmqpMessageType, PublishData, PublishPayload},
    },
    mqtt::types::{Message, TempMessage},
};
use log::info;
use opentelemetry::Context;
use serde::{Deserialize, Serialize};
use std::{error::Error, sync::Arc};

#[async_trait]
pub trait DeliveryIoTMessageService {
    async fn delivery(&self, ctx: &Context, msg: &Message) -> Result<(), Box<dyn Error>>;
}

#[derive(Clone)]
pub struct DeliveryIoTMessageServiceImpl {
    amqp: Arc<dyn IAmqp + Send + Sync>,
}

impl DeliveryIoTMessageServiceImpl {
    pub fn new(
        amqp: Arc<dyn IAmqp + Send + Sync>,
    ) -> Arc<dyn DeliveryIoTMessageService + Sync + Send + 'static> {
        Arc::new(DeliveryIoTMessageServiceImpl { amqp })
    }

    pub fn mock(
        amqp: Arc<dyn IAmqp + Send + Sync>,
    ) -> Option<Arc<dyn DeliveryIoTMessageService + Sync + Send + 'static>> {
        Some(Arc::new(DeliveryIoTMessageServiceImpl { amqp }))
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct AmqpTempMessage {
    pub temp: f32,
    pub time: u64,
}

impl PublishPayload for AmqpTempMessage {
    fn get_type(&self) -> AmqpMessageType {
        AmqpMessageType::Temp
    }
}

impl AmqpTempMessage {
    pub fn new(msg: &TempMessage) -> Result<PublishData, ()> {
        PublishData::new(AmqpTempMessage {
            temp: msg.temp,
            time: msg.time,
        })
        .map_err(|_| ())
    }
}

#[async_trait]
impl DeliveryIoTMessageService for DeliveryIoTMessageServiceImpl {
    async fn delivery(&self, ctx: &Context, msg: &Message) -> Result<(), Box<dyn Error>> {
        info!("MQTT::IDeliveryIoTMessageService");

        let payload = match msg {
            Message::Temp(temp) => AmqpTempMessage::new(temp),
        };

        match self
            .amqp
            .publish(
                ctx,
                "exchange_top_test1",
                "exchange_top_test1_queue_top_test1",
                &payload.unwrap(),
            )
            .await
        {
            Ok(_) => println!("mqtt success"),
            _ => println!("mqtt error"),
        };

        Ok(())
    }
}
