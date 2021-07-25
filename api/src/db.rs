use rocket_db_pools::{sqlx, Database};

#[derive(Clone, Database, Debug)]
#[database("primary_db")]
pub struct Primary(pub(crate) sqlx::PgPool);
