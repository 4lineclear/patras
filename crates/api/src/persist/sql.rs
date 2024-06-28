/// Enables sql syntax highlighting, possibly also formatting in the future
macro_rules! sql {
    ($s:literal) => {
        $s
    };
    ($($(#[$m:meta])* $name:ident = $s:literal),* $(,)?) => {
        $(
        $(#[$m])*
        pub const $name: &str = $s;
        )*
    };
}

// TODO: create new sql crate
// topg-quma OR topo-quma:
// tokio-postgres-query-macros
sql!(
    /// Creates a table
    ///
    /// Should be batch executed
    CREATE_USER_TABLE = r#"\
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    uuid UUID NOT NULL UNIQUE,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
);"#,
    /// Drops a table
    DROP_USER_TABLE = "DROP TABLE IF EXISTS users",
    INSERT_USER = "INSERT INTO users (username, password) VALUES ($1, $2)",
    USERNAME_QUERY = "SELECT * FROM users WHERE username = $1",
);
