use std::sync::Arc;

use axum::response::IntoResponse;

use crate::{
    filter::S3Data,
    server::{AppState, Server},
};

impl Server {
    pub async fn put_object(state: &Arc<AppState>, data: S3Data) -> axum::response::Response {
        s3_core::S3Error::NotImplemented.into_response()
    }

    pub async fn get_object(state: &Arc<AppState>, data: S3Data) -> axum::response::Response {
        s3_core::S3Error::NotImplemented.into_response()
    }
}
