use super::env::Config;

use tracing_bunyan_formatter::BunyanFormattingLayer;
use tracing_log::LogTracer;
use tracing_subscriber::{filter::LevelFilter, fmt::Layer, layer::SubscriberExt, Registry};

pub fn setup(cfg: &Config) {
    LogTracer::init().expect("Unable to setup log tracer!");

    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());

    let bunyan_formatting_layer =
        BunyanFormattingLayer::new(cfg.app_name.to_owned(), non_blocking_writer);

    let subscriber = Registry::default()
        // .with(bunyan_formatting_layer)
        .with(Layer::new().pretty().with_writer(std::io::stdout))
        .with(LevelFilter::DEBUG);

    tracing::subscriber::set_global_default(subscriber).unwrap();
}
