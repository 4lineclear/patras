//! testing if the prod server works locally

use core_server::{anyhow, axum, create_logging, tokio, tracing, tracing_subscriber};

use anyhow::Context;
use tokio::net::TcpListener;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    create_logging().context("Failed to read log env")?.init();
    let router = prod_server::router().await;
    let address = "127.0.0.1:3000";
    let listener = TcpListener::bind(&address).await?;

    tracing::info!("Server Opened, listening on {address}");
    axum::serve(listener, router)
        .await
        .context("Axum Server Error")?;
    tracing::info!("Server Closed");

    Ok(())
}
