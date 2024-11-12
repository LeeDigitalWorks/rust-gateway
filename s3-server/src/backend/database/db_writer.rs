use axum::async_trait;

use crate::backend::types;

use super::{Database, IndexWriter};

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
            INSERT INTO objects (bucket_id, key, size, version_id, owner_id, etag)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            bucket.id,
            object.key,
            object.size,
            object.version_id,
            object.owner_id.to_string(),
            object.etag
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
