//! The core server implementations

use std::{sync::Arc, time::Duration};

use api::{
    libreauth::pass::{Error as HashError, HashBuilder},
    persist::error::ConnectionError,
    thiserror::{self, Error},
    AuthSession,
};

use axum::{
    extract::State,
    http::StatusCode,
    routing::{delete, post},
    Json, Router,
};
use serde::Deserialize;
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
///
/// # Errors
///
/// TODO: errors
pub async fn router(url: String) -> Result<Router, CreateRouterError> {
    let api = Api {
        auth: AuthSession::new(url, HashBuilder::new().finalize()?)
            .await?
            .into(),
    };

    Ok(Router::new()
        .layer((
            CompressionLayer::new(),
            TraceLayer::new_for_http(),
            TimeoutLayer::new(Duration::from_secs(4)),
            CatchPanicLayer::new(),
        ))
        .route("/sign-up", post(sign_up))
        .route("/log-in", post(log_in))
        .route("/log-out", delete(log_out))
        .with_state(api))
}

/// an error while creating the router
#[derive(Debug, Error)]
pub enum CreateRouterError {
    /// db connection/setup error
    #[error(transparent)]
    ConnectionError(#[from] ConnectionError),
    /// db connection/setup error
    #[error(transparent)]
    HashError(#[from] HashError),
}

type SignUpResponse = (StatusCode, ());

async fn sign_up(State(api): State<Api>, info: Json<UserInfo>) -> SignUpResponse {
    api.sign_up(&info).await
}

async fn log_in(State(_api): State<Api>) {}

async fn log_out(State(_api): State<Api>) {}

#[derive(Debug, Deserialize)]
struct UserInfo {
    name: String,
    pass: String,
}

/// The state of the router
#[derive(Debug, Clone)]
pub struct Api {
    auth: Arc<AuthSession>,
}

impl Api {
    async fn sign_up(&self, info: &UserInfo) -> SignUpResponse {
        // TODO: error handling
        match self.auth.sign_up(&info.name, &info.pass).await {
            Ok(_) => (StatusCode::OK, ()),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, ()),
        }
    }
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
