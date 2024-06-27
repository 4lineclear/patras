//! Persist yeah
#![allow(clippy::wildcard_imports)]
#![allow(clippy::enum_glob_use)]

use std::env::{self};

use deadpool_postgres::{Config, GenericClient, Object, Pool, Runtime};
use derivative::Derivative;
use sql::{CREATE_TABLE, INSERT_USER, USERNAME_QUERY};
use tokio_postgres::{NoTls, Statement};

use error::{db_to_conn, db_to_user, AddUserError, ConnectionError, InvalidPassword};

/// Auto generated schema
pub mod models;

/// Holds some queries
#[allow(dead_code)]
pub(crate) mod sql;

/// Errors
#[allow(clippy::module_name_repetitions)]
pub mod error;

/// The overarching database system
#[derive(Debug)]
pub struct Database {
    #[allow(dead_code)]
    pool: Pool,
    auth: AuthManager,
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
        let auth = AuthManager::new(pool.get().await.map_err(db_to_conn)?)
            .await
            .map_err(db_to_conn)?;
        add_tables(&pool).await.map_err(db_to_conn)?;
        Ok(Self { pool, auth })
    }
    /// Tries to add a user
    ///
    /// # Errors
    ///
    /// See [`AddUserError`]
    pub async fn add_user(&mut self, name: String, pass: String) -> Result<(), AddUserError> {
        let pass = hash_password(&pass)?;
        if self.auth.username_taken(&name).await.map_err(db_to_user)? {
            return Err(AddUserError::UsernameTaken);
        }
        Ok(())
    }
}

/// Adds the required tables
async fn add_tables(pool: &Pool) -> Result<(), tokio_postgres::Error> {
    pool.get().await.unwrap().execute(CREATE_TABLE, &[]).await?;
    Ok(())
}

impl api::Database for Database {
    type CE = ConnectionError;

    async fn new(url: Option<String>) -> Result<Self, Self::CE> {
        Self::new(url).await
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
struct AuthManager {
    manager: Object,
    #[derivative(Debug = "ignore")]
    username_taken: Statement,
    #[derivative(Debug = "ignore")]
    insert_user: Statement,
}

impl AuthManager {
    async fn new(manager: Object) -> Result<Self, tokio_postgres::Error> {
        let username_taken = manager.prepare(USERNAME_QUERY).await?;
        let insert_user = manager.prepare(INSERT_USER).await?;
        Ok(Self {
            manager,
            username_taken,
            insert_user,
        })
    }

    /// Checks if the given username is taken
    async fn username_taken(&mut self, name: &str) -> Result<bool, tokio_postgres::Error> {
        self.manager
            .query_opt(&self.username_taken, &[&name])
            .await
            .map(|r| r.is_some())
    }

    /// Adds a user to the table
    ///
    /// The given pass is expected to be properly configured
    async fn add_user(
        &mut self,
        name: &str,
        pass: DbPassword,
    ) -> Result<(), tokio_postgres::Error> {
        self.manager
            .execute(&self.insert_user, &[&name, &pass])
            .await?;
        Ok(())
    }
}

/// A hashed, salted and db ready string password
pub type DbPassword = String;

/// Opens a connection pool
fn open_pool(url: Option<String>) -> Result<Pool, ConnectionError> {
    const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/.env");
    dotenvy::from_path(PATH)?;
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
