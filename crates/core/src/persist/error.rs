use sqlx::migrate::MigrateError;
use thiserror::Error;

/// An error encountered while opening a connection to the database
#[derive(Error, Debug)]
pub enum ConnectionError {
    // /// Create pool error
    // #[error("Database pool creation error: {0}")]
    // CreatePoolError(#[from] CreatePoolError),
    /// An error communicating with the Postgres server.
    #[error("Database error: {0}")]
    Postgres(#[from] sqlx::Error),
    /// An error while migrating
    #[error("Migration error: {0}")]
    Migrate(#[from] MigrateError),
    // /// Pool Error
    // #[error("Database pool error: {0}")]
    // PoolError(#[from] PoolError),
}

/// An error encountered while trying sign up a user
#[derive(Error, Debug)]
pub enum SignUpError {
    /// An error ran into while hashing the given password
    #[error("Unable to hash given password")]
    HashError,
    /// An error communicating with the Postgres server.
    #[error("Database error: {0}")]
    Postgres(#[from] sqlx::Error),
}

/// An error encountered while trying log in a user
#[derive(Error, Debug)]
pub enum LoginError {
    /// An error ran into while hashing the given password
    #[error("Unable to hash given password")]
    HashError,
    /// An error communicating with the Postgres server.
    #[error("Database error: {0}")]
    Postgres(#[from] sqlx::Error),
}
