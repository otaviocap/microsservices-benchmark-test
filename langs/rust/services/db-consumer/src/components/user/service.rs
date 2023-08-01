use sqlx::{types::Uuid, Pool, Postgres};
use super::model::User;

#[derive(Clone)]
pub struct Users {
    pool: Pool<Postgres>
}

#[allow(dead_code)]
impl Users {

    pub fn new(pool: Pool<Postgres>) -> Users {
        Users { pool }
    }

    pub async fn get_all_users(&self) -> Vec<User> {
        sqlx::query_as!(User, r#"--sql
            SELECT * FROM users;
        "#)
        .fetch_all(&self.pool)
        .await
        .expect("Something went wrong while querying users")
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> User{
        sqlx::query_as!(User, r#"--sql
            SELECT 
                username,
                password,
                id
            FROM users
            WHERE id = $1
        "#, id)
        .fetch_one(&self.pool)
        .await
        .expect(format!("Something went wrong while querying user with id: {:?}", id).as_str()) 
    }

    pub async fn add_user(&self, user: User) -> Result<Uuid, ()>{
        let rec = sqlx::query!(r#"--sql
            INSERT INTO users (username, password)
            VALUES ($1, $2)
            RETURNING id
        "#, user.username, user.password)
        .fetch_one(&self.pool)
        .await
        .expect("Something went wrong while querying users");

        Ok(rec.id)
    }


}