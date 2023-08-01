
use axum::{Router, extract::{State, self}, routing::{get, post}, Json, debug_handler};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::utils::Configuration;

use super::model::User;

pub fn router() -> Router<Configuration> {

    Router::new()
        .route("/", get(all_users))
        .route("/", post(add_user))
        .route("/:id", get(single_user))
}

async fn all_users(State(config): State<Configuration>) -> Json<Vec<User>> {
    Json(config.users.get_all_users().await)
}

async fn single_user(
    extract::Path((id,)) : extract::Path<(Uuid, )>,
    State(config): State<Configuration>
) -> Json<User> {
    Json(config.users.get_user_by_id(id).await)
}

async fn add_user(
    State(config): State<Configuration>,
    extract::Json(user): extract::Json<User>
) -> Json<Value> {

    Json(json!({ "id": config.users.add_user(user).await.unwrap() }))
}