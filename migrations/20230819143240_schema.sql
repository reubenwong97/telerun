-- Add migration script here
ALTER TABLE users
ADD CONSTRAINT unique_user_chat UNIQUE(chat_id, user_name);