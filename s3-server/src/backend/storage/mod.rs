pub mod storage;
use aws_sdk_s3::primitives::ByteStream;
use axum::async_trait;
use s3_core::S3Error;

#[async_trait]
pub trait FileStorage: Send + Sync {
    async fn get_file(&self, bucket: &str, key: &str) -> Result<ByteStream, S3Error>;
    async fn get_file_range(
        &self,
        bucket: &str,
        key: &str,
        start: u64,
        end: u64,
    ) -> Result<ByteStream, S3Error>;
    async fn save_file(&self, bucket: &str, key: &str, data: ByteStream) -> Result<(), S3Error>;
    async fn delete_file(&self, bucket: &str, key: &str) -> Result<(), S3Error>;
}
