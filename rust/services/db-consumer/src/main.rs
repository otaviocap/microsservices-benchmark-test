mod user;

use sqlx::postgres::PgPoolOptions;
use user::User;

#[tokio::main]
async fn main() {

    dotenv::dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.").as_str())
        .await
        .expect("Error connecting into the databse!");

    let rows = sqlx::query_as!(User, r#"--sql
        SELECT * FROM users;
    "#)
    .fetch_all(&pool)
    .await
    .expect("Something went wrong while querying users");

    println!("{:?}", rows)


}
