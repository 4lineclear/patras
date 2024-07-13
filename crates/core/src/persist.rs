#![allow(clippy::wildcard_imports)]
#![allow(clippy::enum_glob_use)]

use derivative::Derivative;
use libreauth::pass::Hasher;
// use sql::CREATE_USER_TABLE;
// use deadpool_postgres::{Config, GenericClient, Pool, Runtime};
// use tokio_postgres::{Error as PgError, NoTls};

use error::ConnectionError;
use sql::MIGRATIONS;
use sqlx::PgPool;
use uuid::Uuid;

// /// Handles auth
// pub mod auth;

/// Auto generated schema
pub mod models;

/// Holds some queries
pub(crate) mod sql;

// /// postgres store
// ///
// /// Based off:
// ///
// /// - [https://github.com/maxcountryman/tower-sessions/discussions/120]
// /// - [https://github.com/maxcountryman/tower-sessions-stores/blob/main/sqlx-store/src/postgres_store.rs]
// pub mod store;

/// Errors
#[allow(clippy::module_name_repetitions)]
pub mod error;

/// The overarching database system
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Database {
    /// A pool of database conenctions
    pool: PgPool,
    // auth: Auth,
}

/// Opens a connection pool
async fn open_pool(url: &str) -> Result<PgPool, ConnectionError> {
    let pool = PgPool::connect(url).await?;
    MIGRATIONS.run(&pool).await?;
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
    pub async fn new(url: &str, hasher: Hasher) -> Result<Self, ConnectionError> {
        let pool = open_pool(url).await?;
        // let conn = pool.acquire().await?;
        // let auth = Auth::new(hasher, conn).await?;

        Ok(Self { pool })
    }

    // /// Tries to add a user
    // ///
    // /// # Errors
    // ///
    // /// Fails when the database does
    // pub async fn sign_up(&self, name: &str, pass: &str) -> Result<SignUpAction, SignUpError> {
    //     let Some(pass) = self.auth.hash(pass) else {
    //         return Ok(SignUpAction::InvalidPassword);
    //     };
    //
    //     if self.username_taken(name).await? {
    //         return Ok(SignUpAction::UsernameTaken);
    //     }
    //
    //     let uuid = Uuid::new_v4();
    //     let row: User = self
    //         .auth
    //         .conn
    //         .query_one(&self.auth.insert_user, &[&uuid, &name, &pass])
    //         .await?
    //         .into();
    //     Ok(SignUpAction::UserAdded(row.uuid))
    // }
    //
    // /// Tries to login with the given username & pass
    // ///
    // /// # Errors
    // ///
    // /// Fails when the database does
    // pub async fn login(&self, name: &str, pass: &str) -> Result<LoginAction, LoginError> {
    //     let Some(user) = self
    //         .auth
    //         .conn
    //         .query_opt(&self.auth.get_user, &[&name])
    //         .await?
    //         .map(User::from)
    //     else {
    //         return Ok(LoginAction::UsernameNotFound);
    //     };
    //
    //     match self.auth.validate(&user.password, pass) {
    //         Some(true) => Ok(LoginAction::LoggedIn(user.uuid)),
    //         Some(false) => Ok(LoginAction::IncorrectPassword),
    //         None => Err(LoginError::HashError),
    //     }
    // }
    //
    // async fn username_taken(&self, name: &str) -> Result<bool, PgError> {
    //     self.auth
    //         .conn
    //         .query_opt(&self.auth.get_user, &[&name])
    //         .await
    //         .map(|r| r.is_some())
    // }
}

/// The result of adding a user
#[derive(Debug)]
pub enum SignUpAction {
    /// username was taken
    UsernameTaken,
    /// given password was invalid
    InvalidPassword,
    /// user added successfully
    UserAdded(Uuid),
}

/// The result of logging in a user
#[derive(Debug)]
pub enum LoginAction {
    /// Username not found
    UsernameNotFound,
    /// given password was incorrect
    IncorrectPassword,
    /// logged in
    LoggedIn(Uuid),
}
