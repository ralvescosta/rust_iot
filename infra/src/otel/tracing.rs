use crate::env::Config;
use log::debug;
use opentelemetry::{
    global::{self, BoxedSpan},
    sdk::{
        trace::{self, IdGenerator, Sampler},
        Resource,
    },
    trace::{Span, SpanKind, TraceContextExt, Tracer},
    Context, KeyValue,
};
use opentelemetry_otlp::{Protocol, WithExportConfig};
use std::{error::Error, time::Duration};
use tonic::metadata::*;
use tracing_opentelemetry::OpenTelemetrySpanExt;

pub fn setup(cfg: &Config) -> Result<(), Box<dyn Error>> {
    debug!("telemetry :: starting telemetry setup...");

    let mut map = MetadataMap::with_capacity(3);
    map.insert("api-key", cfg.otlp_key.parse().unwrap());

    debug!("telemetry :: creating the tracer...");

    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(
            trace::config()
                .with_sampler(Sampler::AlwaysOn)
                .with_id_generator(IdGenerator::default())
                .with_max_events_per_span(64)
                .with_max_attributes_per_span(16)
                .with_resource(Resource::new(vec![
                    KeyValue::new("service.name", cfg.app_name),
                    KeyValue::new("service.type", cfg.otlp_service_type),
                ])),
        )
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(cfg.otlp_host)
                .with_protocol(Protocol::Grpc)
                .with_timeout(Duration::from_secs(cfg.otlp_export_time))
                .with_metadata(map.clone()),
        )
        .install_batch(opentelemetry::runtime::Tokio)?;
    debug!("telemetry :: tracer installed");

    // debug!("telemetry :: install metrics...");
    // let export_config = ExportConfig {
    //     endpoint: "https://otlp.nr-data.net:4317".to_string(),
    //     timeout: Duration::from_secs(3),
    //     protocol: Protocol::Grpc,
    // };
    // let meter = opentelemetry_otlp::new_pipeline()
    //     .metrics(tokio::spawn, tokio_interval_stream)
    //     .with_exporter(
    //         opentelemetry_otlp::new_exporter()
    //             .tonic()
    //             .with_export_config(export_config)
    //             .with_metadata(map),
    //     )
    //     .with_period(Duration::from_secs(3))
    //     .with_timeout(Duration::from_secs(10))
    //     .with_aggregator_selector(selectors::simple::Selector::Exact)
    //     .build()?;
    // meter.provider();
    // debug!("telemetry :: metrics installed");

    Ok(())
}

pub fn new_span(trace_name: &'static str, span_name: &'static str) -> (Context, BoxedSpan) {
    let tracer = global::tracer(trace_name);

    let span = tracer
        .span_builder(span_name)
        .with_kind(SpanKind::Consumer)
        .start(&tracer);

    let span_ctx = span.span_context();
    let trace_id = span_ctx.trace_id();
    let span_id = span_ctx.span_id();
    let flags = span_ctx.trace_flags();

    let ctx = Context::current_with_span(span);
    let ctx = ctx.with_value(trace_id);
    let ctx = ctx.with_value(span_id);

    tracing::Span::current().set_parent(ctx.clone());

    (
        ctx.with_value(flags),
        tracer.start_with_context(span_name, &ctx),
    )
}
