/// Enables sql syntax highlighting, possibly also formatting in the future
macro_rules! sql {
    ($s:literal) => {
        $s
    };
    ($($name:ident = $s:literal),* $(,)?) => {
        $(
        pub const $name: &str = $s;
        )*
    };
}

sql!(
    // tables
    CREATE_TABLE = "\
        CREATE TABLE IF NOT EXISTS users (
            id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
            username text NOT NULL UNIQUE,
            password text NOT NULL
        );",
    DROP_TABLE = "DROP TABLE IF EXISTS users;",
    // users
    INSERT_USER = "INSERT INTO users (username, password) VALUES ($1. $2)",
    USERNAME_QUERY = "SELECT id FROM users WHERE username = $1",
);
