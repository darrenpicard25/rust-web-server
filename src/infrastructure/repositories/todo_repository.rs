use std::str::FromStr;

use axum::async_trait;
use sqlx::{
    types::{
        time::{OffsetDateTime, PrimitiveDateTime},
        Uuid,
    },
    FromRow,
};

use crate::{
    domain::{
        entities::todo::Todo,
        repositories::todo_repository::{CreateInput, TodoRepositoryPort, UpdateInput},
    },
    infrastructure::Database,
};

#[derive(FromRow, Debug)]
struct TodoDocument {
    id: Uuid,
    title: String,
    description: String,
    created_at: PrimitiveDateTime,
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
    async fn list(&self) -> Vec<Todo> {
        sqlx::query_as::<_, TodoDocument>("SELECT * FROM todos")
            .fetch_all(&self.db.pool())
            .await
            .unwrap()
            .into_iter()
            .map(|doc| doc.into())
            .collect()
    }

    async fn find_by_id(&self, id: String) -> Todo {
        sqlx::query_as::<_, TodoDocument>("SELECT * FROM todos WHERE id = $1")
            .bind(Uuid::from_str(&id).unwrap())
            .fetch_one(&self.db.pool())
            .await
            .unwrap()
            .into()
    }

    async fn update_one(&self, input: UpdateInput) -> Todo {
        let document = self.find_by_id(input.id).await;
        let now = OffsetDateTime::now_utc();
        let now = PrimitiveDateTime::new(now.date(), now.time());

        sqlx::query_as::<_, TodoDocument>(
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
        .bind(Uuid::from_str(&document.id).unwrap())
        .fetch_one(&self.db.pool())
        .await
        .unwrap()
        .into()
    }

    async fn create(&self, input: CreateInput) -> Todo {
        let id = Uuid::new_v4();
        let now = OffsetDateTime::now_utc();
        let now = PrimitiveDateTime::new(now.date(), now.time());

        sqlx::query_as::<_, TodoDocument>(
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
        .unwrap()
        .into()
    }
}
