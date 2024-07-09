//! The actual api used by the core server
#![allow(clippy::single_match_else)]

// use auth::Auth;
use libreauth::pass::Hasher;
use persist::{
    error::{ConnectionError, LoginError, SignUpError},
    Database, LoginAction, SignUpAction,
};

pub use derivative;
pub use libreauth;
pub use thiserror;

/// Handles persist
pub mod persist;

/// The central state
#[derive(Debug)]
pub struct Context {
    database: Database,
}

// TODO: eventually and email sign up using https://docs.rs/lettre/latest/lettre/

impl Context {
    /// Creates a new auth session
    ///
    /// # Errors
    ///
    /// Fails when [`Database::new`] does.
    pub async fn new(url: String, hasher: Hasher) -> Result<Self, ConnectionError> {
        Ok(Self {
            database: Database::new(url, hasher).await?,
        })
    }

    /// Adds a user
    ///
    /// # Errors
    ///
    /// Fails when [`Database::add_user`] does.
    pub async fn sign_up(&self, name: &str, pass: &str) -> Result<SignUpAction, SignUpError> {
        self.database.sign_up(name, pass).await
    }

    /// Tries to login with the given username & pass
    ///
    /// # Errors
    ///
    /// Fails when the database does
    pub async fn login(&self, name: &str, pass: &str) -> Result<LoginAction, LoginError> {
        self.database.login(name, pass).await
    }
}
