/// Contains mocks for all relevant objects
#[cfg(feature = "mocks")]
pub mod mocks;

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
#[derive(Debug)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    /// Create a new `UserRepository`
    #[must_use]
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Repository for UserRepository {
    type Entity = User;
    type Key = Uuid;
    type Error = ();

    async fn create(&self, new: Self::Entity) -> Result<Self::Entity, Self::Error> {
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
        _key: Self::Key,
        _entity: Self::Entity,
    ) -> Result<Self::Entity, Self::Error> {
        Err(())
    }

    async fn delete(&self, _key: Self::Key) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn find_by_id(&self, key: Self::Key) -> Option<Self::Entity> {
        sqlx::query_as!(Self::Entity, "SELECT * FROM users WHERE id = $1", key)
            .fetch_one(&self.pool)
            .await
            .ok()
    }
}
