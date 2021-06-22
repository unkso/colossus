use std::sync::Arc;

use sqlx::{query_as, types::Uuid, Error, FromRow, PgPool};

#[cfg_attr(test, mockall::automock(type Entity=User; type ID=Uuid; type Source=Arc<PgPool>;))]
#[async_trait::async_trait]
pub trait Repository {
    type Entity;
    type ID;
    type Source;
    async fn get_all(&self) -> Result<Vec<Self::Entity>, Error>;
    async fn get_by_id(&self, id: Self::ID) -> Result<Self::Entity, Error>;
}

pub struct UserRepo {
    pool: Arc<PgPool>,
}

#[async_trait::async_trait]
impl Repository for UserRepo {
    type Entity = User;
    type ID = Uuid;
    type Source = Arc<PgPool>;

    async fn get_all(&self) -> Result<Vec<Self::Entity>, Error> {
        query_as!(Self::Entity, "SELECT * FROM users")
            .fetch_all(&*self.pool)
            .await
    }

    async fn get_by_id(&self, id: Self::ID) -> Result<Self::Entity, Error> {
        query_as!(Self::Entity, "SELECT * FROM users WHERE id = $1", id)
            .fetch_one(&*self.pool)
            .await
    }
}

async fn thing_with_users(repo: &impl Repository) -> bool {
    let result = repo.get_all().await.unwrap();

    true
}

#[derive(FromRow)]
pub struct User {
    pub id: Uuid,
    username: String,
    password: String,
    email: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_user_repo() {
        let mut mock_repo = MockRepository::new();
        mock_repo.expect_get_all().times(1).returning(|| {
            Ok(vec![User {
                id: Uuid::default(),
                username: "test".to_string(),
                password: "test".to_string(),
                email: "john@example.com".to_string(),
            }])
        });

        assert!(thing_with_users(&mock_repo).await);
    }
}
