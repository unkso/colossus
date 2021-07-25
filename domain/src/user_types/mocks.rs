use crate::repositories::Repository;
use crate::user_types::User;
use mockall::mock;
use sqlx::types::Uuid;

mock! {
    pub User {}

    #[async_trait::async_trait]
    impl Repository for User {
        type Entity = User;
        type Key = Uuid;
        type Error = ();

        async fn create(&self, new: User) -> Result<<Self as Repository>::Entity, <Self as Repository>::Error>;

        async fn update(&self, key: Uuid, entity: User) -> Result<<Self as Repository>::Entity, <Self as Repository>::Error>;

        async fn delete(&self, key: Uuid) -> Result<(), <Self as Repository>::Error>;

        async fn find_by_id(&self, key: Uuid) -> Option<<Self as Repository>::Entity>;
    }
}
