
![GitHub_headerImage](https://user-images.githubusercontent.com/3262610/221191767-73b8a8d9-9f8b-440e-8ab6-75cb3c82f2bc.png)

[![Documentation](https://docs.rs/autometrics/badge.svg)](https://docs.rs/autometrics)
[![Crates.io](https://img.shields.io/crates/v/autometrics.svg)](https://crates.io/crates/autometrics)
[![Discord Shield](https://discordapp.com/api/guilds/950489382626951178/widget.png?style=shield)](https://discord.gg/kHtwcH8As9)

**A Rust macro that makes it easy to understand the error rate, response time, and production usage of any function in your code.**

Jump from your IDE to live Prometheus charts for each HTTP/RPC handler, database method, or other piece of application logic.

<video src="https://user-images.githubusercontent.com/3262610/220152261-2ad6ab2b-f951-4b51-8d6e-855fb71440a3.mp4" autoplay loop muted></video>

## Features
- ✨ [`#[autometrics]`](https://docs.rs/autometrics/latest/autometrics/attr.autometrics.html) macro instruments any function or `impl` block to track the most useful metrics
- 💡 Writes Prometheus queries so you can understand the data generated without knowing PromQL
- 🔗 Injects links to live Prometheus charts directly into each function's doc comments
- 📊 (Coming Soon!) Grafana dashboard showing the performance of all instrumented functions
- 🚨 Enable Prometheus alerts using SLO best practices from simple annotations in your code
- ⚙️ Configurable metric collection library (`opentelemetry`, `prometheus`, or `metrics`)
- ⚡ Minimal runtime overhead

See [Why Autometrics?](https://github.com/autometrics-dev#why-autometrics) for more details on the ideas behind autometrics.

## Examples

To see autometrics in action:

1. Install [prometheus](https://prometheus.io/download/) locally
2. Run the [complete example](./examples/full-api):
```
cargo run -p example-full-api serve
```
3. Hover over the [function names](./examples/full-api/src/routes.rs#L13) to see the generated query links
(like in the image above) and try clicking on them to go straight to that Prometheus chart.

See the other [examples](./examples/) for details on how to use the various features and integrations.

Or run the example in Gitpod:

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/autometrics-dev/autometrics-rs)

## Optional Features

### Exporting Prometheus Metrics

Autometrics includes optional functions to help collect and prepare metrics to be collected by Prometheus.

In your `Cargo.toml` file, enable the optional `prometheus-exporter` feature:

```toml
autometrics = { version = "*", features = ["prometheus-exporter"] }
```

Then, call the `global_metrics_exporter` function in your `main` function:
```rust
pub fn main() {
  let _exporter = autometrics::global_metrics_exporter();
  // ...
}
```

And create a route on your API (probably mounted under `/metrics`) that returns the following:
```rust
pub fn get_metrics() -> (StatusCode, String) {
  match autometrics::encode_global_metrics() {
    Ok(metrics) => (StatusCode::OK, metrics),
    Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", err))
  }
}
```

### Alerts / SLOs

Autometrics makes it easy to add Prometheus alerts using Service-Level Objectives (SLOs) to a function or group of functions.

This works using a pre-defined [Prometheus alerting rules](https://prometheus.io/docs/prometheus/latest/configuration/alerting_rules/).
By default, most of the recording rules are dormaint. They are enabled when specific labels are attached to the metrics produced by autometrics.

See the API docs for creating an using an autometrics [Objective](https://docs.rs/autometrics/latest/autometrics/struct.Objective.html). You can also see it the [full API example](./examples/full-api).

## Configuring

### Custom Prometheus URL
By default, Autometrics creates Prometheus query links that point to `http://localhost:9090`.

You can configure a custom Prometheus URL using a build-time environment in your `build.rs` file:

```rust
// build.rs

fn main() {
  let prometheus_url = "https://your-prometheus-url.example";
  println!("cargo:rustc-env=PROMETHEUS_URL={prometheus_url}");
}
```
Note that when using Rust Analyzer, you may need to reload the workspace in order for URL changes to take effect.


### Feature flags

- `prometheus-exporter` - exports a Prometheus metrics collector and exporter (compatible with any of the Metrics Libraries)

#### Metrics Libraries

Configure the crate that autometrics will use to produce metrics by using one of the following feature flags:

- `opentelemetry` (enabled by default) - use the [opentelemetry](https://crates.io/crates/opentelemetry) crate for producing metrics
- `metrics` - use the [metrics](https://crates.io/crates/metrics) crate for producing metrics
- `prometheus` - use the [prometheus](https://crates.io/crates/prometheus) crate for producing metrics
