use axum::async_trait;

use crate::domain::entities::todo::Todo;

pub struct CreateInput {
    pub title: String,
    pub description: String,
}

pub struct UpdateInput {
    pub title: Option<String>,
    pub description: Option<String>,
}

#[async_trait]
pub trait TodoServicePort: Sync + Send {
    async fn list(&self) -> Vec<Todo>;
    async fn get(&self, todo_id: String) -> Todo;
    async fn update(&self, id: String, update: UpdateInput) -> Todo;
    async fn create(&self, input: CreateInput) -> Todo;
}
