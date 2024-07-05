//! Main entrypoint
#![allow(clippy::wildcard_imports)]

use core_server::*;

use anyhow::{Context, Result};
use tokio::net::TcpListener;
use tracing_subscriber::util::SubscriberInitExt;

/// Handles input signal protocols
pub mod signal;

#[tokio::main]
async fn main() -> Result<()> {
    create_logging().context("Failed to read log env")?.init();
    serve().await?;
    Ok(())
}

/// Serves the given router & inits the given subscriber
///
/// # Errors
///
/// Fails when either [`TcpListener`] or [`axum::serve()`] does
pub async fn serve() -> Result<()> {
    let router = router().await.context("Failed t o create router")?;
    let address = format!("127.0.0.1:{}", env!("SERVER_PORT"));
    let listener = TcpListener::bind(&address).await?;

    tracing::info!("Server Opened, listening on {address}");
    axum::serve(listener, router)
        .with_graceful_shutdown(signal::signal())
        .await
        .context("Axum Server Error")?;
    tracing::info!("Server Closed");

    Ok(())
}
