//! Persist yeah
#![allow(clippy::wildcard_imports)]
#![allow(clippy::enum_glob_use)]

use std::env::{self};

use deadpool_postgres::{Config, GenericClient, Object, Pool, Runtime};
use derivative::Derivative;
use dotenvy::dotenv;
use sql::{CREATE_TABLE, USERNAME_QUERY};
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
pub struct DataBase {
    pool: Pool,
    auth: AuthManager,
}

impl DataBase {
    /// Creates a new database with the given db url
    ///
    /// # Errors
    ///
    /// See [`ConnectionError`]
    pub async fn new(url: Option<String>) -> Result<Self, ConnectionError> {
        dotenv()?;
        let pool = open_pool(url)?;
        let auth = AuthManager::new(pool.get().await.map_err(db_to_conn)?)
            .await
            .map_err(db_to_conn)?;
        Self::add_table(&pool).await.map_err(db_to_conn)?;
        Ok(Self { pool, auth })
    }
    /// Adds a table
    async fn add_table(pool: &Pool) -> Result<(), tokio_postgres::Error> {
        pool.get().await.unwrap().execute(CREATE_TABLE, &[]).await?;
        Ok(())
    }
    /// Tries to add a user
    ///
    /// # Errors
    ///
    /// See [`AddUserError`]
    pub async fn add_user(&mut self, name: String, pass: String) -> Result<(), AddUserError> {
        hash_password(&pass)?;
        if self.auth.username_taken(&name).await.map_err(db_to_user)? {
            return Err(AddUserError::UsernameTaken);
        }
        Ok(())
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
struct AuthManager {
    manager: Object,
    #[derivative(Debug = "ignore")]
    username_query: Statement,
}

impl AuthManager {
    async fn new(manager: Object) -> Result<Self, tokio_postgres::Error> {
        let username_query = manager.prepare(USERNAME_QUERY).await?;
        Ok(Self {
            manager,
            username_query,
        })
    }
    async fn username_taken(&mut self, name: &str) -> Result<bool, tokio_postgres::Error> {
        self.manager
            .query_opt(&self.username_query, &[&name])
            .await
            .map(|r| r.is_some())
    }
}

/// Opens a connection pool
fn open_pool(url: Option<String>) -> Result<Pool, ConnectionError> {
    dotenv()?;
    Ok((Config {
        url: Some(url.unwrap_or(env::var("DATABASE_URL")?)),
        ..Default::default()
    })
    .create_pool(Some(Runtime::Tokio1), NoTls)?)
}

/// Hashes the given password
fn hash_password(_password: &str) -> Result<String, InvalidPassword> {
    todo!()
}
