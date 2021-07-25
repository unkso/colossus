/// The Repository trait represents common CRUD operations that can be leveraged by repositories
#[async_trait::async_trait]
pub trait Repository: Send + Sync {
    /// The underlying model type
    type Entity: Send + Sync;
    /// The type that acts as the primary key
    type Key: Send + Sync;
    /// The error type
    type Error: Send + Sync;

    /// Create a new entity and persist it
    async fn create(&self, new: Self::Entity) -> Result<Self::Entity, Self::Error>;

    /// Update an existing entity
    async fn update(
        &self,
        key: Self::Key,
        entity: Self::Entity,
    ) -> Result<Self::Entity, Self::Error>;

    /// Delete an entity
    async fn delete(&self, key: Self::Key) -> Result<(), Self::Error>;

    /// Find an entity with the specified key
    async fn find_by_id(&self, key: Self::Key) -> Option<Self::Entity>;
}
