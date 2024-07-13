#![allow(clippy::wildcard_imports)]
#![allow(clippy::enum_glob_use)]

use derivative::Derivative;

use error::ConnectionError;
use sqlx::PgPool;

/// Auto generated schema
pub mod models;

/// Errors
#[allow(clippy::module_name_repetitions)]
pub mod error;

/// The overarching database system
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Database {
    /// A pool of database conenctions
    pool: PgPool,
}

/// Opens a connection pool
async fn open_pool(url: &str) -> Result<PgPool, ConnectionError> {
    let pool = PgPool::connect(url).await?;
    Ok(pool)
}

impl Database {
    /// Creates a new database with the given db url
    ///
    /// Also creates the required tables if they don't already exist
    ///
    /// # Errors
    ///
    /// See [`ConnectionError`]
    pub async fn new(url: &str) -> Result<Self, ConnectionError> {
        Ok(Self {
            pool: open_pool(url).await?,
        })
    }
}
