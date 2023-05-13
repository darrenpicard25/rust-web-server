use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::error::ServiceStartupError;

pub mod repositories;

pub struct Database {
    pool: Pool<Postgres>,
}

// Constructor
impl Database {
    pub async fn new(connection_string: &str) -> Result<Self, ServiceStartupError> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(connection_string)
            .await
            .map_err(|_| ServiceStartupError::DatabaseConnection)?;

        sqlx::migrate!().run(&pool).await.map_err(|e| {
            tracing::error!("{e}");
            ServiceStartupError::DatabaseMigration
        })?;

        Ok(Self { pool })
    }
}

// Methods
impl Database {
    pub fn pool(&self) -> Pool<Postgres> {
        self.pool.clone()
    }
}
