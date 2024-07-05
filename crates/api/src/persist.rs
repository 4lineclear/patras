//! Persist yeah
#![allow(clippy::wildcard_imports)]
#![allow(clippy::enum_glob_use)]

use deadpool_postgres::{Config, GenericClient, Object, Pool, Runtime};
use derivative::Derivative;
use libreauth::pass::{HashBuilder, Hasher};
use models::User;
use sql::{CREATE_USER_TABLE, INSERT_USER, USERNAME_QUERY};
use tokio_postgres::{Error as PgError, NoTls, Statement};

use error::{ConnectionError, LoginError, SignUpError};
use uuid::Uuid;

/// Auto generated schema
pub mod models;

/// Holds some queries
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

/// Opens a connection pool
async fn open_pool(url: String) -> Result<Pool, ConnectionError> {
    let pool = (Config {
        url: Some(url),
        ..Default::default()
    })
    .create_pool(Some(Runtime::Tokio1), NoTls)?;

    pool.get().await?.execute(CREATE_USER_TABLE, &[]).await?;
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
    pub async fn new(url: String) -> Result<Self, ConnectionError> {
        let pool = open_pool(url).await?;
        let auth_conn = pool.get().await?;
        let statements = Statements::new(&auth_conn).await?;

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
    /// Fails when the database does
    pub async fn sign_up(
        &self,
        name: &str,
        pass: &str,
        hasher: &Hasher,
    ) -> Result<SignUpAction, SignUpError> {
        use libreauth::pass::Error::*;

        let pass = match hasher.hash(pass) {
            Ok(s) => s,
            Err(PasswordTooLong { .. } | PasswordTooShort { .. }) => {
                return Ok(SignUpAction::InvalidPassword)
            }
            Err(InvalidPasswordFormat) => return Err(SignUpError::HashError),
        };
        if self.username_taken(name).await? {
            return Ok(SignUpAction::UsernameTaken);
        }
        let row = self
            .auth_conn
            .query_one(&self.statements.insert_user, &[&name, &pass])
            .await?;
        Ok(SignUpAction::UserAdded(row.get(1)))
    }

    /// Tries to login with the given username & pass
    ///
    /// # Errors
    ///
    /// Fails when the database does
    pub async fn login(&self, name: &str, pass: &str) -> Result<LoginAction, LoginError> {
        use libreauth::pass::Error::*;
        let Some(user) = self
            .auth_conn
            .query_opt(&self.statements.username_taken, &[&name])
            .await?
            .map(User::from)
        else {
            return Ok(LoginAction::UsernameNotFound);
        };

        match HashBuilder::from_phc(&user.password) {
            Ok(h) if h.is_valid(pass) => (),
            Ok(_) => return Ok(LoginAction::IncorrectPassword),
            Err(PasswordTooShort { .. } | PasswordTooLong { .. }) => {
                return Ok(LoginAction::IncorrectPassword)
            }
            Err(InvalidPasswordFormat) => return Err(LoginError::HashError),
        }
        Ok(LoginAction::LoggedIn(user.uuid))
    }

    async fn username_taken(&self, name: &str) -> Result<bool, PgError> {
        self.auth_conn
            .query_opt(&self.statements.username_taken, &[&name])
            .await
            .map(|r| r.is_some())
    }
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

/// Adds the required tables
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

/// The result of hashing a password
#[derive(Debug)]
pub enum HashAction {
    /// The hashed + salted password
    Hashed(String),
    /// Password is shorter than the allowed min
    TooShort,
    /// Password is longer than the allowed max
    TooLong,
}
