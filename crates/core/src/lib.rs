//! The core server implementations

use std::time::Duration;

use anyhow::Context;
use axum::{routing::get, Router};
use tokio::net::TcpListener;
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
    // TODO: Consider using the below layers:
    // SetSensitiveHeadersLayer::new([AUTHORIZATION]),
    // TraceLayer::new_for_http().on_failure(()),
}

/// Serves the given router
///
/// # Errors
///
/// Fails when either [`TcpListener`] or [`axum::serve()`] does
pub async fn serve(router: Router) -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    let address = format!("127.0.0.1:{}", env!("SERVER_PORT"));
    let listener = TcpListener::bind(&address).await?;

    tracing::info!("Server Opened, listening on {address}");
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown::signal())
        .await
        .context("Axum Server Error")?;
    tracing::info!("Server Closed");

    Ok(())
}
