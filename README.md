![GitHub_headerImage](https://user-images.githubusercontent.com/3262610/221191767-73b8a8d9-9f8b-440e-8ab6-75cb3c82f2bc.png)

[![Documentation](https://docs.rs/autometrics/badge.svg)](https://docs.rs/autometrics)
[![Crates.io](https://img.shields.io/crates/v/autometrics.svg)](https://crates.io/crates/autometrics)
[![Discord Shield](https://discordapp.com/api/guilds/950489382626951178/widget.png?style=shield)](https://discord.gg/kHtwcH8As9)

Autometrics provides a macro that makes it easy to instrument any function with the most useful metrics: request rate, error rate, and latency. It then uses the instrumented function names to generate Prometheus queries to help you identify and debug issues in production.

## Features

- ✨ [`#[autometrics]`](https://docs.rs/autometrics/latest/autometrics/attr.autometrics.html) macro instruments any function or `impl` block to track the [most useful metrics](https://docs.rs/autometrics/latest/autometrics/attr.autometrics.html#generated-metrics)
- 💡 Writes Prometheus queries so you can understand the data generated without knowing PromQL
- 🔗 Injects links to live Prometheus charts directly into each function's doc comments
- [🔍 Identify commits](#identifying-commits-that-introduced-problems) that introduced errors or increased latency
- [🚨 Define alerts](https://docs.rs/autometrics/latest/autometrics/objectives/index.html) using SLO best practices directly in your source code
- [📊 Grafana dashboards](https://github.com/autometrics-dev#5-configuring-prometheus) work out of the box to visualize the performance of instrumented functions & SLOs
- [⚙️ Configurable](https://docs.rs/autometrics/latest/autometrics/#metrics-libraries) metric collection library ([`opentelemetry`](https://crates.io/crates/opentelemetry), [`prometheus`](https://crates.io/crates/prometheus), [`prometheus-client`](https://crates.io/crates/prometheus-client) or [`metrics`](https://crates.io/crates/metrics))
- [📍 Attach exemplars](https://docs.rs/autometrics/latest/autometrics/exemplars/index.html) to connect metrics with traces
- ⚡ Minimal runtime overhead

See [Why Autometrics?](https://github.com/autometrics-dev#4-why-autometrics) for more details on the ideas behind autometrics.

# Example + Demo

```rust
use autometrics::autometrics;

#[autometrics]
pub async fn create_user() {
  // Now this function has metrics! 📈
}
```

<details>
<summary>See an example of a PromQL query generated by Autometrics</summary>

<br />

  _If your eyes glaze over when you see this, don't worry! Autometrics writes complex queries like this so you don't have to!_

  ```promql
    (
      sum by (function, module, commit, version) (
          rate({__name__=~"function_calls(_count)?(_total)?",function="create_user",result="error"}[5m])
        * on (instance, job) group_left (version, commit)
          last_over_time(build_info[1s])
      )
    )
  /
    (
      sum by (function, module, commit, version) (
          rate({__name__=~"function_calls(_count)?(_total)?",function="create_user"}[5m])
        * on (instance, job) group_left (version, commit)
          last_over_time(build_info[1s])
      )
    )
  ```

</details>

Here is a demo of jumping from function docs to live Prometheus charts:

https://github.com/autometrics-dev/autometrics-rs/assets/3262610/966ed140-1d6c-45f3-a607-64797d5f0233

## Quickstart

1. Add `autometrics` to your project:
    ```sh
    cargo add autometrics --features=prometheus-exporter
    ```
2. Instrument your functions with the [`#[autometrics]`](https://docs.rs/autometrics/latest/autometrics/attr.autometrics.html) macro

    <details>

    <summary> Tip: Adding autometrics to all functions using the <a href="https://docs.rs/tracing/latest/tracing/instrument/trait.Instrument.html"><code>tracing::instrument</code></a> macro
    </summary>
      <br />

      You can use a search and replace to add autometrics to all functions instrumented with `tracing::instrument`.

      Replace:
      ```rust
      #[instrument]
      ```
      With:
      ```rust
      #[instrument]
      #[autometrics]
      ```

      And then let Rust Analyzer tell you which files you need to add `use autometrics::autometrics` at the top of.

    </details>
    <details>

    <summary> Tip: Adding autometrics to all <code>pub</code> functions (not necessarily recommended 😅)
    </summary>
      <br />

      You can use a search and replace to add autometrics to all public functions. Yes, this is a bit nuts.

      Use a regular expression search to replace:
      ```
      (pub (?:async)? fn.*)
      ```

      With:
      ```
      #[autometrics]
      $1
      ```

      And then let Rust Analyzer tell you which files you need to add `use autometrics::autometrics` at the top of.

    </details>

3. Export the metrics for Prometheus

    <details>

      <summary>
      For projects not currently using Prometheus metrics
      </summary>

      <br />

      Autometrics includes optional functions to help collect and prepare metrics to be collected by Prometheus.

      In your `main` function, initialize the `prometheus_exporter`:

      ```rust
      pub fn main() {
        prometheus_exporter::init();
        // ...
      }
      ```

      And create a route on your API (probably mounted under `/metrics`) that returns the following:

      ```rust
      use autometrics::prometheus_exporter::{self, PrometheusResponse};

      /// Export metrics for Prometheus to scrape
      pub fn get_metrics() -> PrometheusResponse {
        prometheus_exporter::encode_http_response()
      }
      ```

      </details>

      <details>

      <summary>
      For projects already using custom Prometheus metrics
      </summary>

      <br />

      [Configure `autometrics`](https://docs.rs/autometrics/latest/autometrics/#metrics-libraries) to use the same underlying metrics library you use with the appropriate feature flag: `opentelemetry`, `prometheus`, `prometheus-client`, or `metrics`.

      ```toml
      [dependencies]
      autometrics = {
        version = "*",
        features = ["prometheus"],
        default-features = false
      }
      ```

      The `autometrics` metrics will be produced alongside yours.

      > **Note**
      >
      > You must ensure that you are using the exact same version of the library as `autometrics`. If not, the `autometrics` metrics will not appear in your exported metrics.
      > This is because Cargo will include both versions of the crate and the global statics used for the metrics registry will be different.

      You do not need to use the Prometheus exporter functions this library provides (you can leave out the `prometheus-exporter` feature flag) and you do not need a separate endpoint for autometrics' metrics.

      </details>

  4. [Configure Prometheus](https://github.com/autometrics-dev#5-configuring-prometheus) to scrape your metrics endpoint
  5. (Optional) If you have Grafana, import the [Autometrics dashboards](https://github.com/autometrics-dev/autometrics-shared#dashboards) for an overview and detailed view of the function metrics

## [API Docs](https://docs.rs/autometrics)

## [Examples](./examples)

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/autometrics-dev/autometrics-rs)

To see autometrics in action:

1. Install [prometheus](https://prometheus.io/download/) locally
2. Run the [complete example](./examples/full-api):

    ```shell
    cargo run -p example-full-api
    ```

3. Hover over the [function names](./examples/full-api/src/routes.rs#L13) to see the generated query links
   (like in the image above) and view the Prometheus charts

## Contributing

Issues, feature suggestions, and pull requests are very welcome!

If you are interested in getting involved:
- Join the conversation on [Discord](https://discord.gg/9eqGEs56UB)
- Ask questions and share ideas in the [Github Discussions](https://github.com/orgs/autometrics-dev/discussions)
- Take a look at the overall [Autometrics Project Roadmap](https://github.com/orgs/autometrics-dev/projects/1)
