use std::sync::Arc;

use axum::async_trait;

use crate::domain::{
    entities::todo::Todo,
    repositories::todo_repository::{
        CreateInput as RepositoryCreateInput, TodoRepositoryPort,
        UpdateInput as RepositoryUpdateInput,
    },
    services::{
        error::ServiceResult,
        todo_service::{CreateInput, TodoServicePort, UpdateInput},
    },
};

pub struct TodoService {
    todo_repository: Arc<dyn TodoRepositoryPort>,
}

impl TodoService {
    pub fn new(todo_repository: Arc<dyn TodoRepositoryPort>) -> Self {
        Self { todo_repository }
    }
}

#[async_trait]
impl TodoServicePort for TodoService {
    async fn list(&self) -> ServiceResult<Vec<Todo>> {
        let todos = self.todo_repository.list().await?;

        Ok(todos)
    }

    async fn get(&self, todo_id: String) -> ServiceResult<Todo> {
        let todo = self.todo_repository.find_by_id(todo_id).await?;

        Ok(todo)
    }

    async fn create(&self, input: CreateInput) -> ServiceResult<Todo> {
        let todo = self
            .todo_repository
            .create(RepositoryCreateInput {
                title: input.title,
                description: input.description,
            })
            .await?;

        Ok(todo)
    }

    async fn update(&self, id: String, update: UpdateInput) -> ServiceResult<Todo> {
        if update.title.is_none() && update.description.is_none() {
            let todo = self.todo_repository.find_by_id(id).await?;

            return Ok(todo);
        }

        let todo = self
            .todo_repository
            .update_one(RepositoryUpdateInput {
                id,
                title: update.title,
                description: update.description,
            })
            .await?;

        Ok(todo)
    }
}
