use axum::async_trait;

use crate::domain::entities::todo::Todo;

use super::error::ServiceResult;

#[derive(Debug)]
pub struct CreateInput {
    pub title: String,
    pub description: String,
}

#[derive(Debug)]
pub struct UpdateInput {
    pub title: Option<String>,
    pub description: Option<String>,
}

#[async_trait]
pub trait TodoServicePort: Sync + Send {
    async fn list(&self) -> ServiceResult<Vec<Todo>>;
    async fn get(&self, todo_id: String) -> ServiceResult<Todo>;
    async fn update(&self, id: String, update: UpdateInput) -> ServiceResult<Todo>;
    async fn create(&self, input: CreateInput) -> ServiceResult<Todo>;
}
