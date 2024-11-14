use aws_sdk_s3::primitives::ByteStream;
use axum::async_trait;
use s3_core::S3Error;

use super::FileStorage;

pub struct StorageBackend {
    s3_client: aws_sdk_s3::Client,
}

impl StorageBackend {
    pub fn new(s3_client: aws_sdk_s3::Client) -> Self {
        Self { s3_client }
    }
}

#[async_trait]
impl FileStorage for StorageBackend {
    async fn get_file(&self, bucket: &str, key: &str) -> Result<ByteStream, S3Error> {
        let response = self
            .s3_client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .map_err(|_| S3Error::NoSuchKey(key.to_string()))?;

        Ok(response.body)
    }

    async fn get_file_range(
        &self,
        bucket: &str,
        key: &str,
        start: u64,
        end: u64,
    ) -> Result<ByteStream, S3Error> {
        let response = self
            .s3_client
            .get_object()
            .bucket(bucket)
            .key(key)
            .range(format!("bytes={}-{}", start, end))
            .send()
            .await
            .map_err(|_| S3Error::NoSuchKey(key.to_string()))?;

        Ok(response.body)
    }

    async fn save_file(&self, bucket: &str, key: &str, data: ByteStream) -> Result<(), S3Error> {
        let response = self
            .s3_client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(data)
            .send()
            .await
            .map_err(|_| S3Error::InternalError)?;

        Ok(())
    }

    async fn delete_file(&self, bucket: &str, key: &str) -> Result<(), S3Error> {
        let response = self
            .s3_client
            .delete_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .map_err(|_| S3Error::InternalError)?;

        Ok(())
    }
}
