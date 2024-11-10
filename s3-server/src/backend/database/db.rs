use axum::async_trait;

use crate::backend::types;

#[async_trait]
pub trait DatabaseStore {
    async fn create_bucket(&self, bucket: types::Bucket) -> Result<(), sqlx::Error>;
    async fn list_buckets(&self, user_id: &i64) -> Result<Vec<types::Bucket>, sqlx::Error>;
    async fn delete_bucket(&self, bucket_name: &str, user_id: &i64) -> Result<(), sqlx::Error>;
    async fn put_object(&self, object: types::Object) -> Result<(), sqlx::Error>;
    async fn get_object(&self, key: &str) -> Result<types::Object, sqlx::Error>;
    async fn list_objects(&self, bucket_id: uuid::Uuid) -> Result<Vec<types::Object>, sqlx::Error>;
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

    async fn put_object(&self, object: types::Object) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO objects (bucket_id, key, size)
            VALUES ($1, $2, $3)
            "#,
            object.bucket_id,
            object.key,
            object.size,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_object(&self, key: &str) -> Result<types::Object, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT bucket_id, key, size
            FROM objects
            WHERE key = $1
            "#,
            key
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(types::Object {
            bucket_id: result.bucket_id,
            key: result.key,
            size: result.size,
            ..Default::default()
        })
    }

    async fn list_objects(&self, bucket_id: uuid::Uuid) -> Result<Vec<types::Object>, sqlx::Error> {
        let results = sqlx::query!(
            r#"
            SELECT bucket_id, key, size
            FROM objects
            WHERE bucket_id = $1
            "#,
            bucket_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut objects = Vec::new();

        for result in results {
            objects.push(types::Object {
                bucket_id: result.bucket_id,
                key: result.key,
                size: result.size,
                ..Default::default()
            });
        }

        Ok(objects)
    }
}
