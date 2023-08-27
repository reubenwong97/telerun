-- Add migration script here
ALTER TABLE users DROP CONSTRAINT unique_user_chat;
ALTER TABLE users
ADD CONSTRAINT unique_user_chat UNIQUE(telegram_userid, chat_id, user_name);