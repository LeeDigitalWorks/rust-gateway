use aws_sdk_s3::primitives::DateTime;
use aws_smithy_types_convert::date_time::DateTimeExt;
use s3_core::S3Error;

use crate::backend::types;

pub struct StorageBackend {
    s3_client: aws_sdk_s3::Client,
}

impl StorageBackend {
    pub fn new(s3_client: aws_sdk_s3::Client) -> Self {
        Self { s3_client }
    }
}

impl StorageBackend {
    async fn get_object(&self, _bucket_name: &str, _key: &str) -> Result<Vec<u8>, S3Error> {
        Err(S3Error::NotImplemented)
    }

    async fn list_objects(&self, _bucket_name: &str) -> Result<Vec<String>, S3Error> {
        Err(S3Error::NotImplemented)
    }

    async fn list_object_versions(&self, _bucket_name: &str, _key: &str) -> Result<(), S3Error> {
        Err(S3Error::NotImplemented)
    }

    async fn list_parts(
        &self,
        _bucket_name: &str,
        _key: &str,
        _upload_id: &str,
    ) -> Result<(), S3Error> {
        Err(S3Error::NotImplemented)
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
        _data: Vec<u8>,
    ) -> Result<(), S3Error> {
        let resp = self
            .s3_client
            .put_object()
            .bucket(bucket_name)
            .key(key)
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
