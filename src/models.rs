#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub chat_id: String,
    pub user_name: String,
}

#[derive(sqlx::FromRow)]
pub struct Run {
    id: i32,
    distance: f32,
    user_id: i32,
}
