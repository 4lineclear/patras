//! The core server implementations
#![allow(clippy::enum_glob_use)]

use std::{sync::Arc, time::Duration};

use axum::Router;
use axum_login::AuthManagerLayerBuilder;
use persist::{auth::Backend, error::ConnectionError};
use sqlx::PgPool;
use state::{Context, ValidationRules};
use thiserror::{self, Error};
use tokio::task::JoinHandle;
use tower_http::{
    catch_panic::CatchPanicLayer, compression::CompressionLayer, timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tower_sessions::ExpiredDeletion;
use tower_sessions::{cookie::Key, Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
    filter::FromEnvError, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

// Re-Exports for binary crates
pub use anyhow;
pub use axum;
pub use sqlx;
pub use tokio;
pub use tracing;
pub use tracing_subscriber;

/// models
pub mod models;
/// Handles persist
pub mod persist;
/// Handles state
pub mod state;

/// The main app
pub struct App {
    /// The main router
    pub router: Router,
    /// The main deletion handle
    pub deletion_handle: JoinHandle<tower_sessions::session_store::Result<()>>,
}

/// Creates the standard router
///
/// # Errors
///
/// See [`CreateRouterError`]
pub async fn router(pool: PgPool) -> Result<App, CreateRouterError> {
    let rules = ValidationRules {
        pass_min: 8,
        pass_max: 128,
        name_min: 1,
        name_max: 128,
    };
    let ctx = Context::new(pool.clone(), rules).await?.into();
    let api = Api { ctx };

    let session_store = PostgresStore::new(pool.clone());
    session_store
        .migrate()
        .await
        .map_err(ConnectionError::from)?;
    let backend = Backend::new(pool.clone());

    let deletion_handle = tokio::spawn(deletion_task(
        session_store.clone(),
        Duration::from_secs(60),
    ));

    // Generate a cryptographic key to sign the session cookie.
    let key = Key::generate();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(time::Duration::days(1)))
        .with_signed(key);

    let layers = (
        CompressionLayer::new(),
        TraceLayer::new_for_http(),
        TimeoutLayer::new(Duration::from_secs(4)),
        CatchPanicLayer::new(),
        AuthManagerLayerBuilder::new(backend, session_layer).build(),
    );

    Ok(App {
        router: Router::new().layer(layers).with_state(api),
        deletion_handle,
    })
}

async fn deletion_task(
    session_store: PostgresStore,
    period: Duration,
) -> Result<(), tower_sessions::session_store::Error> {
    let mut interval = tokio::time::interval(period);
    loop {
        session_store.delete_expired().await?;
        interval.tick().await;
    }
}

/// an error while creating the router
#[derive(Debug, Error)]
pub enum CreateRouterError {
    /// db connection/setup error
    #[error(transparent)]
    ConnectionError(#[from] ConnectionError),
}

/// The state of the router
#[derive(Debug, Clone)]
pub struct Api {
    #[allow(dead_code)]
    ctx: Arc<Context>,
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
        .add_directive("axum=trace".parse().unwrap())
        .add_directive("core_server=trace".parse().unwrap())
        .add_directive("dev_server=trace".parse().unwrap())
        .add_directive("tower_http=trace".parse().unwrap());

    Ok(tracing_subscriber::registry()
        .with(fitler_layer)
        .with(tracing_subscriber::fmt::layer()))
}
