use std::env::VarError;

use deadpool_postgres::{CreatePoolError, PoolError};
use thiserror::Error;

/// An error encountered while opening a connection to the database
#[derive(Error, Debug)]
pub enum ConnectionError {
    /// Var error
    #[error(transparent)]
    VarError(#[from] VarError),
    /// Env error
    #[error(transparent)]
    EnvError(#[from] dotenvy::Error),
    /// Create pool error
    #[error(transparent)]
    CreatePoolError(#[from] CreatePoolError),
    /// An error communicating with the Postgres server.
    #[error(transparent)]
    TokioPostgres(#[from] tokio_postgres::Error),
    /// Pool Error
    #[error(transparent)]
    PoolError(#[from] PoolError),
}
