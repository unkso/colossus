use sqlx::{query_as, types::Uuid, Error, FromRow, PgConnection};

#[derive(FromRow)]
pub struct User {
    pub id: Uuid,
    username: String,
    password: String,
    email: String,
}

impl User {
    pub async fn get_all(mut conn: PgConnection) -> Result<Vec<User>, Error> {
        query_as!(User, "SELECT * FROM users")
            .fetch_all(&mut conn)
            .await
    }
}
