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
        entities::todo::Todo,
        repositories::{
            error::{RepositoryError, RepositoryResult},
            todo_repository::{CreateInput, TodoRepositoryPort, UpdateInput},
        },
    },
    infrastructure::Database,
};

#[derive(FromRow, Debug)]
struct TodoDocument {
    id: Uuid,
    title: String,
    description: String,
    #[allow(dead_code)]
    created_at: PrimitiveDateTime,
    #[allow(dead_code)]
    updated_at: PrimitiveDateTime,
}

impl Into<Todo> for TodoDocument {
    fn into(self) -> Todo {
        Todo {
            id: self.id.to_string(),
            title: self.title,
            description: self.description,
        }
    }
}

pub struct TodoRepository {
    db: Database,
}

impl TodoRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

#[async_trait]
impl TodoRepositoryPort for TodoRepository {
    async fn list(&self) -> RepositoryResult<Vec<Todo>> {
        tracing::debug!("TodoRepository.list");

        let documents = sqlx::query_as::<_, TodoDocument>("SELECT * FROM todos")
            .fetch_all(&self.db.pool())
            .await
            .map_err(|_| RepositoryError::Unknown)?;

        Ok(documents.into_iter().map(|doc| doc.into()).collect())
    }

    async fn find_by_id(&self, id: String) -> RepositoryResult<Todo> {
        tracing::debug!("TodoRepository.find_by_id | {id}");

        let document = sqlx::query_as::<_, TodoDocument>("SELECT * FROM todos WHERE id = $1")
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

    async fn update_one(&self, input: UpdateInput) -> RepositoryResult<Todo> {
        tracing::debug!("TodoRepository.update_one | {input:?}");

        let document = self.find_by_id(input.id).await?;
        let now = OffsetDateTime::now_utc();
        let now = PrimitiveDateTime::new(now.date(), now.time());

        let document = sqlx::query_as::<_, TodoDocument>(
            r#"UPDATE todos
            SET
            title = $1,
            description = $2,
            updated_at = $3
            WHERE id = $4
            RETURNING *"#,
        )
        .bind(input.title.unwrap_or(document.title))
        .bind(input.description.unwrap_or(document.description))
        .bind(now)
        .bind(Uuid::from_str(&document.id).map_err(|_| RepositoryError::InvalidUuid)?)
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

    async fn create(&self, input: CreateInput) -> RepositoryResult<Todo> {
        tracing::debug!("TodoRepository.create | {input:?}");

        let id = Uuid::new_v4();
        let now = OffsetDateTime::now_utc();
        let now = PrimitiveDateTime::new(now.date(), now.time());

        let document = sqlx::query_as::<_, TodoDocument>(
            r#"INSERT INTO todos
            (id, title, description, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *"#,
        )
        .bind(id)
        .bind(input.title)
        .bind(input.description)
        .bind(now)
        .bind(now)
        .fetch_one(&self.db.pool())
        .await
        .map_err(|e| {
            tracing::error!("{e}");
            match e {
                _ => RepositoryError::Unknown,
            }
        })?;

        Ok(document.into())
    }
}
