use super::topology::{AmqpTopology, QueueDefinition};
use crate::{env::Config, errors::AmqpError};
use async_trait::async_trait;
use lapin::{
    options::{ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions},
    types::{AMQPValue, FieldTable, LongString, ShortString},
    Channel, Connection, ConnectionProperties, ExchangeKind, Queue,
};
use log::debug;
use std::{collections::BTreeMap, sync::Arc};

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
    async fn binding_exchange_queue(
        &self,
        exch: &str,
        queue: &str,
        key: &str,
    ) -> Result<(), AmqpError>;
    async fn install_topology(&self, topology: AmqpTopology) -> Result<(), AmqpError>;
}

pub struct Amqp {
    conn: Connection,
    channel: Channel,
}

impl Amqp {
    pub async fn new(cfg: &Config) -> Result<Arc<dyn IAmqp + Send + Sync>, AmqpError> {
        debug!("creating amqp connection...");
        let options =
            ConnectionProperties::default().with_connection_name(LongString::from(cfg.app_name));

        let uri = &cfg.amqp_uri();
        let conn = Connection::connect(uri, options)
            .await
            .map_err(|_| AmqpError::ConnectionError {})?;
        debug!("amqp connected");

        debug!("creating amqp channel...");
        let channel = conn
            .create_channel()
            .await
            .map_err(|_| AmqpError::ChannelError {})?;
        debug!("channel created");

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
            .map_err(|_| AmqpError::DeclareQueueError(name.to_owned()))
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
            .map_err(|_| AmqpError::DeclareExchangeError(name.to_owned()))
    }

    async fn binding_exchange_queue(
        &self,
        exch: &str,
        queue: &str,
        key: &str,
    ) -> Result<(), AmqpError> {
        self.channel
            .queue_bind(
                queue,
                exch,
                key,
                QueueBindOptions { nowait: false },
                FieldTable::default(),
            )
            .await
            .map_err(|_| AmqpError::BindingExchangeToQueueError(exch.to_owned(), queue.to_owned()))
    }

    async fn install_topology(&self, topology: AmqpTopology) -> Result<(), AmqpError> {
        //declare exchanges
        //declare queues with DLQ and TTL queues to retry strategy
        //binds exchanges -> queue
        debug!("creating exchanges...");
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
                .map_err(|_| AmqpError::DeclareExchangeError(exch.name.to_owned()))?;
        }
        debug!("exchanges created");

        debug!("creating and binding queues...");
        for queue in topology.queues {
            self.install_queues(&queue).await?;
        }
        debug!("queues created and bonded");

        Ok(())
    }
}

impl Amqp {
    async fn install_queues<'i>(&self, def: &'i QueueDefinition) -> Result<(), AmqpError> {
        self.install_retry(def).await?;
        let map = self.install_dlq(def).await?;

        self.channel
            .queue_declare(
                def.name,
                QueueDeclareOptions {
                    passive: false,
                    durable: true,
                    exclusive: false,
                    auto_delete: false,
                    nowait: false,
                },
                FieldTable::from(map),
            )
            .await
            .map_err(|_| AmqpError::DeclareQueueError(def.name.to_owned()))?;

        for bind in def.clone().bindings {
            self.channel
                .queue_bind(
                    bind.queue,
                    bind.exchange,
                    bind.routing_key,
                    QueueBindOptions { nowait: false },
                    FieldTable::default(),
                )
                .await
                .map_err(|_| {
                    AmqpError::BindingExchangeToQueueError(
                        bind.exchange.to_owned(),
                        bind.queue.to_owned(),
                    )
                })?;
        }

        Ok(())
    }

    async fn install_retry<'i>(
        &self,
        def: &'i QueueDefinition,
    ) -> Result<BTreeMap<ShortString, AMQPValue>, AmqpError> {
        if !def.with_retry {
            return Ok(BTreeMap::new());
        }

        debug!("creating retry...");
        let mut map = BTreeMap::new();
        map.insert(
            ShortString::from("x-dead-letter-exchange"),
            AMQPValue::LongString(LongString::from(self.dlq_name(def.name))),
        );

        map.insert(
            ShortString::from("x-dead-letter-routing-key"),
            AMQPValue::LongString(LongString::from(self.retry_key(def.name))),
        );

        let name = self.retry_name(def.name);
        self.channel
            .queue_declare(
                &name,
                QueueDeclareOptions {
                    passive: false,
                    durable: true,
                    exclusive: false,
                    auto_delete: false,
                    nowait: false,
                },
                FieldTable::default(),
            )
            .await
            .map_err(|_| AmqpError::DeclareQueueError(name))?;

        Ok(map)
    }

    async fn install_dlq<'i>(
        &self,
        def: &'i QueueDefinition,
    ) -> Result<BTreeMap<ShortString, AMQPValue>, AmqpError> {
        if !def.with_dlq {
            return Ok(BTreeMap::new());
        }

        debug!("creating DLQ...");
        let mut map = BTreeMap::new();
        map.insert(
            ShortString::from("x-dead-letter-exchange"),
            AMQPValue::LongString(LongString::from("")),
        );

        map.insert(
            ShortString::from("x-dead-letter-routing-key"),
            AMQPValue::LongString(LongString::from(self.dlq_key(def.name))),
        );

        let name = self.dlq_name(def.name);
        self.channel
            .queue_declare(
                &name,
                QueueDeclareOptions {
                    passive: false,
                    durable: true,
                    exclusive: false,
                    auto_delete: false,
                    nowait: false,
                },
                FieldTable::default(),
            )
            .await
            .map_err(|_| AmqpError::DeclareQueueError(name))?;
        debug!("DLQ created");

        Ok(map)
    }

    fn retry_name(&self, queue: &str) -> String {
        format!("{}-retry", queue)
    }

    fn retry_key(&self, queue: &str) -> String {
        format!("{}-retry-key", queue)
    }

    fn dlq_name(&self, queue: &str) -> String {
        format!("{}-dlq", queue)
    }

    fn dlq_key(&self, queue: &str) -> String {
        format!("{}-dlq-key", queue)
    }
}
