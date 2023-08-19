use crate::models::{Run, User};
use sqlx::{postgres::PgRow, PgPool};
use teloxide::types::ChatId;

pub async fn create_user(
    user_name: &str,
    chat_id: ChatId,
    connection: PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO users (chat_id, user_name) 
        VALUES ($1, $2)
        ON CONFLICT (chat_id, user_name) DO NOTHING",
    )
    .bind(chat_id.to_string())
    .bind(user_name)
    .execute(&connection)
    .await?;

    Ok(())
}

pub async fn delete_user(
    user_name: &str,
    chat_id: ChatId,
    connection: PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE chat_id = $1 AND user_name = $2")
        .bind(chat_id.to_string())
        .bind(user_name)
        .execute(&connection)
        .await?;

    Ok(())
}

async fn get_user(
    user_name: &str,
    chat_id: ChatId,
    connection: PgPool,
) -> Result<Option<User>, sqlx::Error> {
    let user: Option<User> = sqlx::query_as!(
        User,
        "SELECT id, chat_id, user_name
    FROM users
    WHERE user_name = $1 AND chat_id = $2",
        user_name,
        chat_id.to_string()
    )
    .fetch_optional(&connection)
    .await?;

    Ok(user)
}

pub async fn add_run_wrapper(
    distance: f32,
    user_name: &str,
    chat_id: ChatId,
    connection: PgPool,
) -> Result<(), sqlx::Error> {
    Ok(())
}

async fn add_run(distance: f32, user_id: i32, connection: PgPool) -> Result<(), sqlx::Error> {
    Ok(())
}
