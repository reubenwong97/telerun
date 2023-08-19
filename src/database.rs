use sqlx::PgPool;
use teloxide::types::ChatId;

pub async fn create_user(
    user_name: &str,
    chat_id: ChatId,
    connection: PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO users (chat_id, user_name) VALUES ($1, $2)")
        .bind(chat_id.to_string())
        .bind(user_name.to_string())
        .execute(&connection)
        .await?;

    Ok(())
}
