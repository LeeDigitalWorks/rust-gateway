use async_trait::async_trait;
use s3_core::error::S3Error;
use s3_core::response::ListBucketsResponse;

pub mod memory;
pub mod proxy;

#[async_trait]
pub trait Backend: Send + Sync {
    async fn list_buckets(&self) -> Result<ListBucketsResponse, S3Error>;
    async fn create_bucket(&self, name: &str) -> Result<(), S3Error>;
    async fn delete_bucket(&self, name: &str) -> Result<(), S3Error>;
    async fn get_object(&self, bucket: &str, key: &str) -> Result<Vec<u8>, S3Error>;
    async fn put_object(&self, bucket: &str, key: &str, data: Vec<u8>) -> Result<(), S3Error>;
}

pub use memory::InMemoryBackend;
