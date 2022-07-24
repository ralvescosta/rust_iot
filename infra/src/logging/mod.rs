use std::error::Error;

use crate::env::Environment;

use super::env::Config;

use tracing::Subscriber;
use tracing_bunyan_formatter::BunyanFormattingLayer;
use tracing_log::LogTracer;
use tracing_subscriber::{
    filter::{FilterFn, LevelFilter},
    fmt::{
        format::{Format, Pretty},
        Layer,
    },
    layer::SubscriberExt,
};

pub fn setup(cfg: &Config) -> Result<(), Box<dyn Error>> {
    LogTracer::init()?;

    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());

    let level_filter = get_log_level_filter(cfg);

    let mut filter_mqtt = None;
    if cfg.enable_mqtt_logging {
        filter_mqtt = Some(FilterFn::new(|meta| {
            meta.module_path() != Some("rumqttc::state")
        }));
    }

    let mut fmt_pretty: Option<Layer<Box<dyn Subscriber>, Pretty, Format<Pretty>>> = None;
    let mut fmt_json = None;

    if cfg.env == Environment::Local {
        fmt_pretty = Some(Layer::new().pretty());
    } else {
        fmt_json = Some(BunyanFormattingLayer::new(
            cfg.app_name.to_owned(),
            non_blocking_writer,
        ));
    }

    tracing::subscriber::set_global_default(
        tracing_subscriber::registry()
            .with(level_filter)
            .with(fmt_json)
            // .with(fmt_pretty)
            .with(filter_mqtt),
    )?;

    Ok(())
}

fn get_log_level_filter(cfg: &Config) -> LevelFilter {
    match cfg.log_level {
        "debug" | "Debug" | "DEBUG" => LevelFilter::DEBUG,
        "info" | "Info" | "INFO" => LevelFilter::INFO,
        "warn" | "Warn" | "WARN" => LevelFilter::WARN,
        "error" | "Error" | "ERROR" => LevelFilter::ERROR,
        "trace" | "Trace" | "TRACE" => LevelFilter::TRACE,
        _ => LevelFilter::OFF,
    }
}
