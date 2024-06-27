//! The actual api used by the core server

use std::error::Error;

use derivative::Derivative;
use libreauth::pass::Hasher;

/// The central auth authority
#[derive(Derivative)]
#[derivative(Debug)]
pub struct AuthSession<DB> {
    database: DB,
    #[derivative(Debug = "ignore")]
    #[allow(dead_code)]
    hasher: Hasher,
}

impl<DB> AuthSession<DB>
where
    DB: Database,
{
    /// Creates a new auth session
    ///
    /// # Errors
    ///
    /// Fails when [`Database::new`] does.
    pub async fn new(url: Option<String>, hasher: Hasher) -> Result<Self, DB::CE> {
        Ok(Self {
            database: DB::new(url).await?,
            hasher,
        })
    }
    /// Adds a user
    pub async fn add_user(name: &str, pass: &str) -> Result<(), ()> {
        todo!()
        //
    }
}

/// A database
#[allow(async_fn_in_trait)]
#[trait_variant::make(Send)]
pub trait Database: core::fmt::Debug + Sized {
    /// A connection error
    type CE: Error;
    /// Creates a new DB
    async fn new(url: Option<String>) -> Result<Self, Self::CE>;
}

// TODO: everything
