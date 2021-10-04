use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::repositories::Repository;

/// The User model
#[derive(sqlx::FromRow, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct User {
    /// Primary ID
    pub id: Uuid,
    /// User's name
    pub username: String,
    /// User's password
    pub password: String,
    /// User's email
    pub email: String,
    /// Date of last activity
    pub last_active: NaiveDateTime,
    /// Date the entry was created
    pub created_at: NaiveDateTime,
    /// Date of the entry's last update
    pub updated_at: NaiveDateTime,
}

/// Repository to do CRUD operations on users
#[cfg_attr(any(test, feature = "mocks"), faux::create)]
#[derive(Debug)]
pub struct UserRepository {
    pool: PgPool,
}

#[cfg_attr(any(test, feature = "mocks"), faux::methods)]
impl UserRepository {
    /// Create a new `UserRepository`
    #[must_use]
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[cfg_attr(any(test, feature = "mocks"), faux::methods)]
#[async_trait::async_trait]
impl Repository for UserRepository {
    type Entity = User;
    type Key = Uuid;
    type Error = ();

    async fn create(
        &self,
        new: <UserRepository as Repository>::Entity,
    ) -> Result<<UserRepository as Repository>::Entity, <UserRepository as Repository>::Error> {
        sqlx::query_as!(
            Self::Entity,
            "INSERT INTO users (username, password, email) VALUES ($1, $2, $3) RETURNING *",
            new.username,
            new.password,
            new.email,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| ())
    }

    async fn update(
        &self,
        _key: <UserRepository as Repository>::Key,
        _entity: <UserRepository as Repository>::Entity,
    ) -> Result<<UserRepository as Repository>::Entity, <UserRepository as Repository>::Error> {
        Err(())
    }

    async fn delete(
        &self,
        _key: <UserRepository as Repository>::Key,
    ) -> Result<(), <UserRepository as Repository>::Error> {
        Ok(())
    }

    async fn find_by_id(
        &self,
        key: <UserRepository as Repository>::Key,
    ) -> Option<<UserRepository as Repository>::Entity> {
        sqlx::query_as!(Self::Entity, "SELECT * FROM users WHERE id = $1", key)
            .fetch_one(&self.pool)
            .await
            .ok()
    }
}
