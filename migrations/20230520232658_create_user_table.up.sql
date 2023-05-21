-- Add up migration script here

CREATE TABLE users
(
    id              UUID PRIMARY KEY UNIQUE NOT NULL,
    email           TEXT NOT NULL UNIQUE,
    first_name      TEXT NOT NULL,
    created_at      TIMESTAMP NOT NULL,
    updated_at      TIMESTAMP NOT NULL
);

