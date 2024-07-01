/// Dummy function to allow for highlight & format
#[allow(clippy::inline_always)]
#[inline(always)]
const fn sql(s: &str) -> &str {
    s
}

// TODO: create macro that automagically imports sql
// TODO: create treesitter-loc

pub const CREATE_USER_TABLE: &str = sql("\
CREATE TABLE IF NOT EXISTS users (
  id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  uuid UUID NOT NULL UNIQUE,
  username TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL
)");

pub const DROP_USER_TABLE: &str = sql("DROP TABLE IF EXISTS users");

pub const INSERT_USER: &str = sql("\
INSERT INTO
  users (uuid, username, password)
VALUES
  ($1, $2, $3)
RETURNING
  *");

pub const USERNAME_QUERY: &str = sql("\
SELECT
  *
FROM
  users
WHERE
  username = $1");

// use topoma::topoma;

macro_rules! topoma {
    ($($t:tt)*) => {};
}

// struct Yeah;
//
// impl Yeah {
//     const YEAH: () = panic!("Yeah!");
//     //
// }

topoma![
    table as CREATE_USER_TABLE
        >> r"
CREATE TABLE IF NOT EXISTS users (
  id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  uuid UUID NOT NULL UNIQUE,
  username TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL
);",
    drop as DROP_USER_TABLE
        >> r"
DROP TABLE IF EXISTS users",
    INSERT_USER
        >> r"
SELECT
  *
FROM
  users
WHERE
  username = $1",
    QUERY_USERNAME
        >> r"
SELECT
  *
FROM
  users
WHERE
  username = $1",
    ID_QUERY << "id_query",
];
