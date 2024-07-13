-- Add up migration script here
Create Table If Not Exists users (
  id BigInt Primary Key Generated Always As Identity,
  uuid Uuid Not Null Unique,
  username Text Not Null Unique,
  password Text Not Null
);
