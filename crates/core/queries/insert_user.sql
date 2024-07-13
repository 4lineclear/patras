Insert Into
  users (uuid, username, password)
Values
  ($1, $2, $3)
Returning
  *
