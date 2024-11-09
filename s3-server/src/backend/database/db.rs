use axum::async_trait;

use crate::backend::types;

#[async_trait]
pub trait DatabaseStore {
    async fn create_bucket(&self, bucket: types::Bucket) -> Result<(), sqlx::Error>;
    async fn list_buckets(&self, user_id: &i64) -> Result<Vec<types::Bucket>, sqlx::Error>;
    async fn delete_bucket(&self, bucket_name: &str, user_id: &i64) -> Result<(), sqlx::Error>;
}

pub struct Database {
    pool: sqlx::PgPool,
}

impl Database {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DatabaseStore for Database {
    async fn create_bucket(&self, bucket: types::Bucket) -> Result<(), sqlx::Error> {
        sqlx::query_as!(
            types::Bucket,
            r#"
            INSERT INTO buckets (id, name, user_id)
            VALUES ($1, $2, $3)
            "#,
            bucket.id,
            bucket.name,
            bucket.user_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn list_buckets(&self, user_id: &i64) -> Result<Vec<types::Bucket>, sqlx::Error> {
        sqlx::query_as!(
            types::Bucket,
            r#"
            SELECT id, name, user_id, created_at
            FROM buckets
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn delete_bucket(&self, bucket_name: &str, user_id: &i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM buckets
            WHERE name = $1 AND user_id = $2
            "#,
            bucket_name,
            user_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
