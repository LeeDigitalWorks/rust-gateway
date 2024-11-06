use std::sync::Arc;

use tokio::sync::RwLock;

use crate::server::{AppState, Server};

impl Server {
    pub async fn list_buckets(state: &Arc<RwLock<AppState>>) -> axum::response::Response {
        let response = state.read().await.backend.list_buckets().await;
        match response {
            Ok(response) => {
                return axum::response::IntoResponse::into_response(response.into_response());
            }
            Err(error) => {
                return axum::response::IntoResponse::into_response(error);
            }
        }
    }

    pub async fn create_bucket(
        state: &Arc<RwLock<AppState>>,
        bucket_name: String,
    ) -> axum::response::Response {
        let response = state.read().await.backend.create_bucket(&bucket_name).await;
        match response {
            Ok(_) => {
                return axum::response::IntoResponse::into_response(());
            }
            Err(error) => {
                return axum::response::IntoResponse::into_response(error);
            }
        }
    }

    pub async fn delete_bucket(
        state: &Arc<RwLock<AppState>>,
        bucket_name: String,
    ) -> axum::response::Response {
        let response = state.read().await.backend.delete_bucket(&bucket_name).await;
        match response {
            Ok(_) => {
                return axum::response::IntoResponse::into_response(());
            }
            Err(error) => {
                return axum::response::IntoResponse::into_response(error);
            }
        }
    }
}
