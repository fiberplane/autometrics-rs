//! # Autometrics
//!
//! Autometrics is a library that makes it easy to collect metrics for any function
//! -- and easily understand the data with automatically generated Prometheus queries for each function.
//!
//! ## Example
//! See the [example](https://github.com/fiberplane/autometrics-rs/blob/main/examples/axum.rs) for a full example of how to use Autometrics.
//!
//! ## Usage
//! 1. Annotate any function with `#[autometrics]` to collect metrics for that function.
//!   You can also annotate an entire `impl` block to collect metrics for all of the functions in that block.
//!
//! ```rust
//! #[autometrics]
//! async fn create_user(Json(payload): Json<CreateUser>) -> Result<Json<User>, ApiError> {
//!   // ...
//! }
//!
//! #[autometrics]
//! impl Database {
//!   async fn save_user(&self, user: User) -> Result<User, DbError> {
//!     // ...
//!   }
//! }
//!
//! ```
//!
//! 2. Call the `global_metrics_exporter` function in your `main` function:
//! ```rust
//! pub fn main() {
//!   let _exporter = autometrics::global_metrics_exporter();
//!   // ...
//! }
//! ```
//!
//! 3. Create a route on your API (probably mounted under `/metrics`) for Prometheus to scrape:
//! ```rust
//! pub fn get_metrics() -> (StatusCode, String) {
//!   match autometrics::encode_global_metrics() {
//!     Ok(metrics) => (StatusCode::OK, metrics),
//!     Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", err))
//!   }
//! }
//! ```
//!
//! 4. Hover over any autometrics-enabled function to see the links to graphs of the generated metrics
//! 5. Click on the link to see the graph of the live metrics for that function
//!
//! ## Feature flags
//!
//! - `prometheus-exporter`: Exports a Prometheus metrics collector and exporter
//!
mod labels;
#[cfg(feature = "prometheus-exporter")]
mod prometheus;
mod task_local;
mod tracker;

#[cfg(feature = "prometheus-exporter")]
#[cfg_attr(docsrs, doc(cfg(feature = "prometheus-exporter")))]
pub use self::prometheus::*;
pub use autometrics_macros::autometrics;

#[cfg(not(feature = "opentelemetry"))]
compile_error!(
    "autometrics requires one of the following feature flags to be enabled: opentelemetry"
);

// Not public API.
#[doc(hidden)]
pub mod __private {
    use crate::task_local::LocalKey;
    use std::{cell::RefCell, thread_local};

    pub use crate::labels::{GetLabels, GetLabelsFromResult};
    pub use crate::tracker::AutometricsTracker;
    pub use const_format::str_replace;

    /// Task-local value used for tracking which function called the current function
    pub static CALLER: LocalKey<&'static str> = {
        // This does the same thing as the tokio::thread_local macro with the exception that
        // it initializes the value with the empty string.
        // The tokio macro does not allow you to get the value before setting it.
        // However, in our case, we want it to simply return the empty string rather than panicking.
        thread_local! {
            static CALLER_KEY: RefCell<Option<&'static str>> = const { RefCell::new(Some("")) };
        }

        LocalKey { inner: CALLER_KEY }
    };
}
