use axum::async_trait;
use s3_core::error::S3Error;
use s3_core::response::ListBucketsResponse;

#[async_trait]
pub trait Indexer: Send + Sync {
    async fn list_buckets(&self, user_id: &u64) -> Result<ListBucketsResponse, S3Error>;
    async fn create_bucket(&self, bucket_name: &str, user_id: &u64) -> Result<(), S3Error>;
    async fn delete_bucket(&self, bucket_name: &str, user_id: &u64) -> Result<(), S3Error>;
    async fn get_object(&self, bucket: &str, key: &str) -> Result<Vec<u8>, S3Error>;
    async fn put_object(&self, bucket: &str, key: &str, data: Vec<u8>) -> Result<(), S3Error>;
}

pub mod memory;
pub use memory::InMemoryBackend;
pub mod proxy;

pub mod types;