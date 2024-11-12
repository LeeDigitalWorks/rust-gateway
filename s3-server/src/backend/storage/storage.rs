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

impl StorageBackend {
    pub async fn create_bucket(&self, bucket_name: &str, _user_id: &i64) -> Result<(), S3Error> {
        let resp = self
            .s3_client
            .create_bucket()
            .bucket(bucket_name)
            .send()
            .await;
        match resp {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("Error creating bucket: {:?}", e);
                Err(S3Error::InternalError)
            }
        }
    }

    pub async fn delete_bucket(&self, _bucket_name: &str, _user_id: &i64) -> Result<(), S3Error> {
        let resp = self
            .s3_client
            .delete_bucket()
            .bucket(_bucket_name)
            .send()
            .await;
        match resp {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("Error deleting bucket: {:?}", e);
                Err(S3Error::InternalError)
            }
        }
    }

    async fn put_object(
        &self,
        bucket_name: &str,
        key: &str,
        data: ByteStream,
    ) -> Result<(), S3Error> {
        let resp = self
            .s3_client
            .put_object()
            .bucket(bucket_name)
            .key(key)
            .body(data)
            .send()
            .await;
        match resp {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("Error putting object: {:?}", e);
                Err(S3Error::InternalError)
            }
        }
    }

    async fn delete_object(&self, _bucket_name: &str, _key: &str) -> Result<(), S3Error> {
        Err(S3Error::NotImplemented)
    }

    async fn delete_objects(&self, _bucket_name: &str, _keys: Vec<String>) -> Result<(), S3Error> {
        Err(S3Error::NotImplemented)
    }
}
