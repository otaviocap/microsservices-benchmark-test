
use axum::{Router, extract::State, routing::get, Json};

use crate::utils::Configuration;

use super::model::User;

pub fn router() -> Router<Configuration> {

    Router::new().route("/", get(all_users))
}

async fn all_users(State(config): State<Configuration>) -> Json<Vec<User>> {
    Json(config.users.get_all_users().await)
}

