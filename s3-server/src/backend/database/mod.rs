use axum::async_trait;

use super::types;

pub mod db_reader;
pub mod db_writer;
pub mod filters;

#[async_trait]
pub trait Indexer: IndexReader + IndexWriter {}

#[async_trait]
pub trait IndexReader: Send + Sync {
    async fn get_bucket_quota(&self, user_id: &i64) -> Result<i64, sqlx::Error>;
    async fn get_bucket(&self, bucket_name: &str) -> Result<types::Bucket, sqlx::Error>;
    async fn list_buckets(&self, user_id: &i64) -> Result<Vec<types::Bucket>, sqlx::Error>;
    async fn get_object(
        &self,
        bucket_id: uuid::Uuid,
        key: &str,
    ) -> Result<types::Object, sqlx::Error>;
    async fn list_objects(&self, bucket_id: uuid::Uuid) -> Result<Vec<types::Object>, sqlx::Error>;
    async fn list_object_versions(&self, bucket: &str, key: &str) -> Result<(), sqlx::Error>;
    async fn list_parts(&self, bucket: &str, key: &str, upload_id: &str)
        -> Result<(), sqlx::Error>;
}

#[async_trait]
pub trait IndexWriter: Send + Sync {
    async fn create_bucket(&self, bucket: &types::Bucket) -> Result<(), sqlx::Error>;
    async fn delete_bucket(&self, bucket: &types::Bucket, user_id: &i64)
        -> Result<(), sqlx::Error>;
    async fn put_object(
        &self,
        bucket: &types::Bucket,
        object: &types::Object,
    ) -> Result<(), sqlx::Error>;
    async fn delete_object(&self, bucket: &str, key: &str) -> Result<(), sqlx::Error>;
    async fn delete_objects(&self, bucket: &str, keys: Vec<String>) -> Result<(), sqlx::Error>;
}

#[async_trait]
impl Indexer for Database {}

pub struct Database {
  pub pool: sqlx::PgPool,
}

impl Database {
  pub fn new(pool: sqlx::PgPool) -> Self {
      Self { pool }
  }
}
