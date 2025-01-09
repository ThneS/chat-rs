-- PostgreSQL
-- Add migration script here
CREATE TABLE IF NOT EXISTS users
(
    id SERIAL PRIMARY KEY,
    fullname VARCHAR(64) NOT NULL,
    email VARCHAR(64) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

CREATE TYPE CHAT_TYPE AS ENUM ('single', 'private', 'public');


CREATE TABLE IF NOT EXISTS chats
(
    id SERIAL PRIMARY KEY,
    chat_type CHAT_TYPE NOT NULL,
    members INT[] NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS messages
(
    id SERIAL PRIMARY KEY,
    chat_id INT NOT NULL REFERENCES chats(id),
    user_id INT NOT NULL REFERENCES users(id),
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
