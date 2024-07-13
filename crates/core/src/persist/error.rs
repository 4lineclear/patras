use sqlx::migrate::MigrateError;
use thiserror::Error;

/// An error encountered while opening a connection to the database
#[derive(Error, Debug)]
pub enum ConnectionError {
    /// An error communicating with the Postgres server.
    #[error("Database error: {0}")]
    Postgres(#[from] sqlx::Error),
    /// An error while migrating
    #[error("Migration error: {0}")]
    Migrate(#[from] MigrateError),
}
