-- Add migration script here
ALTER TABLE users
ALTER COLUMN chat_id TYPE VARCHAR;