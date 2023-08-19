-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id serial PRIMARY KEY,
    user_name varchar(32) NOT NULL
);

CREATE TABLE IF NOT EXISTS chats (
    id bigserial PRIMARY KEY,
);

CREATE TABLE IF NOT EXISTS runs (
    id serial PRIMARY KEY,
    distance real NOT NULL,
    medals integer NOT NULL,
    constraint user_id
        foreign key(id)
            references users(id)
    constraint chat_id
        foreign key(id)
            references chats(id)
);
