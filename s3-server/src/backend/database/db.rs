use sqlx::Row;

use crate::backend::types;

pub struct Database {
    pool: sqlx::PgPool,
}

impl Database {
    pub fn new(pool: sqlx::PgPool) -> Result<Self, sqlx::Error> {
        Ok(Self { pool })
    }

    pub async fn create_bucket(&self, bucket: types::Bucket) -> Result<(), sqlx::Error> {
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

    pub async fn list_buckets(&self, user_id: &i64) -> Result<Vec<types::Bucket>, sqlx::Error> {
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
}
