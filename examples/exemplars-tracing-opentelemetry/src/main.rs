use autometrics::{autometrics, prometheus_exporter};
use autometrics_example_util::run_prometheus;
use axum::{routing::get, Router};
use opentelemetry::sdk::export::trace::stdout;
use std::{io, net::SocketAddr, time::Duration};
use tracing::{instrument, trace};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{layer::SubscriberExt, prelude::*, Registry};

// The OpenTelemetryLayer attaches the OpenTelemetry Context to every
// Span, including those created by the instrument macro.
//
// Autometrics will pick up that Context and create exemplars from it.
#[autometrics]
#[instrument]
async fn outer_function() -> String {
    trace!("Outer function called");
    inner_function("hello");

    "Hello world!".to_string()
}

// This function will also have exemplars because it is called within
// the span of the outer_function
#[autometrics]
fn inner_function(param: &str) {
    trace!("Inner function called");
}

#[tokio::main]
async fn main() {
    // Run Prometheus with flag --enable-feature=exemplars-storage
    let _prometheus = run_prometheus(true);
    tokio::spawn(generate_random_traffic());

    prometheus_exporter::init();

    let tracer = stdout::new_pipeline()
        // Throw away the spans instead of printing them to stdout
        .with_writer(io::sink())
        .install_simple();

    // Create a tracing subscriber with the OpenTelemetry layer
    Registry::default()
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();

    let app = Router::new().route("/", get(outer_function)).route(
        "/metrics",
        // Expose the metrics to Prometheus in the OpenMetrics format
        get(|| async { prometheus_exporter::encode_http_response() }),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let server = axum::Server::bind(&addr);

    println!("\nVisit the following URL to see one of the charts along with the exemplars:");
    println!("http://localhost:9090/graph?g0.expr=%23%20Rate%20of%20calls%20to%20the%20%60outer_function%60%20function%20per%20second%2C%20averaged%20over%205%20minute%20windows%0A%0Asum%20by%20(function%2C%20module%2C%20commit%2C%20version)%20(rate(%7B__name__%3D~%22function_calls(_count)%3F(_total)%3F%22%2Cfunction%3D%22outer_function%22%7D%5B5m%5D)%20*%20on%20(instance%2C%20job)%20group_left(version%2C%20commit)%20last_over_time(build_info%5B1s%5D))&g0.tab=0&g0.stacked=0&g0.show_exemplars=1&g0.range_input=1h");

    server
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("Error starting example API server");

    opentelemetry::global::shutdown_tracer_provider();
}

pub async fn generate_random_traffic() {
    let client = reqwest::Client::new();
    loop {
        client.get("http://localhost:3000").send().await.ok();
        tokio::time::sleep(Duration::from_millis(100)).await
    }
}
