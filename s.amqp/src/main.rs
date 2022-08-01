mod consumers;

use app::ConsumeIoTMessageServiceImpl;
use consumers::iot::IoTConsumer;
use futures_util::StreamExt;
use infra::{
    amqp::client::Amqp,
    amqp::topology::{AmqpTopology, ExchangeDefinition, QueueBindingDefinition, QueueDefinition},
    env::Config,
    logging, otel,
    repositories::iot_repository::IoTRepositoryImpl,
};
use log::error;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut cfg = Config::new();
    cfg.app_name = "amqp";
    cfg.otlp_service_type = "AMQP";

    logging::setup(&cfg)?;
    otel::tracing::setup(&cfg)?;
    let amqp = Amqp::new(&cfg).await?;

    let topology = AmqpTopology::new()
        .exchange(ExchangeDefinition::name("exchange_top_test1").direct())
        .queue(
            QueueDefinition::name("queue_top_test1")
                .with_dlq()
                .with_retry(18000, 3)
                .binding(QueueBindingDefinition::new(
                    "exchange_top_test1",
                    "queue_top_test1",
                    "exchange_top_test1_queue_top_test1",
                )),
        )
        .boxed();

    amqp.clone().install_topology(&topology).await?;

    let def = topology.get_consumers_def("queue_top_test1").unwrap();
    let mut consumer = amqp.consumer(def.queue, def.queue).await?;
    let spawn_iot = tokio::spawn({
        let cloned = amqp.clone();
        let repo = IoTRepositoryImpl::new();
        let service = ConsumeIoTMessageServiceImpl::new(repo, amqp.clone());
        let handler = IoTConsumer::new(service);

        async move {
            while let Some(delivery) = consumer.next().await {
                match delivery {
                    Ok(d) => match cloned.consume(&def, handler.clone(), &d).await {
                        Ok(_) => {}
                        _ => error!("errors consume msg"),
                    },
                    _ => error!("error receiving delivery msg"),
                };
            }
        }
    });

    let (tk1,) = tokio::join!(spawn_iot);

    tk1?;

    Ok(())
}
