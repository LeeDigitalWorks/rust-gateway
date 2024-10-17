use std::sync::Arc;

use axum::{extract::State, http::HeaderMap, response::IntoResponse};
use s3_core::{response::ListBucketsResponse, S3Error};

use crate::server::AppState;

pub async fn list_buckets(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    tracing::debug!(headers = ?headers, "Listing buckets");
    let response = state.backend.list_buckets().await;
    match response {
        Ok(response) => axum::response::IntoResponse::into_response(response.into_response()),
        Err(error) => axum::response::IntoResponse::into_response(error),
    }
}

pub async fn create_bucket() -> &'static str {
    "Bucket created"
}

pub async fn delete_bucket() -> &'static str {
    "Bucket deleted"
}
