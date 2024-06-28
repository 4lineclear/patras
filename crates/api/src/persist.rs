//! Persist yeah
#![allow(clippy::wildcard_imports)]
#![allow(clippy::enum_glob_use)]

use std::env::{self};

use deadpool_postgres::{Config, GenericClient, Object, Pool, Runtime};
use derivative::Derivative;
use sql::{CREATE_USER_TABLE, INSERT_USER, USERNAME_QUERY};
use tokio_postgres::{Error as PgError, NoTls, Statement};

use error::ConnectionError;
use uuid::Uuid;

/// Auto generated schema
pub mod models;

/// Holds some queries
#[allow(dead_code)]
pub(crate) mod sql;

/// Errors
#[allow(clippy::module_name_repetitions)]
pub mod error;

/// The overarching database system
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Database {
    /// A pool of database conenctions
    #[allow(dead_code)]
    pool: Pool,
    /// Statements to be used with `auth_conn`
    #[derivative(Debug = "ignore")]
    statements: Statements,
    /// A db connection for auth queries
    auth_conn: Object,
}

impl Database {
    /// Creates a new database with the given db url
    ///
    /// Also creates the required tables if they don't already exist
    ///
    /// # Errors
    ///
    /// See [`ConnectionError`]
    pub async fn new(url: Option<String>) -> Result<Self, ConnectionError> {
        let pool = open_pool(url)?;
        let auth_conn = pool.get().await?;
        let statements = Statements::new(&auth_conn).await?;
        add_tables(&pool).await?;

        Ok(Self {
            pool,
            statements,
            auth_conn,
        })
    }

    /// Tries to add a user
    ///
    /// # Errors
    ///
    /// See [`AddUserError`]
    pub async fn add_user(&mut self, name: &str, pass: &str) -> Result<AddUserAction, PgError> {
        let pass = match hash_password(pass) {
            Ok(p) => p,
            Err(ip) => return Ok(AddUserAction::InvalidPassword(ip)),
        };
        if self.username_taken(name).await? {
            return Ok(AddUserAction::UsernameTaken);
        }
        self.auth_conn
            .execute(&self.statements.insert_user, &[&name, &pass])
            .await?;
        todo!()
        // Ok(AddUserAction::UserAdded())
    }

    async fn username_taken(&mut self, name: &str) -> Result<bool, PgError> {
        self.auth_conn
            .query_opt(&self.statements.username_taken, &[&name])
            .await
            .map(|r| r.is_some())
    }
}

/// The result of adding a user
#[derive(Debug)]
pub enum AddUserAction {
    /// username was taken
    UsernameTaken,
    /// given password was invalid
    InvalidPassword(InvalidPassword),
    /// user added successfully
    UserAdded(Uuid),
}

/// Adds the required tables
async fn add_tables(pool: &Pool) -> Result<(), PgError> {
    pool.get()
        .await
        .unwrap()
        .batch_execute(CREATE_USER_TABLE)
        .await?;
    Ok(())
}

/// Holds all premade statements
struct Statements {
    username_taken: Statement,
    insert_user: Statement,
}

impl Statements {
    async fn new(manager: &Object) -> Result<Self, PgError> {
        let username_taken = manager.prepare(USERNAME_QUERY).await?;
        let insert_user = manager.prepare(INSERT_USER).await?;
        Ok(Self {
            username_taken,
            insert_user,
        })
    }
}

/// The default path where an env file is read
pub const DEFAULT_ENV_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/.env");

/// Opens a connection pool
fn open_pool(url: Option<String>) -> Result<Pool, ConnectionError> {
    dotenvy::from_path(DEFAULT_ENV_PATH)?;
    Ok((Config {
        url: Some(dbg!(url.unwrap_or(env::var("DATABASE_URL")?))),
        ..Default::default()
    })
    .create_pool(Some(Runtime::Tokio1), NoTls)?)
}

/// Hashes the given password
fn hash_password(_password: &str) -> Result<String, InvalidPassword> {
    todo!()
}

/// The result of adding a password
#[derive(Debug)]
pub enum InvalidPassword {
    /// Password is shorter than the allowed min
    TooShort(usize),
    /// Password is longer than the allowed max
    TooLong(usize),
}
