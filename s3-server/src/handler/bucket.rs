use std::sync::Arc;

use crate::{
    filter::S3Data,
    server::{AppState, Server},
};

impl Server {
    pub async fn list_buckets(
        state: &Arc<AppState>,
        data: &mut S3Data,
    ) -> axum::response::Response {
        let response = state.fullstack.list_buckets(data).await;
        axum::response::IntoResponse::into_response(response)
    }

    pub async fn create_bucket(
        state: &Arc<AppState>,
        data: &mut S3Data,
    ) -> axum::response::Response {
        let response = state.fullstack.create_bucket(data).await;
        axum::response::IntoResponse::into_response(response)
    }

    pub async fn delete_bucket(
        state: &Arc<AppState>,
        data: &mut S3Data,
    ) -> axum::response::Response {
        let response = state.fullstack.delete_bucket(data).await;
        axum::response::IntoResponse::into_response(response)
    }
}
