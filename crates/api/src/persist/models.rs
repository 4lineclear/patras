use axum_login::AuthUser;
use tokio_postgres::Row;
use uuid::Uuid;

/// A user's data
#[derive(Debug)]
pub struct User {
    /// A user's id
    pub id: i64,
    /// A user's uuid
    pub uuid: Uuid,
    /// A user name. Mostly used for authentication
    pub username: String,
    /// A (non plaintext) password
    pub password: String,
}

impl From<Row> for User {
    fn from(value: Row) -> Self {
        Self {
            id: value.get(0),
            uuid: value.get(1),
            username: value.get(2),
            password: value.get(3),
        }
    }
}

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        &self.password.as_bytes()
    }
}
