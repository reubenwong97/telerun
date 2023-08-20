#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub chat_id: String,
    pub user_name: String,
}

#[derive(sqlx::FromRow)]
pub struct Run {
    pub id: i32,
    pub distance: f32,
    pub user_id: i32,
}

pub struct Score {
    pub user_name: String,
    pub medals: u32,
    pub distance: f32,
}
