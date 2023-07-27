use sqlx::{types::Uuid, Pool, Postgres};

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub id: Uuid,
}

pub struct Users {
    pool: Pool<Postgres>
}

impl Users {

    pub fn new(pool: Pool<Postgres>) -> Users {
        Users { pool }
    }

    pub async fn getAllUsers(&self) -> Vec<User> {
        sqlx::query_as!(User, r#"--sql
            SELECT * FROM users;
        "#)
        .fetch_all(&self.pool)
        .await
        .expect("Something went wrong while querying users")
    }

    pub async fn getUserById(&self, id: Uuid) -> User{
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

    pub async fn addUser(&self, user: User) -> Result<Uuid, ()>{
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