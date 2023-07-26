use sqlx::types::Uuid;

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub id: Uuid,
    pub status: Option<String>,
}