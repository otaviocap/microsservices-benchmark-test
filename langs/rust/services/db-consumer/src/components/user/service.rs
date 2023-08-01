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

    pub async fn get_all(&self) -> Vec<User> {
        sqlx::query_as!(User, r#"--sql
            SELECT * FROM users;
        "#)
        .fetch_all(&self.pool)
        .await
        .expect("Something went wrong while querying users")
    }

    pub async fn get(&self, id: Uuid) -> User{
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
        .expect(&format!("Something went wrong while querying user with id: {id:?}")) 
    }

    pub async fn add(&self, user: User) -> Result<Uuid, ()>{
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

    pub async fn edit(&self, id: Uuid, user: User) {
        let rec = sqlx::query!(r#"--sql
            UPDATE users
            SET
                username = $1,
                password = $2
            WHERE
                id = $3
        "#, user.username, user.password, id)
        .execute(&self.pool)
        .await
        .expect("Something went wrong while updating user with id: {id:?}");
    }

    pub async fn delete(&self, id: Uuid) {
        let rec = sqlx::query!(r#"--sql
            DELETE FROM users
            WHERE
                id = $1
        "#, id)
        .execute(&self.pool)
        .await
        .expect("Something went wrong while deleting user with id: {id:?}");
    }


}