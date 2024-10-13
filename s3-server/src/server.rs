use std::io::Error;
use std::sync::Arc;

use axum::http::HeaderMap;
use axum::response::Response;
use axum::{extract::State, routing::get, Router};
use s3_backend::memory;
use tokio::signal::unix::{signal, SignalKind};

struct AppState {
    backend: Arc<dyn s3_backend::Backend>,
}

pub async fn start_server(addr: &str) -> Result<(), Error> {
    let backend = Arc::new(memory::InMemoryBackend::new());
    let state = Arc::new(AppState { backend });
    let app = Router::new()
        .route("/", get(list_buckets))
        .with_state(state)
        .fallback(invalid_request);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(Into::into)
}

async fn shutdown_signal() {
    let mut sigterm = signal(SignalKind::terminate()).unwrap();
    let mut sighup = signal(SignalKind::hangup()).unwrap();
    let mut sigint = signal(SignalKind::interrupt()).unwrap();
    tokio::select! {
        _ = sigterm.recv() => {}
        _ = sighup.recv() => {}
        _ = sigint.recv() => {}
    }
}

async fn invalid_request() -> &'static str {
    "Invalid request"
}

async fn list_buckets(State(state): State<Arc<AppState>>, headers: HeaderMap) -> Response {
    tracing::debug!(headers = ?headers, "Listing buckets");
    let response = state.backend.list_buckets().await;
    match response {
        Ok(response) => axum::response::IntoResponse::into_response(response.into_response()),
        Err(error) => axum::response::IntoResponse::into_response(error),
    }
}

async fn create_bucket() -> &'static str {
    "Bucket created"
}

async fn delete_bucket() -> &'static str {
    "Bucket deleted"
}
