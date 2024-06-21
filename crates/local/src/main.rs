//! Main entrypoint
#![allow(clippy::wildcard_imports)]

use server_core::*;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let router = server_core::router();
    server_core::serve(router).await?;
    Ok(())
}
