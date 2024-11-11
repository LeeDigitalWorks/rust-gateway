use axum::async_trait;
use s3_core::error::S3Error;
use s3_core::response::{ListBucketsResponse, ResponseData};

#[async_trait]
pub trait Indexer: IndexReader + IndexWriter {}

#[async_trait]
pub trait IndexReader: Send + Sync {
    async fn get_bucket(&self, bucket_name: &str) -> Result<types::Bucket, S3Error>;
    async fn list_buckets(&self, user_id: &i64) -> Result<ListBucketsResponse, S3Error>;
    async fn get_object(&self, bucket: &str, key: &str) -> Result<types::Object, S3Error>;
    async fn list_objects(&self, bucket: &str) -> Result<Vec<String>, S3Error>;
    async fn list_object_versions(&self, bucket: &str, key: &str) -> Result<(), S3Error>;
    async fn list_parts(&self, bucket: &str, key: &str, upload_id: &str) -> Result<(), S3Error>;
}

#[async_trait]
pub trait IndexWriter: Send + Sync {
    async fn create_bucket(
        &self,
        bucket_name: &str,
        user_id: &i64,
    ) -> Result<ResponseData, S3Error>;
    async fn delete_bucket(&self, bucket: &types::Bucket, user_id: &i64) -> Result<(), S3Error>;
    async fn put_object(
        &self,
        bucket: &types::Bucket,
        object: &types::Object,
    ) -> Result<(), S3Error>;
    async fn delete_object(&self, bucket: &str, key: &str) -> Result<(), S3Error>;
    async fn delete_objects(&self, bucket: &str, keys: Vec<String>) -> Result<(), S3Error>;
}

pub mod storage;
pub use storage::FileStorage;
pub mod database;
pub use database::*;
pub mod fullstack;
pub use fullstack::FullstackBackend;

pub mod types;
