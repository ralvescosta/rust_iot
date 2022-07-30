use crate::errors::AmqpError;
use lapin::types::FieldTable;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, Default)]
pub struct QueueBindingDefinition {
    pub exchange: &'static str,
    pub queue: &'static str,
    pub routing_key: &'static str,
}

impl QueueBindingDefinition {
    pub fn new(exchange: &'static str, queue: &'static str, routing_key: &'static str) -> Self {
        QueueBindingDefinition {
            exchange,
            queue,
            routing_key,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct QueueDefinition {
    pub name: &'static str,
    pub bindings: Vec<QueueBindingDefinition>,
    pub with_dlq: bool,
    pub with_retry: bool,
    pub retry_ttl: Option<i32>,
}

impl QueueDefinition {
    pub fn name(name: &'static str) -> QueueDefinition {
        QueueDefinition {
            name,
            ..Default::default()
        }
    }

    pub fn with_dlq(mut self) -> Self {
        self.with_dlq = true;
        self
    }

    pub fn with_retry(mut self, milliseconds: i32) -> Self {
        self.with_retry = true;
        self.retry_ttl = Some(milliseconds);
        self
    }

    pub fn binding(mut self, bind: QueueBindingDefinition) -> Self {
        self.bindings.push(bind);
        self
    }
}

#[derive(Debug, Clone, Default)]
pub enum ExchangeKind {
    #[default]
    Direct,
    Fanout,
    Options,
    Header,
}

#[derive(Debug, Clone, Default)]
pub struct ExchangeDefinition {
    pub name: &'static str,
    pub kind: ExchangeKind,
}

impl ExchangeDefinition {
    pub fn name(name: &'static str) -> Self {
        ExchangeDefinition {
            name,
            kind: ExchangeKind::default(),
        }
    }

    pub fn direct(mut self) -> Self {
        self.kind = ExchangeKind::Direct;
        self
    }

    pub fn fanout(mut self) -> Self {
        self.kind = ExchangeKind::Fanout;
        self
    }

    pub fn header(mut self) -> Self {
        self.kind = ExchangeKind::Header;
        self
    }

    pub fn options(mut self) -> Self {
        self.kind = ExchangeKind::Options;
        self
    }
}

pub trait ConsumerHandler {
    fn exec(&self) -> Result<(), AmqpError>;
}

pub struct ConsumerDefinition {
    pub name: &'static str,
    pub queue: &'static str,
    pub with_retry: bool,
    pub retries: i64,
    pub with_dlq: bool,
    pub handler: Arc<dyn ConsumerHandler + Send + Sync>,
}

pub struct DummyConsumerHandler;

impl ConsumerHandler for DummyConsumerHandler {
    fn exec(&self) -> Result<(), AmqpError> {
        todo!()
    }
}

impl ConsumerDefinition {
    pub fn name(name: &'static str) -> ConsumerDefinition {
        ConsumerDefinition {
            name,
            queue: "",
            retries: 1,
            with_dlq: false,
            with_retry: false,
            handler: Arc::new(DummyConsumerHandler {}),
        }
    }

    pub fn queue(mut self, queue: &'static str) -> Self {
        self.queue = queue;
        self
    }

    pub fn handler(mut self, handler: Arc<dyn ConsumerHandler + Send + Sync>) -> Self {
        self.handler = handler;
        self
    }

    pub fn with_dlq(mut self) -> Self {
        self.with_dlq = true;
        self
    }

    pub fn with_retry(mut self, retries: i64) -> Self {
        self.with_retry = true;
        self.retries = retries;
        self
    }
}

pub struct AmqpTopology {
    pub exchanges: Vec<ExchangeDefinition>,
    pub queues: Vec<QueueDefinition>,
    pub consumers: Vec<ConsumerDefinition>,
}

impl AmqpTopology {
    pub fn new() -> Self {
        AmqpTopology {
            exchanges: vec![],
            queues: vec![],
            consumers: vec![],
        }
    }

    pub fn exchange(mut self, exch: ExchangeDefinition) -> Self {
        self.exchanges.push(exch);
        self
    }

    pub fn queue(mut self, queue: QueueDefinition) -> Self {
        self.queues.push(queue);
        self
    }

    pub fn consumer(mut self, consumer: ConsumerDefinition) -> Self {
        self.consumers.push(consumer);
        self
    }

    pub fn build(self) -> Self {
        self
    }
}

#[derive(Debug)]
pub struct Metadata {
    pub count: i64,
    pub traceparent: String,
}

impl Metadata {
    pub fn extract(header: &FieldTable) -> Metadata {
        let count = match header.inner().get("x-death") {
            Some(value) => match value.as_array() {
                Some(arr) => match arr.as_slice().get(0) {
                    Some(value) => match value.as_field_table() {
                        Some(table) => match table.inner().get("count") {
                            Some(value) => match value.as_long_long_int() {
                                Some(long) => long,
                                _ => 0,
                            },
                            _ => 0,
                        },
                        _ => 0,
                    },
                    _ => 0,
                },
                _ => 0,
            },
            _ => 0,
        };

        let traceparent = match header.inner().get("traceparent") {
            Some(value) => match value.as_long_string() {
                Some(st) => st.to_string(),
                _ => "".to_owned(),
            },
            _ => "".to_owned(),
        };

        Metadata { count, traceparent }
    }
}
