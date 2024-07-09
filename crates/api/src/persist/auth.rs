use deadpool_postgres::Object;
use derivative::Derivative;
use libreauth::pass::{HashBuilder, Hasher};
use tokio_postgres::Statement;

use crate::persist::sql::{INSERT_USER, USERNAME_QUERY};

/// The central auth authority
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Auth {
    #[derivative(Debug = "ignore")]
    hasher: Hasher,
    #[derivative(Debug = "ignore")]
    pub(super) get_user: Statement,
    #[derivative(Debug = "ignore")]
    pub(super) insert_user: Statement,
    /// A db connection for auth queries
    pub(super) conn: Object,
}

impl Auth {
    /// Creates a new auth sessions
    ///
    /// # Errors
    ///
    /// An error while preparing statements
    pub async fn new(hasher: Hasher, conn: Object) -> Result<Self, tokio_postgres::error::Error> {
        let username_taken = conn.prepare(USERNAME_QUERY).await?;
        let insert_user = conn.prepare(INSERT_USER).await?;

        Ok(Self {
            hasher,
            get_user: username_taken,
            insert_user,
            conn,
        })
    }

    /// Hashes the given password
    ///
    /// # Errors
    ///
    /// Fails when the hasher does
    #[must_use]
    pub fn hash(&self, password: &str) -> Option<String> {
        use libreauth::pass::Error::*;

        match self.hasher.hash(password) {
            Ok(s) => Some(s),
            Err(PasswordTooShort { .. } | PasswordTooLong { .. } | InvalidPasswordFormat) => None,
        }
    }

    /// Valides the given hash and the given password to check
    #[must_use]
    pub fn validate(&self, hashed: &str, to_check: &str) -> Option<bool> {
        use libreauth::pass::Error::*;

        match HashBuilder::from_phc(hashed) {
            Ok(h) => Some(h.is_valid(to_check)),
            Err(PasswordTooShort { .. } | PasswordTooLong { .. }) => Some(false),
            Err(InvalidPasswordFormat) => None,
        }
    }
}
