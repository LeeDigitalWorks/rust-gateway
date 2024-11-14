use aws_sdk_s3::primitives::{ByteStream, SdkBody};
use md5::Digest;
use s3_core::{
    response::{ListBucketsResponse, ResponseData},
    types::{BucketContainer, Owner},
    S3Error,
};
use uuid::{timestamp::context, Timestamp, Uuid};

use crate::{
    backend::{
        types::{self, Bucket},
        FileStorage, Indexer,
    },
    filter::S3Data,
};

pub struct FullstackBackend {
    database: Box<dyn Indexer>,
    storage: Box<dyn FileStorage>,
}

impl FullstackBackend {
    pub fn new(database: Box<dyn Indexer>, storage: Box<dyn FileStorage>) -> Self {
        Self { database, storage }
    }
}

impl FullstackBackend {
    pub async fn create_bucket(&self, data: &mut S3Data) -> Result<ResponseData, S3Error> {
        // Check if bucket already exists
        let bucket = data.bucket.as_ref();
        if bucket.is_some() {
            if let Some(bucket) = bucket {
                if bucket.user_id == data.auth_key.user_id {
                    return Err(S3Error::BucketAlreadyOwnedByYou(data.bucket_name.clone()));
                }
            }
            return Err(S3Error::BucketAlreadyExists(data.bucket_name.clone()));
        }
        self.database
            .create_bucket(&Bucket {
                id: Uuid::now_v7(),
                name: data.bucket_name.clone(),
                user_id: data.auth_key.user_id,
                created_at: chrono::Utc::now(),
            })
            .await
            .map_err(|e| {
                tracing::error!("Error creating bucket: {:?}", e);
                S3Error::InternalError
            })?;

        data.res
            .with_header("Location".to_string(), format!("/{}", data.bucket_name));
        data.res.with_status_code(200);

        Ok(data.res.clone())
    }

    pub async fn delete_bucket(&self, data: &mut S3Data) -> Result<ResponseData, S3Error> {
        let bucket = data
            .bucket
            .as_ref()
            .ok_or(S3Error::NoSuchBucket(data.bucket_name.clone()))?;
        let user_id = &data.auth_key.user_id;
        // Check if bucket is not empty
        // TODO: Only check for at most one object in the bucket
        let objects = self
            .database
            .list_objects(bucket.id)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => S3Error::NoSuchBucket(bucket.name.clone()),
                _ => {
                    tracing::error!("Error listing objects: {:?}", e);
                    S3Error::InternalError
                }
            })?;
        if !objects.is_empty() {
            return Err(S3Error::BucketNotEmpty);
        }
        // Delete bucket from database backend
        self.database
            .delete_bucket(&bucket, user_id)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => S3Error::NoSuchBucket(bucket.name.clone()),
                _ => {
                    tracing::error!("Error deleting bucket: {:?}", e);
                    S3Error::InternalError
                }
            })?;

        data.res.with_status_code(204);
        Ok(data.res.clone())
    }

    pub async fn put_object(&self, data: &mut S3Data) -> Result<ResponseData, S3Error> {
        let bucket = data
            .bucket
            .as_ref()
            .ok_or(S3Error::NoSuchBucket(data.bucket_name.clone()))?;

        let content_length: i64 = data
            .req
            .headers()
            .get("Content-Length")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok())
            .unwrap_or_default();

        let expect = data
            .req
            .headers()
            .get("Expect")
            .and_then(|v| v.to_str().ok())
            .unwrap_or_default();
        if expect == "100-continue" {
            data.res.with_status_code(100);
            return Ok(data.res.clone());
        }

        // Compute ETag for object
        let bytes = data.req.body();
        let mut hasher = md5::Md5::new();
        hasher.update(bytes);
        let etag = const_hex::encode(hasher.finalize().to_vec());
        let body = ByteStream::new(bytes.to_owned().into());

        // Validate Content-Length
        if content_length > s3_core::MAX_OBJECT_PART_SIZE as i64 {
            return Err(S3Error::EntityTooLarge);
        }
        if content_length < 0 {
            return Err(S3Error::MissingContentLength);
        }

        let object = types::Object {
            bucket_id: bucket.id,
            key: data.key.clone(),
            owner_id: data.auth_key.user_id,
            version_id: Uuid::now_v7(),
            is_latest: true,
            size: content_length,
            etag: etag.clone(),
            ..Default::default()
        };

        // Insert into database backend
        self.database
            .put_object(&bucket, &object)
            .await
            .map_err(|e| {
                tracing::error!("Error putting object: {:?}", e);
                S3Error::InternalError
            })?;
        // Insert into storage backend
        // self.storage
        //     .save_file(&bucket.name, &object.key, body)
        //     .await?;

        data.res
            .with_status_code(200)
            .with_header("ETag".to_string(), etag);
        Ok(data.res.clone())
    }

    pub async fn delete_object(&self, data: &mut S3Data) -> Result<ResponseData, S3Error> {
        Err(S3Error::NotImplemented)
    }

    pub async fn delete_objects(
        &self,
        bucket_name: &str,
        keys: Vec<String>,
    ) -> Result<(), S3Error> {
        Err(S3Error::NotImplemented)
    }

    pub async fn get_bucket(&self, bucket_name: &str) -> Result<types::Bucket, S3Error> {
        let bucket = self
            .database
            .get_bucket(bucket_name)
            .await
            .map_err(|_| S3Error::NoSuchBucket(bucket_name.to_string()))?;
        Ok(bucket)
    }

    pub async fn list_buckets(&self, data: &mut S3Data) -> Result<ListBucketsResponse, S3Error> {
        let user_id = &data.auth_key.user_id;
        let buckets = self
            .database
            .list_buckets(user_id)
            .await
            .map_err(|_| S3Error::InternalError)?;
        Ok(ListBucketsResponse {
            buckets: BucketContainer {
                buckets: buckets
                    .iter()
                    .map(|bucket| s3_core::Bucket {
                        name: bucket.name.clone(),
                        creation_date: bucket.created_at.to_string(),
                    })
                    .collect(),
            },
            owner: Owner {
                id: user_id.to_string(),
                display_name: user_id.to_string(),
            },
        })
    }

    pub async fn get_object(&self, _data: &mut S3Data) -> Result<ResponseData, S3Error> {
        Err(S3Error::NotImplemented)
    }

    pub async fn list_objects(&self, _data: &mut S3Data) -> Result<ResponseData, S3Error> {
        Err(S3Error::NotImplemented)
    }

    pub async fn list_object_versions(&self, _data: &mut S3Data) -> Result<ResponseData, S3Error> {
        Err(S3Error::NotImplemented)
    }

    pub async fn list_parts(&self, _data: &mut S3Data) -> Result<ResponseData, S3Error> {
        Err(S3Error::NotImplemented)
    }
}
