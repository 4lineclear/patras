//! The core server implementations
#![allow(clippy::enum_glob_use)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::module_name_repetitions)]

use std::{sync::Arc, time::Duration};

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum_login::{login_required, AuthManagerLayerBuilder};
use sqlx::PgPool;
use thiserror::{self, Error};
use tokio::task::JoinHandle;
use tower::ServiceBuilder;
use tower_http::{
    catch_panic::CatchPanicLayer, compression::CompressionLayer, timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tower_sessions::{cookie::Key, ExpiredDeletion, Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
    filter::FromEnvError, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

use auth::{AuthSession, Backend, Credentials};
use state::{persist::error::ConnectionError, AddUserAction, Context, ValidationRules};

// Re-Exports for binary crates
pub use anyhow;
pub use axum;
pub use sqlx;
pub use tokio;
pub use tower_sessions;
pub use tracing;
pub use tracing_subscriber;

/// Handles auth
pub mod auth;
/// models
pub mod models;
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
pub async fn router(key: Key, pool: PgPool) -> Result<App, CreateRouterError> {
    let session_store = PostgresStore::new(pool.clone());
    session_store
        .migrate()
        .await
        .map_err(ConnectionError::from)?;

    let deletion_handle = tokio::spawn(deletion_task(
        session_store.clone(),
        Duration::from_secs(60),
    ));

    let session_manager_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(time::Duration::days(1)))
        .with_signed(key);
    let backend = Backend::new(pool.clone());
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_manager_layer).build();

    let layers = ServiceBuilder::new()
        .layer(CatchPanicLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(TimeoutLayer::new(Duration::from_secs(4)))
        .layer(auth_layer);

    let rules = ValidationRules {
        pass_min: 8,
        pass_max: 128,
        name_min: 1,
        name_max: 128,
    };
    let api = Context::new(pool, rules).await?;
    let api = Arc::new(api);

    let router = auth_routes()
        .merge(protected_routes())
        .layer(layers)
        .with_state(api);

    Ok(App {
        router,
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

/// Creates the actual routes
fn auth_routes() -> Router<Api> {
    Router::new()
        .route("/api/log-in", post(login))
        .route("/api/sign-up", post(sign_up))
}

/// Creates the actual routes
fn protected_routes() -> Router<Api> {
    Router::new()
        .route("/api/check-login", get(check_login))
        .route("/api/log-out", post(logout))
        .route_layer(login_required!(Backend))
}

async fn check_login() -> impl IntoResponse {
    StatusCode::OK
}

async fn logout(mut auth: AuthSession) -> impl IntoResponse {
    match auth.logout().await {
        Ok(Some(_)) => StatusCode::OK,
        Ok(None) => StatusCode::UNAUTHORIZED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn login(mut auth: AuthSession, creds: Json<Credentials>) -> StatusCode {
    let user = match auth.authenticate(creds.0).await {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::UNAUTHORIZED,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    if auth.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}

async fn sign_up(State(api): State<Api>, creds: Json<Credentials>) -> impl IntoResponse {
    use AddUserAction::*;

    match api.sign_up(&creds.username, &creds.password).await {
        Ok(Added(_)) => StatusCode::OK,
        Ok(InvalidName) => StatusCode::CONFLICT,
        Ok(InvalidPass) => StatusCode::BAD_REQUEST,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

/// an error while creating the router
#[derive(Debug, Error)]
pub enum CreateRouterError {
    /// db connection/setup error
    #[error(transparent)]
    ConnectionError(#[from] ConnectionError),
}

type Api = Arc<Context>;

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
        .add_directive("axum_login=trace".parse().unwrap())
        .add_directive("core_server=trace".parse().unwrap())
        .add_directive("dev_server=trace".parse().unwrap())
        .add_directive("tower_http=trace".parse().unwrap());

    Ok(tracing_subscriber::registry()
        .with(fitler_layer)
        .with(tracing_subscriber::fmt::layer()))
}
