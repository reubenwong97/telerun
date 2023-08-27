-- Add migration script here
ALTER TABLE users
ADD COLUMN telegram_userid BIGINT NOT NULL DEFAULT 0;
ALTER TABLE users
ALTER COLUMN telegram_userid DROP DEFAULT;