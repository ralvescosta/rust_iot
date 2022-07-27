use crate::env::Config;

pub trait IAmqp {}

pub struct Amqp {}

impl Amqp {
    pub fn new(cfg: &Config) {}
}
