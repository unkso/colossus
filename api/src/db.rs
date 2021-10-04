use rocket::figment::Figment;
use rocket_db_pools::{sqlx, Config, Database};

#[derive(Clone, Database, Debug)]
#[database("primary_db")]
pub struct Primary(pub(crate) sqlx::PgPool);

#[derive(Debug)]
pub struct OrmConnection {
    pub conn: sea_orm::DatabaseConnection,
}

#[rocket::async_trait]
impl rocket_db_pools::Pool for OrmConnection {
    type Error = sea_orm::DbErr;

    type Connection = sea_orm::DatabaseConnection;

    async fn init(figment: &Figment) -> Result<Self, Self::Error> {
        let config = figment.extract::<Config>().unwrap();
        let conn = sea_orm::Database::connect(&config.url).await.unwrap();

        Ok(Self { conn })
    }

    async fn get(&self) -> Result<Self::Connection, Self::Error> {
        Ok(self.conn.clone())
    }
}
