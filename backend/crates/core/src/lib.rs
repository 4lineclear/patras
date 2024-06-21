//! The core server implementations

use axum::{routing::get, Router};

use anyhow::Context;
use tokio::net::TcpListener;

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
}

/// Serves the given router
///
/// # Errors
///
/// Fails when either [`TcpListener`] or [`axum::serve`] does
pub async fn serve(router: Router) -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    let address = format!("127.0.0.1:{}", env!("BACKEND_PORT"));
    let listener = TcpListener::bind(&address).await?;

    tracing::info!("Server Opened, listening on {address}");
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown::signal())
        .await
        .context("Axum Server Error")?;
    tracing::info!("Server Closed");

    Ok(())
}
