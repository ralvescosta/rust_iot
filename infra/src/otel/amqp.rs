use opentelemetry::{
    global::{BoxedSpan, BoxedTracer},
    trace::{Span, SpanId, SpanKind, TraceId, Tracer},
};

const TRACE_VERSION: &str = "00";

pub fn traceparent(span: &BoxedSpan) -> String {
    let ctx = span.span_context();
    let trace_id = ctx.trace_id().to_string();
    let parent_id = ctx.span_id().to_string();
    let trace_flags = ctx.trace_flags().to_u8();
    format!(
        "{}-{}-{}-{}",
        TRACE_VERSION, trace_id, parent_id, trace_flags
    )
}

pub struct Traceparent {
    pub trace_id: String,
    pub span_id: String,
    pub trace_flags: u8,
}

pub fn parse_traceparent(traceparent: String) -> Traceparent {
    let splitted: Vec<&str> = traceparent.split("-").collect();

    let trace_id = splitted[1].to_string();
    let span_id = splitted[2].to_string();
    let trace_flags = splitted[3].as_bytes()[0];

    Traceparent {
        trace_id,
        span_id,
        trace_flags,
    }
}

pub fn get_span(name: &'static str, traceparent: String, tracer: &BoxedTracer) -> BoxedSpan {
    if traceparent.is_empty() {
        return tracer
            .span_builder(name)
            .with_kind(SpanKind::Consumer)
            .start(tracer);
    }

    let parsed = parse_traceparent(traceparent);

    tracer
        .span_builder(name)
        .with_kind(SpanKind::Consumer)
        .with_trace_id(TraceId::from_hex(&parsed.trace_id).unwrap())
        .with_span_id(SpanId::from_hex(&parsed.span_id).unwrap())
        .start(tracer)
}
