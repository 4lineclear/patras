//! The core server implementations

use std::time::Duration;

use axum::Router;
use tower_http::{
    catch_panic::CatchPanicLayer, compression::CompressionLayer, timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
    filter::FromEnvError, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

// Re-Exports for binary crates
pub use anyhow;
pub use axum;
pub use tokio;
pub use tracing;
pub use tracing_subscriber;

/// Creates the standard router
pub fn router() -> Router {
    Router::new().layer((
        CompressionLayer::new(),
        TraceLayer::new_for_http(),
        TimeoutLayer::new(Duration::from_secs(4)),
        CatchPanicLayer::new(),
    ))
}

/// Creates a logging object
///
/// # Errors
///
/// Fails when the filter is not installed
///
/// # Panics
///
/// Never
pub fn create_logging() -> Result<impl SubscriberInitExt + SubscriberExt, FromEnvError> {
    let fitler_layer = EnvFilter::builder()
        .with_default_directive(LevelFilter::OFF.into())
        .from_env()?
        .add_directive("core_server=trace".parse().unwrap())
        .add_directive("dev_server=trace".parse().unwrap())
        .add_directive("tower_http=trace".parse().unwrap());

    Ok(tracing_subscriber::registry()
        .with(fitler_layer)
        .with(tracing_subscriber::fmt::layer()))
}
