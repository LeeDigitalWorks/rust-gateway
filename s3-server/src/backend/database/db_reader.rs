use axum::async_trait;

use crate::backend::{types, IndexReader, IndexWriter, Indexer};

pub struct Database {
    pool: sqlx::PgPool,
}

impl Database {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Indexer for Database {}

#[async_trait]
impl IndexReader for Database {
    async fn list_object_versions(&self, bucket: &str, key: &str) -> Result<(), sqlx::Error> {
        // Implement the logic to list object versions here
        Ok(())
    }

    async fn list_parts(
        &self,
        bucket: &str,
        key: &str,
        upload_id: &str,
    ) -> Result<(), sqlx::Error> {
        // Implement the logic to list parts here
        Ok(())
    }

    async fn get_bucket_quota(&self, user_id: &i64) -> Result<i64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT max_buckets
            FROM users
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(result.max_buckets.into())
    }

    async fn get_bucket(&self, bucket_name: &str) -> Result<types::Bucket, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT id, name, user_id, created_at
            FROM buckets
            WHERE name = $1
            "#,
            bucket_name
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(types::Bucket {
            id: result.id,
            name: result.name,
            user_id: result.user_id,
            created_at: result.created_at,
        })
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

    async fn get_object(
        &self,
        bucket_id: uuid::Uuid,
        key: &str,
    ) -> Result<types::Object, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT bucket_id, key, size
            FROM objects
            WHERE key = $1 and bucket_id = $2
            "#,
            key,
            bucket_id
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

#[async_trait]
impl IndexWriter for Database {
    async fn delete_object(&self, bucket: &str, key: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM objects
            WHERE bucket_id = (SELECT id FROM buckets WHERE name = $1) AND key = $2
            "#,
            bucket,
            key
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete_objects(&self, bucket: &str, keys: Vec<String>) -> Result<(), sqlx::Error> {
        for key in keys {
            self.delete_object(bucket, &key).await?;
        }
        Ok(())
    }

    async fn create_bucket(&self, bucket: &types::Bucket) -> Result<(), sqlx::Error> {
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

    async fn delete_bucket(
        &self,
        bucket: &types::Bucket,
        user_id: &i64,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM buckets
            WHERE id = $1 AND user_id = $2
            "#,
            bucket.id,
            user_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn put_object(
        &self,
        bucket: &types::Bucket,
        object: &types::Object,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO objects (bucket_id, key, size)
            VALUES ($1, $2, $3)
            "#,
            bucket.id,
            object.key,
            object.size,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
