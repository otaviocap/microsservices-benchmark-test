
use axum::{Router, extract::{State, self}, routing::{get, post, delete}, Json, Error, response::Response, body::Body};
use hyper::StatusCode;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::utils::Configuration;

use super::model::User;

pub fn router() -> Router<Configuration> {

    Router::new()
        .route("/", get(all_users))
        .route("/", post(add_user))
        .route("/:id", get(single_user))
        .route("/:id", post(edit_user))
        .route("/:id", delete(delete_user))
}

async fn all_users(State(config): State<Configuration>) -> Json<Vec<User>> {
    Json(config.users.get_all().await)
}

async fn single_user(
    extract::Path((id,)) : extract::Path<(Uuid, )>,
    State(config): State<Configuration>
) -> Json<User> {
    Json(config.users.get(id).await)
}

async fn add_user(
    State(config): State<Configuration>,
    extract::Json(user): extract::Json<User>
) -> Json<Value> {

    Json(json!({ "id": config.users.add(user).await.unwrap() }))
}

async fn edit_user(
    State(config): State<Configuration>,
    extract::Path((id,)) : extract::Path<(Uuid, )>,
    extract::Json(user): extract::Json<User>
) -> StatusCode {

    config.users.edit(id, user).await;

    StatusCode::OK
}

async fn delete_user(
    extract::Path((id,)) : extract::Path<(Uuid, )>,
    State(config): State<Configuration>
) -> StatusCode {
    config.users.delete(id).await;

    StatusCode::OK
}