use serde::Serialize;
use sqlx::types::Uuid;

#[derive(sqlx::FromRow, Debug, Default, Serialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub id: Option<Uuid>,
}