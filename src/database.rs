use crate::models::{Run, Score, User};
use sqlx::PgPool;
use teloxide::types::ChatId;

type DBResult<T> = Result<T, sqlx::Error>;

pub async fn create_user(user_name: &str, chat_id: ChatId, connection: &PgPool) -> DBResult<()> {
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

pub async fn delete_user(user_name: &str, chat_id: ChatId, connection: &PgPool) -> DBResult<()> {
    sqlx::query!(
        "DELETE FROM users WHERE chat_id = $1 AND user_name = $2",
        chat_id.to_string(),
        user_name
    )
    .execute(connection)
    .await?;

    Ok(())
}

async fn get_user(user_name: &str, chat_id: ChatId, connection: &PgPool) -> DBResult<Option<User>> {
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

pub async fn get_users_in_chat(
    chat_id: ChatId,
    connection: &PgPool,
) -> DBResult<Option<Vec<User>>> {
    let users: Vec<User> = sqlx::query!(
        "SELECT id, chat_id, user_name
        FROM users
        WHERE chat_id = $1",
        chat_id.to_string()
    )
    .fetch_all(connection)
    .await?
    .iter()
    .map(|user_row| User {
        id: user_row.id,
        chat_id: user_row.chat_id.clone(),
        user_name: user_row.user_name.clone(),
    })
    .collect();

    if !users.is_empty() {
        Ok(Some(users))
    } else {
        Ok(None)
    }
}

// TODO: refactor in the future
pub async fn add_run_wrapper(
    distance: f32,
    user_name: &str,
    chat_id: ChatId,
    connection: &PgPool,
) -> DBResult<()> {
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

async fn add_run(distance: f32, user_id: i32, connection: &PgPool) -> DBResult<()> {
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

pub async fn get_runs(
    chat_id: ChatId,
    limit: i64,
    connection: &PgPool,
) -> DBResult<Option<Vec<Run>>> {
    let users_in_chat = get_users_in_chat(chat_id, connection).await?;
    if let Some(users) = users_in_chat {
        let user_ids: Vec<i32> = users.iter().map(|user| user.id).collect();
        let runs: Vec<_> = sqlx::query!(
            "SELECT *
            FROM runs
            WHERE user_id = ANY($1)
            ORDER BY run_datetime DESC
            LIMIT $2",
            &user_ids[..],
            limit
        )
        .fetch_all(connection)
        .await?
        .iter()
        .map(|row| Run {
            id: row.id,
            distance: row.distance,
            run_datetime: row.run_datetime,
            user_id: row.user_id,
        })
        .collect();

        if !runs.is_empty() {
            Ok(Some(runs))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

pub async fn update_run(run_id: i32, distance: f32, connection: &PgPool) -> DBResult<()> {
    sqlx::query!(
        "UPDATE runs
        SET distance = $1
        WHERE id = $2",
        distance,
        run_id,
    )
    .execute(connection)
    .await?;

    Ok(())
}

pub async fn delete_run(run_id: i32, connection: &PgPool) -> DBResult<()> {
    sqlx::query!(
        "DELETE FROM runs
        WHERE id = $1",
        run_id,
    )
    .execute(connection)
    .await?;

    Ok(())
}

pub async fn get_tally(chat_id: ChatId, connection: &PgPool) -> DBResult<Option<Vec<Score>>> {
    let users = get_users_in_chat(chat_id, connection).await?;

    if let Some(users) = users {
        let user_ids: Vec<i32> = users.iter().map(|user| user.id).collect();
        let tally = sqlx::query!(
            "SELECT user_name, COUNT(*), SUM(distance)
            FROM runs
            JOIN users ON users.id = runs.user_id
            WHERE user_id = ANY($1)
            GROUP BY user_name",
            &user_ids[..],
        )
        .fetch_all(connection)
        .await?;

        let scores: Vec<Score> = tally
            .iter()
            .map(|tally| Score {
                user_name: tally.user_name.clone(),
                medals: tally.count.unwrap() as u32,
                distance: tally.sum.unwrap(),
            })
            .collect();

        return Ok(Some(scores));
    }

    Ok(None)
}
