-- Add migration script here
CREATE TABLE IF NOT EXISTS cowsay.clients (
    id TEXT PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    token TEXT UNIQUE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP DEFAULT NULL
);
