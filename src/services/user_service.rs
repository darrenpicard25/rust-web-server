use std::sync::Arc;

use axum::async_trait;

use crate::domain::{
    entities::user::User,
    repositories::user_repository::{
        CreateInput as RepositoryCreateInput, UpdateInput as RepositoryUpdateInput,
        UserRepositoryPort,
    },
    services::{
        error::{ServiceError, ServiceResult},
        user_service::{CreateInput, UpdateInput, UserServicePort},
    },
};

pub struct UserService {
    user_repository: Arc<dyn UserRepositoryPort>,
}

impl UserService {
    pub fn new(user_repository: Arc<dyn UserRepositoryPort>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl UserServicePort for UserService {
    async fn get(&self, id: String) -> ServiceResult<User> {
        tracing::debug!("UserService.get | {id}");

        let user = self.user_repository.find_by_id(id).await?;

        Ok(user)
    }

    async fn create(&self, input: CreateInput) -> ServiceResult<User> {
        tracing::debug!("UserService.create | {input:?}");

        if self.is_email_already_in_use(input.email.clone()).await? {
            tracing::warn!("Email already claimed in system by other user");
            return Err(ServiceError::BadInput);
        }

        let input = RepositoryCreateInput {
            email: input.email,
            first_name: input.first_name,
        };

        let user = self.user_repository.create(input).await?;

        Ok(user)
    }

    async fn update(&self, id: String, update: UpdateInput) -> ServiceResult<User> {
        tracing::debug!("UserService.update | {id} | {update:?}");

        let user = self.user_repository.find_by_id(id).await?;

        if update.email.is_none() && update.first_name.is_none() {
            tracing::warn!("No new information passed into update. Returning early");
            return Ok(user);
        }

        if let Some(email) = update.email.clone() {
            if self.is_email_already_in_use(email).await? {
                tracing::warn!("Email already claimed in system by other user");
                return Err(ServiceError::BadInput);
            }
        }
        let input = RepositoryUpdateInput {
            email: update.email.unwrap_or(user.email),
            first_name: update.first_name.unwrap_or(user.first_name),
        };

        let user = self.user_repository.update_one(user.id, input).await?;

        Ok(user)
    }
}

impl UserService {
    async fn is_email_already_in_use(&self, email: String) -> ServiceResult<bool> {
        tracing::debug!("UserService.is_email_already_in_use | {email}");

        let existing_user = self.user_repository.find_by_email(email).await?;

        match existing_user {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}
