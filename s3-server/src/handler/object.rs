use std::sync::Arc;

use axum::response::IntoResponse;

use crate::{
    filter::S3Data,
    server::{AppState, Server},
};

impl Server {
    pub async fn put_object(state: &Arc<AppState>, data: &mut S3Data) -> axum::response::Response {
        let response = state.fullstack.put_object(data).await;
        tracing::info!("Put object response: {:?}", response);
        s3_core::S3Error::NotImplemented.into_response()
    }

    pub async fn get_object(state: &Arc<AppState>, data: &mut S3Data) -> axum::response::Response {
        let response = state.fullstack.get_object(data).await;
        tracing::info!("Get object response: {:?}", response);
        s3_core::S3Error::NotImplemented.into_response()
    }
}
