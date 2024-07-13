#![allow(clippy::wildcard_imports)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::module_name_repetitions)]

use crate::models::User;
use derivative::Derivative;
use error::ConnectionError;
use sqlx::PgPool;
use uuid::Uuid;

/// Errors
pub mod error;

/// Auth
pub mod auth;

/// The overarching database system
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Database {
    /// A pool of database conenctions
    pool: PgPool,
}

impl Database {
    /// Creates a new database with the given db url
    ///
    /// Also creates the required tables if they don't already exist
    ///
    /// # Errors
    ///
    /// See [`ConnectionError`]
    #[allow(clippy::unused_async)]
    pub async fn new(pool: PgPool) -> Result<Self, ConnectionError> {
        Ok(Self { pool })
    }
    /// Add a user to the database, returning the user's info
    ///
    /// Note: password must be hashed
    ///
    /// # Errors
    ///
    /// See [`sqlx`]
    ///
    /// # Panics
    ///
    /// May be possible due to sqlx
    pub async fn add_user(&self, username: &str, password: &str) -> Result<User, sqlx::Error> {
        sqlx::query_file_as!(
            User,
            "queries/insert_user.sql",
            Uuid::new_v4(),
            username,
            password
        )
        .fetch_one(&self.pool)
        .await
    }
}
