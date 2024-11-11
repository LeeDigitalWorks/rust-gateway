use std::sync::Arc;

use axum::response::IntoResponse;

use crate::{
    backend::types,
    filter::S3Data,
    server::{AppState, Server},
};

impl Server {
    pub async fn put_object(state: &Arc<AppState>, data: &mut S3Data) -> axum::response::Response {
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
            .put_object(
                &bucket,
                &types::Object {
                    bucket_id: bucket.id,
                    key: data.key.clone(),
                    size: data
                        .req
                        .headers()
                        .get("Content-Length")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|v| v.parse().ok())
                        .unwrap_or_default(),
                    ..Default::default()
                },
                data.req.body(),
            )
            .await;
        tracing::info!("Put object response: {:?}", response);
        s3_core::S3Error::NotImplemented.into_response()
    }

    pub async fn get_object(state: &Arc<AppState>, data: &mut S3Data) -> axum::response::Response {
        let response = state.backend.get_object(&data.bucket_name, &data.key).await;
        tracing::info!("Get object response: {:?}", response);
        s3_core::S3Error::NotImplemented.into_response()
    }
}
