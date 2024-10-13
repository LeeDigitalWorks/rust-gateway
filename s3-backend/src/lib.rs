use async_trait::async_trait;
use s3_core::error::S3Error;
use s3_core::response::ListBucketsResponse;

pub mod memory;

#[async_trait]
pub trait Backend: Send + Sync {
    async fn list_buckets(&self) -> Result<ListBucketsResponse, S3Error>;
    async fn create_bucket(&self, name: &str) -> Result<(), S3Error>;
    async fn delete_bucket(&self, name: &str) -> Result<(), S3Error>;
}

pub use memory::InMemoryBackend;
