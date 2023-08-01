use serde::{Serialize, Deserialize};
use sqlx::types::Uuid;

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Default)]
pub struct User {
    pub username: String,
    pub password: String,
    pub id: Option<Uuid>,
}