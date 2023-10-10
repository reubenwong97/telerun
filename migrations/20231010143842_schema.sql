-- Add migration script here
ALTER TABLE users ALTER COLUMN telegram_userid TYPE varchar(20) USING telegram_userid::varchar(20);
