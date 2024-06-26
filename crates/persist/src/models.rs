/// A user's data
pub struct User {
    /// A user's id
    pub id: i32,
    /// A user name. Mostly used for authentication
    pub username: String,
    /// A (non plaintext) password
    pub password: String,
}
