use futures_util::StreamExt;
use std::error::Error;
mod consumers;
use consumers::iot::IoTConsumer;
use infra::{
    amqp::client::Amqp,
    amqp::topology::{AmqpTopology, ExchangeDefinition, QueueBindingDefinition, QueueDefinition},
    env::Config,
    logging,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::new();
    logging::setup(&cfg)?;

    let topology = AmqpTopology::new()
        .exchange(ExchangeDefinition::name("exchange_top_test1").direct())
        .queue(
            QueueDefinition::name("queue_top_test1")
                .with_dlq()
                .with_retry(18000)
                .binding(QueueBindingDefinition::new(
                    "exchange_top_test1",
                    "queue_top_test1",
                    "exchange_top_test1_queue_top_test1",
                )),
        )
        .arc();

    let amqp = Amqp::new(&cfg).await?;
    amqp.clone().install_topology(&topology).await?;
    let consumers_def = topology.get_consumers_def();

    for def in consumers_def {
        let mut consumer = amqp.consumer(def.queue, def.queue).await?;

        tokio::spawn({
            let cloned = amqp.clone();
            let handler = IoTConsumer::new();

            async move {
                while let Some(delivery) = consumer.next().await {
                    cloned.consume(def, handler.clone(), delivery);
                }
            }
        });
    }
    // for task in tasks {
    //     tokio::join!(task);
    // }

    Ok(())
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let cfg = Config::new();
//     logging::setup(&cfg)?;

//     let uri = "amqp://admin:password@localhost:5672";
//     let options =
//         ConnectionProperties::default().with_connection_name(LongString::from(cfg.app_name));

//     let connection = Connection::connect(uri, options).await.unwrap();
//     let channel = connection.create_channel().await.unwrap();

//     let _queue1 = channel
//         .queue_declare(
//             "queue_test1",
//             QueueDeclareOptions::default(),
//             FieldTable::default(),
//         )
//         .await
//         .unwrap();

//     let _queue2 = channel
//         .queue_declare(
//             "queue_test2",
//             QueueDeclareOptions::default(),
//             FieldTable::default(),
//         )
//         .await
//         .unwrap();

//     let mut consumer1 = channel
//         .basic_consume(
//             "queue_test1",
//             "tag_foo1",
//             BasicConsumeOptions::default(),
//             FieldTable::default(),
//         )
//         .await
//         .unwrap();

//     let mut consumer2 = channel
//         .basic_consume(
//             "queue_test2",
//             "tag_foo2",
//             BasicConsumeOptions::default(),
//             FieldTable::default(),
//         )
//         .await
//         .unwrap();

//     channel
//         .basic_publish(
//             "",
//             "queue_test",
//             BasicPublishOptions::default(),
//             b"Hello world!",
//             BasicProperties::default(),
//         )
//         .await
//         .unwrap()
//         .await
//         .unwrap();

//     let d1 = tokio::spawn(async move {
//         while let Some(delivery) = consumer1.next().await {
//             tokio::spawn(async move {
//                 if delivery.is_err() {
//                     println!("err");
//                 }
//                 let delivery = match delivery {
//                     // Carries the delivery alongside its channel
//                     Ok(d) => d,
//                     // Carries the error and is always followed by Ok(None)
//                     Err(error) => {
//                         dbg!("Failed to consume queue message {}", error);
//                         return;
//                     }
//                 };

//                 println!("consumer received msg: {:?}", delivery.data);

//                 delivery
//                     .ack(BasicAckOptions::default())
//                     .await
//                     .expect("Failed to ack send_webhook_event message");
//             });
//         }
//     });

//     let d2 = tokio::spawn(async move {
//         while let Some(delivery) = consumer2.next().await {
//             tokio::spawn(async move {
//                 if delivery.is_err() {
//                     println!("err");
//                     return;
//                 }
//                 let delivery = match delivery {
//                     // Carries the delivery alongside its channel
//                     Ok(d) => d,
//                     // Carries the error and is always followed by Ok(None)
//                     Err(error) => {
//                         dbg!("Failed to consume queue message {}", error);
//                         return;
//                     }
//                 };

//                 println!("consumer received msg: {:?}", delivery.data);

//                 delivery
//                     .ack(BasicAckOptions::default())
//                     .await
//                     .expect("Failed to ack send_webhook_event message");
//             });
//         }
//     });

//     tokio::join!(d1, d2);

//     Ok(())
// }
