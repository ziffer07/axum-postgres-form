-- Add migration script here
CREATE EXTENSION IF NOT EXISTS citext;

CREATE TABLE IF NOT EXISTS contents (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email CITEXT NOT NULL
);
