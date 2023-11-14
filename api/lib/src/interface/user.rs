use crate::prelude::*;
use shared::prelude::*;
use uuid::Uuid;

pub type Error = WiseError;
pub type FamilyResult<T> = Result<T, Error>;

#[async_trait::async_trait]
pub trait FamilyUser: Send + Sync + 'static {
    async fn get(&self, id: Uuid) -> FamilyResult<User>;
    async fn get_all(&self) -> FamilyResult<Vec<User>>;
    async fn create(&self, user: &User) -> FamilyResult<User>;
    async fn update(&self, user: &User) -> FamilyResult<User>;
    async fn delete(&self, user: &User) -> FamilyResult<()>;
}
