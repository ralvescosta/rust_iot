mod consumers;

use consumers::something::SomethingConsumer;
use futures_util::StreamExt;
use infra::{
    amqp::client::Amqp,
    amqp::topology::{AmqpTopology, ExchangeDefinition, QueueBindingDefinition, QueueDefinition},
    env::Config,
    logging, otel,
};
use log::error;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "info");

    let mut cfg = Config::new();
    cfg.app_name = "dummy";
    cfg.otlp_service_type = "AMQP";

    logging::setup(&cfg)?;
    otel::tracing::setup(&cfg)?;

    let amqp = Amqp::new(&cfg).await?;

    let topology = AmqpTopology::new()
        .exchange(ExchangeDefinition::name("exchange_top_fanout").fanout())
        .queue(
            QueueDefinition::name("queue_top_fanout2")
                .with_dlq()
                .with_retry(18000, 3)
                .binding(QueueBindingDefinition::new(
                    "exchange_top_fanout",
                    "queue_top_fanout2",
                    "",
                )),
        )
        .boxed();

    amqp.clone().install_topology(&topology).await?;

    let def_fanout2 = topology.get_consumers_def("queue_top_fanout2").unwrap();
    let mut consumer_fanout2 = amqp.consumer(def_fanout2.queue, "fanout2.queue").await?;
    let spawn_fan2 = tokio::spawn({
        let cloned = amqp.clone();
        let handler = SomethingConsumer::new("queue_top_fanout2");

        async move {
            while let Some(delivery) = consumer_fanout2.next().await {
                match delivery {
                    Ok(d) => match cloned.consume(&def_fanout2, handler.clone(), &d).await {
                        Ok(_) => {}
                        _ => error!("errors consume msg"),
                    },
                    Err(err) => error!("error receiving delivery msg - {:?}", err),
                };
            }
        }
    });

    let (tk1,) = tokio::join!(spawn_fan2);

    tk1?;

    Ok(())
}
