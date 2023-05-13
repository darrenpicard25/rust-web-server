-- Add up migration script here

CREATE TABLE todos
(
    id              UUID PRIMARY KEY UNIQUE NOT NULL,
    title           TEXT NOT NULL,
    description     TEXT NOT NULL,
    created_at      TIMESTAMP NOT NULL,
    updated_at      TIMESTAMP NOT NULL
);

