use axum::async_trait;

use crate::domain::entities::user::User;

use super::error::RepositoryResult;

#[derive(Debug)]
pub struct UpdateInput {
    pub email: String,
    pub first_name: String,
}

#[derive(Debug)]
pub struct CreateInput {
    pub email: String,
    pub first_name: String,
}

#[async_trait]
pub trait UserRepositoryPort: Send + Sync {
    async fn find_by_id(&self, id: String) -> RepositoryResult<User>;
    async fn find_by_email(&self, email: String) -> RepositoryResult<Option<User>>;
    async fn update_one(&self, id: String, input: UpdateInput) -> RepositoryResult<User>;
    async fn create(&self, input: CreateInput) -> RepositoryResult<User>;
}
