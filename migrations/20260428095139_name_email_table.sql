-- Add migration script here
CREATE EXTENSION IF NOT EXISTS citext;

CREATE TABLE IF NOT EXISTS contents (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email CITEXT NOT NULL,
    title TEXT NOT NULL DEFAULT 'Untitled',
    description TEXT NOT NULL DEFAULT 'No description provided'
);
