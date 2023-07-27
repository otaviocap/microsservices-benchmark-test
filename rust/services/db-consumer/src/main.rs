mod user;

use std::str::FromStr;

use sqlx::{postgres::PgPoolOptions, types::Uuid};

use crate::user::Users;

#[tokio::main]
async fn main() {

    println!("RUST => Db Consumer running!");

    dotenv::dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(
            std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set.")
            .as_str()
        )
        .await
        .expect("Error connecting into the databse!");

    let users = Users::new(pool);

    println!("{:?}", users.getAllUsers().await);
    println!("{:?}", users.getUserById(Uuid::parse_str("aa304752-2bf7-11ee-8732-0242ac140002").unwrap()).await);


}
