use axum_login::AuthUser;
use sqlx::prelude::FromRow;
// use tokio_postgres::Row;
use uuid::Uuid;

/// A user's data
#[derive(Clone, FromRow)]
pub struct User {
    /// A user's id
    pub id: i32,
    /// A user's uuid
    pub uuid: Uuid,
    /// A user name. Mostly used for authentication
    pub username: String,
    /// A (non plaintext) password
    pub password: String,
}

// Here we've implemented `Debug` manually to avoid accidentally logging the
// password hash.
impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("uuid", &self.uuid)
            .field("username", &self.username)
            .field("password", &"[redacted]")
            .finish()
    }
}

impl AuthUser for User {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes()
    }
}
