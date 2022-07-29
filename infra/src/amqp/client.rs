use std::sync::Arc;

use super::topology::AmqpTopology;
use crate::{env::Config, errors::AmqpError};
use async_trait::async_trait;
use lapin::{
    options::{ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions},
    topology,
    types::{FieldTable, LongString},
    Channel, Connection, ConnectionProperties, ExchangeKind, Queue,
};

#[async_trait]
pub trait IAmqp {
    fn channel(&'static self) -> &'static Channel;
    fn connection(&'static self) -> &'static Connection;
    async fn declare_queue(
        &self,
        name: &str,
        delete: bool,
        durable: bool,
        exclusive: bool,
    ) -> Result<Queue, AmqpError>;
    async fn declare_exchange(
        &self,
        name: &str,
        delete: bool,
        durable: bool,
        internal: bool,
    ) -> Result<(), AmqpError>;
    async fn install_topology(&self, topology: AmqpTopology) -> Result<(), AmqpError>;
}

pub struct Amqp {
    conn: Connection,
    channel: Channel,
}

impl Amqp {
    pub async fn new(cfg: &Config) -> Result<Arc<dyn IAmqp + Send + Sync>, AmqpError> {
        let uri = "amqp://admin:password@localhost:5672";
        let options =
            ConnectionProperties::default().with_connection_name(LongString::from(cfg.app_name));

        let conn = Connection::connect(uri, options)
            .await
            .map_err(|_| AmqpError::ConnectionError {})?;

        let channel = conn
            .create_channel()
            .await
            .map_err(|_| AmqpError::ChannelError {})?;

        // conn.topology()

        Ok(Arc::new(Amqp { conn, channel }))
    }
}

#[async_trait]
impl IAmqp for Amqp {
    fn channel(&'static self) -> &'static Channel {
        &self.channel
    }

    fn connection(&'static self) -> &'static Connection {
        &self.conn
    }

    async fn declare_queue(
        &self,
        name: &str,
        delete: bool,
        durable: bool,
        exclusive: bool,
    ) -> Result<Queue, AmqpError> {
        self.channel
            .queue_declare(
                name,
                QueueDeclareOptions {
                    auto_delete: delete,
                    durable,
                    exclusive,
                    nowait: false,
                    passive: false,
                },
                FieldTable::default(),
            )
            .await
            .map_err(|_| AmqpError::ChannelError {})
    }

    async fn declare_exchange(
        &self,
        name: &str,
        delete: bool,
        durable: bool,
        internal: bool,
    ) -> Result<(), AmqpError> {
        self.channel
            .exchange_declare(
                name,
                ExchangeKind::Direct,
                ExchangeDeclareOptions {
                    auto_delete: delete,
                    durable,
                    internal,
                    nowait: false,
                    passive: false,
                },
                FieldTable::default(),
            )
            .await
            .map_err(|_| AmqpError::ChannelError {})
    }

    async fn install_topology(&self, topology: AmqpTopology) -> Result<(), AmqpError> {
        //declare exchanges
        //declare queues with DLQ and TTL queues to retry strategy
        //binds exchanges -> queue
        //binds exchange -> exchange
        for exch in topology.exchanges {
            self.channel
                .exchange_declare(
                    exch.name,
                    ExchangeKind::Direct,
                    ExchangeDeclareOptions {
                        auto_delete: false,
                        durable: true,
                        internal: false,
                        nowait: false,
                        passive: false,
                    },
                    FieldTable::default(),
                )
                .await
                .map_err(|_| AmqpError::DeclareExchangeError {})?;
        }

        for queue in topology.queues {
            self.channel
                .queue_declare(
                    queue.name,
                    QueueDeclareOptions {
                        passive: false,
                        durable: true,
                        exclusive: true,
                        auto_delete: false,
                        nowait: false,
                    },
                    FieldTable::default(),
                )
                .await
                .map_err(|_| AmqpError::DeclareQueueError)?;

            for bind in queue.bindings {
                self.channel
                    .queue_bind(
                        bind.queue,
                        bind.exchange,
                        bind.routing_key,
                        QueueBindOptions { nowait: false },
                        FieldTable::default(),
                    )
                    .await
                    .map_err(|_| AmqpError::DeclareQueueError {})?;
            }
        }

        Ok(())
    }
}
