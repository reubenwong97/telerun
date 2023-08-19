use crate::models::{Run, User};
use sqlx::{postgres::PgRow, PgPool};
use teloxide::types::ChatId;

pub async fn create_user(
    user_name: &str,
    chat_id: ChatId,
    connection: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO users (chat_id, user_name) 
        VALUES ($1, $2)
        ON CONFLICT (chat_id, user_name) DO NOTHING",
        chat_id.to_string(),
        user_name
    )
    .execute(connection)
    .await?;

    Ok(())
}

pub async fn delete_user(
    user_name: &str,
    chat_id: ChatId,
    connection: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "DELETE FROM users WHERE chat_id = $1 AND user_name = $2",
        chat_id.to_string(),
        user_name
    )
    .execute(connection)
    .await?;

    Ok(())
}

async fn get_user(
    user_name: &str,
    chat_id: ChatId,
    connection: &PgPool,
) -> Result<Option<User>, sqlx::Error> {
    let user: Option<User> = sqlx::query_as!(
        User,
        "SELECT id, chat_id, user_name
    FROM users
    WHERE user_name = $1 AND chat_id = $2",
        user_name,
        chat_id.to_string()
    )
    .fetch_optional(connection)
    .await?;

    Ok(user)
}

pub async fn add_run_wrapper(
    distance: f32,
    user_name: &str,
    chat_id: ChatId,
    connection: PgPool,
) -> Result<(), sqlx::Error> {
    let user = get_user(user_name, chat_id, &connection).await?;

    if let Some(user) = user {
        add_run(distance, user.id, &connection).await?;
    } else {
        create_user(user_name, chat_id, &connection).await?;
        let user = get_user(user_name, chat_id, &connection).await?;
        if let Some(user) = user {
            add_run(distance, user.id, &connection).await?;
        } else {
            error!("Unable to add run to database.");
        }
    }

    Ok(())
}

async fn add_run(distance: f32, user_id: i32, connection: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO runs (distance, user_id)
    VALUES ($1, $2)
    ",
        distance,
        user_id,
    )
    .execute(connection)
    .await?;

    Ok(())
}
