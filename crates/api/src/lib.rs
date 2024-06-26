//! The actual api used by the core server

use derivative::Derivative;
use libreauth::pass::Hasher;
use persist::{
    error::{ConnectionError, SignUpError},
    Database, LoginAction, SignUpAction,
};
use tokio_postgres::Error as PgError;

/// Handles persist
pub mod persist;

/// The central auth authority
#[derive(Derivative)]
#[derivative(Debug)]
pub struct AuthSession {
    database: Database,
    #[derivative(Debug = "ignore")]
    #[allow(dead_code)]
    hasher: Hasher,
}

impl AuthSession {
    /// Creates a new auth session
    ///
    /// # Errors
    ///
    /// Fails when [`Database::new`] does.
    pub async fn new(url: Option<String>, hasher: Hasher) -> Result<Self, ConnectionError> {
        Ok(Self {
            database: Database::new(url).await?,
            hasher,
        })
    }

    /// Adds a user
    ///
    /// # Errors
    ///
    /// Fails when [`Database::add_user`] does.
    pub async fn sign_up(&mut self, name: &str, pass: &str) -> Result<SignUpAction, SignUpError> {
        self.database.sign_up(name, pass, &self.hasher).await
    }

    /// Tries to login with the given username & pass
    ///
    /// # Errors
    ///
    /// Fails when the database does
    pub async fn login(&mut self, name: &str, pass: &str) -> Result<LoginAction, PgError> {
        self.database.login(name, pass).await
    }
}
