use crate::errors::AmqpError;

#[derive(Debug, Clone, Copy, Default)]
pub struct QueueBindingDefinition {
    pub queue: &'static str,
    pub exchange: &'static str,
    pub routing_key: &'static str,
}

impl QueueBindingDefinition {
    pub fn new(queue: &'static str, exchange: &'static str, routing_key: &'static str) -> Self {
        QueueBindingDefinition {
            queue,
            exchange,
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
    pub retry_ttl: Option<u32>,
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

    pub fn with_retry(mut self, ttl: u32) -> Self {
        self.with_retry = true;
        self.retry_ttl = Some(ttl);
        self
    }

    pub fn binding(mut self, bind: QueueBindingDefinition) -> Self {
        self.bindings.push(bind);
        self
    }

    fn dlq_name(&self) -> &'static str {
        Box::leak(format!("{}-dlq", self.name).into_boxed_str())
    }

    fn dlq_key(&self) -> &'static str {
        Box::leak(format!("{}-key-dlq", self.name).into_boxed_str())
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
    pub handler: Option<Box<dyn ConsumerHandler + Send + Sync>>,
}

impl ConsumerDefinition {
    pub fn name(name: &'static str) -> ConsumerDefinition {
        ConsumerDefinition {
            name,
            queue: "",
            handler: None,
        }
    }

    pub fn queue(mut self, queue: &'static str) -> Self {
        self.queue = queue;
        self
    }

    pub fn handler(mut self, handler: Box<dyn ConsumerHandler + Send + Sync>) -> Self {
        self.handler = Some(handler);
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
