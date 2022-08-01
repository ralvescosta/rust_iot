use opentelemetry::{
    global::{BoxedSpan, BoxedTracer},
    trace::{
        SpanContext, SpanId, SpanKind, TraceContextExt, TraceFlags, TraceId, TraceState, Tracer,
    },
    Context,
};

const TRACE_VERSION: u8 = 0;

pub struct Traceparent {
    pub trace_id: String,
    pub span_id: String,
    pub trace_flags: u8,
}

///traceparent is compos from {trace-version}-{trace-id}-{parent-id}-{trace-flags}
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
        let trace_id = ctx.get::<TraceId>().unwrap();
        let parent_id = ctx.get::<SpanId>().unwrap();
        let trace_flags = ctx.get::<TraceFlags>().unwrap();

        format!(
            "{:02x}-{:032x}-{:016x}-{:02x}",
            TRACE_VERSION, trace_id, parent_id, trace_flags
        )
    }
}

pub fn get_span(
    tracer: &BoxedTracer,
    traceparent: String,
    span_name: &'static str,
) -> (Context, BoxedSpan) {
    let parsed = Traceparent::from_string(traceparent);

    let trace_id = TraceId::from_hex(&parsed.trace_id).unwrap();
    let span_id = SpanId::from_hex(&parsed.span_id).unwrap();
    let trace_flags = TraceFlags::new(parsed.trace_flags);

    let ctx = Context::new().with_remote_span_context(SpanContext::new(
        trace_id,
        span_id,
        trace_flags,
        true,
        TraceState::default(),
    ));

    let span = tracer
        .span_builder(span_name)
        .with_kind(SpanKind::Consumer)
        .start_with_context(tracer, &ctx);

    let ctx = ctx.with_value(trace_id);
    let ctx = ctx.with_value(span_id);

    (ctx.with_value(trace_flags), span)
}
