//! Struct models for database tables.
//!
//! Contains structs for an "ORM-like" approach to
//! database interactions.

use sqlx::types::chrono;

/// Represents a user row in the `users` table.
#[derive(sqlx::FromRow)]
pub struct User {
    /// User id
    pub id: i32,
    /// User id as seen from Telegram
    /// PostgreSQL does not natively support u64 values,
    /// and thus we will attempt to cast the values from Telegram
    /// as i64 first before storing in DB.
    pub telegram_userid: i64,
    /// Id of telegram chat
    pub chat_id: String,
    /// Self-specified username
    pub user_name: String,
}

/// Represents a run row in the `runs` table.
#[derive(sqlx::FromRow)]
pub struct Run {
    /// Run id
    pub id: i32,
    /// Distance ran for a particular run
    pub distance: f32,
    /// Datetime when the run was submitted to the database
    pub run_datetime: Option<chrono::NaiveDateTime>,
    /// User_id of the user who submitted the run
    pub user_id: i32,
}

/// Represents a score that appears in the tally.
///
/// While this struct those not correspond direclty to a database
/// table, it is built directly from results retrieved.
pub struct Score {
    /// Self-specified username
    pub user_name: String,
    /// Number of runs for the user, or in this case, medals
    pub medals: u32,
    /// Total distance run by the user
    pub distance: f32,
}
