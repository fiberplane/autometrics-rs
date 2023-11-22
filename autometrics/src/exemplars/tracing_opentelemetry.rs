use super::TraceLabels;
use std::iter::FromIterator;
use opentelemetry_api::trace::TraceContextExt as _;
use tracing::Span;

#[cfg(all(
    not(doc),
    all(
        feature = "exemplars-tracing-opentelemetry-0_20",
        feature = "exemplars-tracing-opentelemetry-0_21"
    )
))]
compile_error!("Only one of the `exemplars-tracing-opentelemetry-0_20` and `exemplars-tracing-opentelemetry-0_21` features can be enabled at a time");

pub fn get_exemplar() -> Option<TraceLabels> {
    // Get the OpenTelemetry Context from the tracing span
    #[cfg(feature = "exemplars-tracing-opentelemetry-0_20")]
    let context = tracing_opentelemetry_0_20::OpenTelemetrySpanExt::context(&Span::current());
    #[cfg(feature = "exemplars-tracing-opentelemetry-0_21")]
    let context = tracing_opentelemetry_0_21::OpenTelemetrySpanExt::context(&Span::current());

    // Now get the OpenTelemetry "span" from the Context
    // (it's confusing because the word "span" is used by both tracing and OpenTelemetry
    // to mean slightly different things)
    let span = context.span();
    let span_context = span.span_context();

    if span_context.is_valid() {
        Some(TraceLabels::from_iter([
            ("trace_id", span_context.trace_id().to_string()),
            ("span_id", span_context.span_id().to_string()),
        ]))
    } else {
        None
    }
}
