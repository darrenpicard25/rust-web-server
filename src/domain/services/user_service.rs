use axum::async_trait;

use crate::domain::entities::user::User;

use super::error::ServiceResult;

#[derive(Debug)]
pub struct CreateInput {
    pub email: String,
    pub first_name: String,
}

#[derive(Debug)]
pub struct UpdateInput {
    pub email: Option<String>,
    pub first_name: Option<String>,
}

#[async_trait]
pub trait UserServicePort: Sync + Send {
    async fn get(&self, todo_id: String) -> ServiceResult<User>;
    async fn update(&self, id: String, update: UpdateInput) -> ServiceResult<User>;
    async fn create(&self, input: CreateInput) -> ServiceResult<User>;
}
