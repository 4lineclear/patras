use async_trait::async_trait;
use axum_login::{AuthnBackend, AuthzBackend, UserId};
use password_auth::verify_password;
use serde::Deserialize;
use sqlx::PgPool;
use tokio::task;

use crate::models::User;

/// A sessions for auth
pub type AuthSession = axum_login::AuthSession<Backend>;

/// This allows us to extract the authentication fields from forms. We use this
/// to authenticate requests with the backend.
#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    /// Username
    pub username: String,
    /// Password
    pub password: String,
}

/// Auth backend
#[derive(Debug, Clone)]
pub struct Backend {
    pool: PgPool,
}

impl Backend {
    /// Creates a new auth backend, using the given pool
    #[must_use]
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

/// An error during auth
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// DB error
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    /// Async error
    TaskJoin(#[from] task::JoinError),
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = Error;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let Some(user) = sqlx::query_file_as!(User, "queries/select_username.sql", creds.username)
            .fetch_optional(&self.pool)
            .await?
        else {
            return Ok(None);
        };

        task::spawn_blocking(|| {
            Ok(verify_password(creds.password, &user.password)
                .ok()
                .map(|()| user))
        })
        .await?
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_file_as!(User, "queries/select_id.sql", user_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }
}

#[async_trait]
impl AuthzBackend for Backend {
    /// Permission type.
    type Permission = ();
}
