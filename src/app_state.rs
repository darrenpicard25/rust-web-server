use std::sync::Arc;

use crate::{
    domain::services::todo_service::TodoServicePort,
    error::ServiceStartupError,
    infrastructure::{repositories::todo_repository::TodoRepository, Database},
    services::todo_service::TodoService,
};

#[derive(Clone)]
pub struct AppState {
    pub todo_service: Arc<dyn TodoServicePort>,
}

impl AppState {
    pub async fn new(connection_string: &str) -> Result<Self, ServiceStartupError> {
        let database = Database::new(connection_string).await?;

        // Repositories
        let todo_repository = Arc::new(TodoRepository::new(database));

        // Services
        let todo_service = Arc::new(TodoService::new(todo_repository));

        Ok(Self { todo_service })
    }
}
