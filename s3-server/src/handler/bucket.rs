use std::sync::Arc;

use axum::response::IntoResponse;
use tokio::sync::RwLock;

use crate::{
    backend::{IndexReader, IndexWriter},
    filter::S3Data,
    server::{AppState, Server},
};

impl Server {
    pub async fn list_buckets(
        state: &Arc<AppState>,
        data: &mut S3Data,
    ) -> axum::response::Response {
        let response = state.backend.list_buckets(&data.auth_key.user_id).await;
        axum::response::IntoResponse::into_response(response)
    }

    pub async fn create_bucket(
        state: &Arc<AppState>,
        data: &mut S3Data,
    ) -> axum::response::Response {
        let response = state
            .backend
            .create_bucket(&data.bucket_name, &data.auth_key.user_id)
            .await;
        axum::response::IntoResponse::into_response(response)
    }

    pub async fn delete_bucket(
        state: &Arc<AppState>,
        data: &mut S3Data,
    ) -> axum::response::Response {
        let bucket = data
            .bucket
            .as_ref()
            .ok_or(s3_core::S3Error::NoSuchBucket(data.bucket_name.clone()));
        if let Err(e) = bucket {
            return e.into_response();
        }
        let bucket = bucket.unwrap();
        let response = state
            .backend
            .delete_bucket(bucket, &data.auth_key.user_id)
            .await;
        axum::response::IntoResponse::into_response(response)
    }
}
