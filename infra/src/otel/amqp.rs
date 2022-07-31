use opentelemetry::{
    global::{self, BoxedSpan},
    trace::{
        SpanContext, SpanId, SpanKind, TraceContextExt, TraceFlags, TraceId, TraceState, Tracer,
    },
    Context,
};
use tracing_opentelemetry::OpenTelemetrySpanExt;

const TRACE_VERSION: &str = "00";

pub struct Traceparent {
    pub trace_id: String,
    pub span_id: String,
    pub trace_flags: u8,
}

impl Traceparent {
    pub fn from_string(traceparent: String) -> Traceparent {
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

    pub fn string_from_ctx(ctx: &Context) -> String {
        let trace_id = ctx.get::<TraceId>().unwrap().to_string();
        let parent_id = ctx.get::<SpanId>().unwrap().to_string();
        let trace_flags = ctx.get::<TraceFlags>().unwrap().to_u8();
        format!(
            "{}-{}-{}-{}",
            TRACE_VERSION, trace_id, parent_id, trace_flags
        )
    }
}

pub fn get_span(
    traceparent: String,
    trace_name: &'static str,
    span_name: &'static str,
) -> (Context, BoxedSpan) {
    let parsed = Traceparent::from_string(traceparent);

    let ctx = Context::new().with_remote_span_context(SpanContext::new(
        TraceId::from_hex(&parsed.trace_id).unwrap(),
        SpanId::from_hex(&parsed.span_id).unwrap(),
        TraceFlags::new(parsed.trace_flags),
        true,
        TraceState::default(),
    ));

    tracing::Span::current().set_parent(ctx.clone());

    let tracer = global::tracer(trace_name);
    let span = tracer
        .span_builder(span_name)
        .with_kind(SpanKind::Consumer)
        .start_with_context(&tracer, &ctx);

    (ctx, span)
}
