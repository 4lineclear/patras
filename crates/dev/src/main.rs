//! Main entrypoint
#![allow(clippy::wildcard_imports)]

use core_server::*;

use anyhow::{Context, Result};
use tokio::net::TcpListener;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Handles input signal protocols
pub mod signal;

#[tokio::main]
async fn main() -> Result<()> {
    init_logging()?;
    serve().await?;
    Ok(())
}

/// Serves the given router & inits the given subscriber
///
/// # Errors
///
/// Fails when either [`TcpListener`] or [`axum::serve()`] does
pub async fn serve() -> Result<()> {
    let router = router();
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

/// Initializes logging
///
/// # Errors
///
/// Fails when the filter is not installed
pub fn init_logging() -> Result<()> {
    let fitler_layer = EnvFilter::builder()
        .with_default_directive(LevelFilter::OFF.into())
        .from_env()
        .context("Failed to parse env for logging")?
        .add_directive("core_server=trace".parse()?)
        .add_directive("dev_server=trace".parse()?)
        .add_directive("tower_http=trace".parse()?);
    tracing_subscriber::registry()
        .with(fitler_layer)
        .with(tracing_subscriber::fmt::layer())
        .init();
    Ok(())
}
