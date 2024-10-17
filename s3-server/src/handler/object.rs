use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
};

use crate::server::AppState;

pub async fn get_object(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
) -> impl IntoResponse {
    let response = state.backend.get_object("", &key).await;
    match response {
        Ok(data) => axum::response::IntoResponse::into_response(data),
        Err(e) => axum::response::IntoResponse::into_response(e),
    }
}
