use topoma::topoma;

topoma![
    CREATE_USER_TABLE
        >> r"
CREATE TABLE IF NOT EXISTS users (
  id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  uuid UUID NOT NULL UNIQUE,
  username TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL
);
",
    DROP_USER_TABLE
        >> r"
DROP TABLE IF EXISTS users",
    INSERT_USER
        >> r"
INSERT INTO
  users (uuid, username, password)
VALUES
  ($1, $2, $3)
RETURNING
  *
",
    USERNAME_QUERY
        >> r"
SELECT
  *
FROM
  users
WHERE
  username = $1
",
];

// TODO: add a way to add documentation
