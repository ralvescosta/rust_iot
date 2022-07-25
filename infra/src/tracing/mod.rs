use opentelemetry::sdk::{
    trace::{self, IdGenerator, Sampler},
    Resource,
};
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use std::error::Error;
use std::time::Duration;
use tonic::metadata::*;

pub fn setup() -> Result<(), Box<dyn Error>> {
    let mut map = MetadataMap::with_capacity(3);

    map.insert("x-host", "example.com".parse().unwrap());
    map.insert("x-number", "123".parse().unwrap());
    map.insert_bin(
        "trace-proto-bin",
        MetadataValue::from_bytes(b"[binary data]"),
    );

    let _tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317")
                .with_timeout(Duration::from_secs(3))
                .with_metadata(map),
        )
        .with_trace_config(
            trace::config()
                .with_sampler(Sampler::AlwaysOn)
                .with_id_generator(IdGenerator::default())
                .with_max_events_per_span(64)
                .with_max_attributes_per_span(16)
                .with_max_events_per_span(16)
                .with_resource(Resource::new(vec![KeyValue::new(
                    "service.name",
                    "example",
                )])),
        )
        .install_batch(opentelemetry::runtime::Tokio)?;

    Ok(())
}
