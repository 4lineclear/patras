//! Main entrypoint
#![allow(clippy::wildcard_imports)]

use core_server::*;

use anyhow::{Context, Result};
use axum::Router;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// TODO: add async stdin + console clearing for better dev expeerience

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();
    serve(router()).await?;
    Ok(())
}

/// Serves the given router & inits the given subscriber
///
/// # Errors
///
/// Fails when either [`TcpListener`] or [`axum::serve()`] does
pub async fn serve(router: Router) -> Result<()> {
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

fn init_logging() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
}
