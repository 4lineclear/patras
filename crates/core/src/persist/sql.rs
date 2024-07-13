use topoma::topoma;

topoma![
    CREATE_USER_TABLE
        >> r"
Create Table If Not Exists users (
  id Integer Primary Key Generated Always As Identity,
  uuid Uuid Not Null Unique,
  username Text Not Null Unique,
  password Text Not Null
);
",
    DROP_USER_TABLE
        >> r"
Drop Table If Exists users
",
    INSERT_USER
        >> r"
Insert Into
  users (uuid, username, password)
Values
  ($1, $2, $3)
Returning
  *
",
    USERNAME_QUERY
        >> r"
Select
  *
From
  users
Where
  username = $1
"
];

// TODO:
// add string literal documentation, rather than attributes.
// add statement struct generation
// add syntax "<< $item as $type"
