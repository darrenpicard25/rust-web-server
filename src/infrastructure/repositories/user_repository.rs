use std::str::FromStr;

use axum::async_trait;
use sqlx::{
    types::{
        time::{OffsetDateTime, PrimitiveDateTime},
        Uuid,
    },
    Error, FromRow,
};

use crate::{
    domain::{
        entities::user::User,
        repositories::{
            error::{RepositoryError, RepositoryResult},
            user_repository::{CreateInput, UpdateInput, UserRepositoryPort},
        },
    },
    infrastructure::Database,
};

#[derive(FromRow, Debug)]
struct UserDocument {
    id: Uuid,
    email: String,
    first_name: String,
    #[allow(dead_code)]
    created_at: PrimitiveDateTime,
    #[allow(dead_code)]
    updated_at: PrimitiveDateTime,
}

impl Into<User> for UserDocument {
    fn into(self) -> User {
        User {
            id: self.id.to_string(),
            email: self.email,
            first_name: self.first_name,
        }
    }
}

pub struct UserRepository {
    db: Database,
}

impl UserRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepositoryPort for UserRepository {
    async fn find_by_id(&self, id: String) -> RepositoryResult<User> {
        tracing::debug!("UserRepository.find_by_id | {id}");

        let document = sqlx::query_as::<_, UserDocument>("SELECT * FROM users WHERE id = $1")
            .bind(Uuid::from_str(&id).map_err(|_| RepositoryError::InvalidUuid)?)
            .fetch_one(&self.db.pool())
            .await
            .map_err(|e| {
                tracing::error!("{e}");
                match e {
                    Error::RowNotFound => RepositoryError::NotFound,
                    _ => RepositoryError::Unknown,
                }
            })?;

        Ok(document.into())
    }

    async fn find_by_email(&self, email: String) -> RepositoryResult<Option<User>> {
        tracing::debug!("UserRepository.find_by_email | {email}");

        let document = sqlx::query_as::<_, UserDocument>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_one(&self.db.pool())
            .await;

        match document {
            Ok(doc) => Ok(Some(doc.into())),
            Err(Error::RowNotFound) => Ok(None),
            Err(e) => {
                tracing::error!("{e}");
                Err(RepositoryError::Unknown)
            }
        }
    }

    async fn update_one(&self, id: String, input: UpdateInput) -> RepositoryResult<User> {
        tracing::debug!("UserRepository.update_one | {id} | {input:?}");

        let user = self.find_by_id(id).await?;
        let now = OffsetDateTime::now_utc();
        let now = PrimitiveDateTime::new(now.date(), now.time());

        let document = sqlx::query_as::<_, UserDocument>(
            r#"UPDATE users
            SET
            email = $1,
            first_name = $2,
            updated_at = $3
            WHERE id = $4
            RETURNING *"#,
        )
        .bind(input.email)
        .bind(input.first_name)
        .bind(now)
        .bind(Uuid::from_str(&user.id).map_err(|_| RepositoryError::InvalidUuid)?)
        .fetch_one(&self.db.pool())
        .await
        .map_err(|e| {
            tracing::error!("{e}");
            match e {
                Error::RowNotFound => RepositoryError::NotFound,
                _ => RepositoryError::Unknown,
            }
        })?;

        Ok(document.into())
    }

    async fn create(&self, input: CreateInput) -> RepositoryResult<User> {
        tracing::debug!("UserRepository.create | {input:?}");

        let id = Uuid::new_v4();
        let now = OffsetDateTime::now_utc();
        let now = PrimitiveDateTime::new(now.date(), now.time());

        let document = sqlx::query_as::<_, UserDocument>(
            r#"INSERT INTO users
            (id, email, first_name, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *"#,
        )
        .bind(id)
        .bind(input.email)
        .bind(input.first_name)
        .bind(now)
        .bind(now)
        .fetch_one(&self.db.pool())
        .await
        .map_err(|e| {
            tracing::error!("User Repository Error: {e}");
            match e {
                _ => RepositoryError::Unknown,
            }
        })?;

        Ok(document.into())
    }
}
