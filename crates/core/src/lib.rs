//! The core server implementations

use std::time::Duration;

use axum::{routing::get, Router};
use tower_http::{
    catch_panic::CatchPanicLayer, compression::CompressionLayer, timeout::TimeoutLayer,
    trace::TraceLayer,
};

// Re-Exports for binary crates
pub use anyhow;
pub use axum;
pub use tokio;
pub use tracing;
pub use tracing_subscriber;

/// Handles the shutdown procedure
pub mod shutdown;

/// Creates the standard router
pub fn router() -> Router {
    Router::new()
        // temp route
        .route("/hello-world", get(|| async { "Hello, World!" }))
        .layer((
            CompressionLayer::new(),
            TraceLayer::new_for_http(),
            TimeoutLayer::new(Duration::from_secs(4)),
            CatchPanicLayer::new(),
        ))
}
