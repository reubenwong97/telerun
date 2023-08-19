-- Add migration script here
CREATE TABLE IF NOT EXISTS chats (id bigserial PRIMARY KEY);
CREATE TABLE IF NOT EXISTS users (
    id serial PRIMARY KEY,
    chat_id bigserial REFERENCES chats(id),
    user_name varchar(32) NOT NULL
);
CREATE TABLE IF NOT EXISTS runs (
    id serial PRIMARY KEY,
    distance real NOT NULL,
    medals integer NOT NULL,
    user_id serial REFERENCES users(id)
);