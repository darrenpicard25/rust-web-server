use axum::async_trait;

use crate::domain::entities::todo::Todo;

pub struct UpdateInput {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
}

pub struct CreateInput {
    pub title: String,
    pub description: String,
}

#[async_trait]
pub trait TodoRepositoryPort: Send + Sync {
    async fn list(&self) -> Vec<Todo>;
    async fn find_by_id(&self, id: String) -> Todo;
    async fn update_one(&self, input: UpdateInput) -> Todo;
    async fn create(&self, input: CreateInput) -> Todo;
}
