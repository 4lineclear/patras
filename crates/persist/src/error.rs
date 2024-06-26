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
    /// Pool error
    #[error(transparent)]
    DbError(#[from] DbError),
}

/// An error due to an invalid password
#[derive(Error, Debug)]
pub enum InvalidPassword {
    /// Password is shorter than the allowed min
    #[error("Password is too short, min is {0}")]
    TooShort(usize),
    /// Password is longer than the allowed max
    #[error("Password is too long, max is {0}")]
    TooLong(usize),
}

/// An error encountered while adding a user
#[derive(Error, Debug)]
pub enum AddUserError {
    /// Username taken
    #[error("Username taken")]
    UsernameTaken,
    /// Invalid Passwords
    #[error(transparent)]
    InvalidPassword(#[from] InvalidPassword),
    /// Pool error
    #[error(transparent)]
    DbError(#[from] DbError),
}

/// A pool error
#[derive(Error, Debug)]
pub enum DbError {
    /// An error communicating with the Postgres server.
    #[error(transparent)]
    TokioPostgres(#[from] tokio_postgres::Error),
    /// Pool Error
    #[error(transparent)]
    PoolError(#[from] PoolError),
}

macro_rules! def_error_coerce {
($($name:ident::<$from:ident, $to:ident> ),*) => {
    $(
    pub(crate) fn $name(e: impl Into<$from>) -> $to {
        $to::$from(e.into())
    }
    )*
};
}

def_error_coerce!(
    db_to_user::<DbError, AddUserError>,
    db_to_conn::<DbError, ConnectionError>
);
