-- Add up migration script here
CREATE TABLE users (
    id INTEGER NOT NULL CONSTRAINT PK_users PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL
)

