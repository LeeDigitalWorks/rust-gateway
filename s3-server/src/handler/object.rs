use std::sync::Arc;

use crate::{
    filter::S3Data,
    server::{AppState, Server},
};

impl Server {
    pub async fn put_object(state: &Arc<AppState>, data: &mut S3Data) -> axum::response::Response {
        let response = state.fullstack.put_object(data).await;
        axum::response::IntoResponse::into_response(response)
    }

    pub async fn get_object(state: &Arc<AppState>, data: &mut S3Data) -> axum::response::Response {
        let response = state.fullstack.get_object(data).await;
        axum::response::IntoResponse::into_response(response)
    }
}
