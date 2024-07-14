#![allow(clippy::single_match_else)]

use derivative::Derivative;
use password_auth::generate_hash;
use sqlx::PgPool;

use crate::models::User;
use persist::{error::ConnectionError, Database};

/// Handles persist
pub mod persist;

/// The central state
#[derive(Derivative)]
#[derivative(Debug)]
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
    pub async fn new(pool: PgPool, rules: ValidationRules) -> Result<Self, ConnectionError> {
        Ok(Self {
            database: Database::new(pool).await?,
            rules,
        })
    }
    /// Try signing up a user
    ///
    /// # Errors
    ///
    /// See [`sqlx`]
    pub async fn sign_up(
        &self,
        username: &str,
        password: &str,
    ) -> Result<AddUserAction, sqlx::Error> {
        match self.rules.validate(username, password) {
            Validated::Valid => {
                if self.database.get_user(username).await?.is_some() {
                    return Ok(AddUserAction::InvalidName);
                }
            }
            Validated::InvalidName => return Ok(AddUserAction::InvalidName),
            Validated::InvalidPass => return Ok(AddUserAction::InvalidPass),
        }

        self.database
            .add_user(username, &generate_hash(password))
            .await
            .map(AddUserAction::Added)
    }
    /// Try get a user according to their username
    ///
    /// # Errors
    ///
    /// See [`sqlx`]
    pub async fn get_user(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        self.database.get_user(username).await
    }
}

/// Rules for validation
///
/// All values are treated as inclusive
#[derive(Debug, Clone, Copy)]
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
            Validated::InvalidName
        } else if pass.len() < self.pass_min || pass.len() > self.pass_max {
            Validated::InvalidPass
        } else {
            Validated::Valid
        }
    }
    /// Validates the given values
    #[must_use]
    pub const fn is_valid(&self, name: &str, pass: &str) -> bool {
        matches!(self.validate(name, pass), Validated::Valid)
    }
}

/// The result of a validation
#[derive(Debug)]
pub enum Validated {
    /// The given inputs are valid
    Valid,
    /// The given pass is invalid
    InvalidPass,
    /// The given name is invalid
    InvalidName,
}

/// The result of adding a user
pub enum AddUserAction {
    /// User added
    Added(User),
    /// An invalid password
    InvalidPass,
    /// An invalid user
    InvalidName,
}
