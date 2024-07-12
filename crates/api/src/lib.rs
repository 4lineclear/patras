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
    rules: ValidationRules,
}

// TODO: eventually and email sign up using https://docs.rs/lettre/latest/lettre/

impl Context {
    /// Creates a new auth session
    ///
    /// # Errors
    ///
    /// Fails when [`Database::new`] does.
    pub async fn new(
        url: String,
        hasher: Hasher,
        rules: ValidationRules,
    ) -> Result<Self, ConnectionError> {
        Ok(Self {
            database: Database::new(url, hasher).await?,
            rules,
        })
    }

    /// Adds a user
    ///
    /// # Errors
    ///
    /// Fails when [`Database::add_user`] does.
    pub async fn sign_up(&self, name: &str, pass: &str) -> Result<SignUpAction, SignUpError> {
        match self.rules.validate(name, pass) {
            Validated::Allowed => (),
            Validated::Pass => return Ok(SignUpAction::InvalidPassword),
            Validated::Name => return Ok(SignUpAction::UsernameTaken),
        }
        self.database.sign_up(name, pass).await
    }

    /// Tries to login with the given username & pass
    ///
    /// # Errors
    ///
    /// Fails when the database does
    pub async fn login(&self, name: &str, pass: &str) -> Result<LoginAction, LoginError> {
        match self.rules.validate(name, pass) {
            Validated::Allowed => (),
            Validated::Pass => return Ok(LoginAction::IncorrectPassword),
            Validated::Name => return Ok(LoginAction::UsernameNotFound),
        }
        self.database.login(name, pass).await
    }
}

/// Rules for validation
///
/// All values are treated as inclusive
#[derive(Debug)]
pub struct ValidationRules {
    /// The minimum username size
    pub name_min: usize,
    /// The maximum username size
    pub name_max: usize,

    /// The minimum password size
    pub pass_min: usize,
    /// The maximum password size
    pub pass_max: usize,
}

impl ValidationRules {
    /// Validates the given values
    #[must_use]
    pub const fn validate(&self, name: &str, pass: &str) -> Validated {
        if name.len() < self.name_min || name.len() > self.name_max {
            Validated::Name
        } else if pass.len() < self.pass_min || pass.len() > self.pass_max {
            Validated::Pass
        } else {
            Validated::Allowed
        }
    }
}

/// The result of a validation
#[derive(Debug)]
pub enum Validated {
    /// The given inputs are valid
    Allowed,
    /// The given pass is invalid
    Pass,
    /// The given name is invalid
    Name,
}
