use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{
    filter::S3Data,
    server::{AppState, Server},
};

impl Server {
    pub async fn list_buckets(
        state: &Arc<RwLock<AppState>>,
        data: S3Data,
    ) -> axum::response::Response {
        let response = state
            .read()
            .await
            .backend
            .list_buckets(&data.auth_key.user_id)
            .await;
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
        data: S3Data,
    ) -> axum::response::Response {
        let response = state
            .read()
            .await
            .backend
            .create_bucket(&data.bucket_name, &data.auth_key.user_id)
            .await;
        match response {
            Ok(_) => {
                let mut res = axum::response::IntoResponse::into_response(());
                res.headers_mut().insert(
                    "Location",
                    format!("/{}", data.bucket_name).parse().unwrap(),
                );
                res
            }
            Err(error) => {
                return axum::response::IntoResponse::into_response(error);
            }
        }
    }

    pub async fn delete_bucket(
        state: &Arc<RwLock<AppState>>,
        data: S3Data,
    ) -> axum::response::Response {
        let response = state
            .read()
            .await
            .backend
            .delete_bucket(&data.bucket_name, &data.auth_key.user_id)
            .await;
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
