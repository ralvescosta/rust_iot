use crate::errors::AmqpError;
use async_trait::async_trait;

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
    pub dlq_name: &'static str,
    pub with_retry: bool,
    pub retry_ttl: Option<i32>,
    pub retries: Option<i64>,
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
        self.dlq_name = Box::leak(Box::new(self.dlq_name()));
        self
    }

    pub fn with_retry(mut self, milliseconds: i32, retries: i64) -> Self {
        self.with_retry = true;
        self.retries = Some(retries);
        self.retry_ttl = Some(milliseconds);
        self
    }

    pub fn binding(mut self, bind: QueueBindingDefinition) -> Self {
        self.bindings.push(bind);
        self
    }

    fn dlq_name(&self) -> String {
        format!("{}-dlq", self.name)
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

#[async_trait]
pub trait ConsumerHandler {
    async fn exec(&self) -> Result<(), AmqpError>;
}

#[derive(Debug, Clone, Copy)]
pub struct ConsumerDefinition {
    pub name: &'static str,
    pub queue: &'static str,
    pub with_retry: bool,
    pub retries: i64,
    pub with_dlq: bool,
    pub dlq_name: &'static str,
}

impl ConsumerDefinition {
    pub fn name(name: &'static str) -> ConsumerDefinition {
        ConsumerDefinition {
            name,
            queue: "",
            retries: 1,
            with_retry: false,
            with_dlq: false,
            dlq_name: "",
        }
    }

    pub fn queue(mut self, queue: &'static str) -> Self {
        self.queue = queue;
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

    pub fn arc(self) -> Box<Self> {
        Box::new(self)
    }

    pub fn get_consumers_def(&self, queue_name: &str) -> Option<ConsumerDefinition> {
        for queue in self.queues.clone() {
            if queue.name == queue_name {
                let retries = match queue.retries {
                    Some(r) => r,
                    _ => 0,
                };
                return Some(ConsumerDefinition {
                    name: queue.name,
                    queue: queue.name,
                    retries,
                    with_dlq: queue.with_dlq,
                    dlq_name: queue.dlq_name,
                    with_retry: queue.with_retry,
                });
            }
        }
        None
    }
}
