use axum::Router;
use sqlx::postgres::PgPoolOptions;

use crate::{utils::Configuration, components::user::{self, service::Users}};

pub async fn build() -> Router {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(
            std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set.")
            .as_str()
        )
        .await
        .expect("Error connecting into the databse!");

    let config = Configuration {
        users: Users::new(pool)
    };

    Router::new()
        .nest("/users", user::use_component())
        .with_state(config)
}