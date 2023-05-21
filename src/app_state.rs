use std::sync::Arc;

use crate::{
    domain::services::{todo_service::TodoServicePort, user_service::UserServicePort},
    error::ServiceStartupError,
    infrastructure::{
        repositories::{todo_repository::TodoRepository, user_repository::UserRepository},
        Database,
    },
    services::{todo_service::TodoService, user_service::UserService},
};

#[derive(Clone)]
pub struct AppState {
    pub todo_service: Arc<dyn TodoServicePort>,
    pub user_service: Arc<dyn UserServicePort>,
}

impl AppState {
    pub async fn new(connection_string: &str) -> Result<Self, ServiceStartupError> {
        let database = Database::new(connection_string).await?;

        // Repositories
        let todo_repository = Arc::new(TodoRepository::new(database.clone()));
        let user_repository = Arc::new(UserRepository::new(database));

        // Services
        let todo_service = Arc::new(TodoService::new(todo_repository));
        let user_service = Arc::new(UserService::new(user_repository));

        Ok(Self {
            todo_service,
            user_service,
        })
    }
}
