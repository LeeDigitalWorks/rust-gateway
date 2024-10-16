use std::collections::HashMap;
use std::env;
use std::io::Error;
use std::sync::Arc;

use axum::extract::{Host, Path, Query};
use axum::http::HeaderMap;
use axum::middleware;
use axum::response::{IntoResponse, Response};
use axum::routing::any;
use axum::{extract::State, routing::get, Router};
use s3_backend::memory;
use s3_core::S3Error;
use s3_iam::iam::StreamKeysRequest;
use tokio::signal::unix::{signal, SignalKind};

use crate::authz::is_req_authenticated;
use crate::handler::list_buckets;
use crate::limiter::is_req_limited;

const DEFAULT_S3_HOST: &str = "127.0.0.1:3000";

pub struct AppState {
    pub backend: Arc<dyn s3_backend::Backend>,
    pub keys: Arc<HashMap<String, s3_iam::iampb::iam::Key>>,
}

pub async fn start_server(
    addr: &str,
    client: s3_iam::iam::iam_client::IamClient<tonic::transport::Channel>,
) -> Result<(), Error> {
    let backend = Arc::new(memory::InMemoryBackend::new());
    let keys = Arc::new(refresh_keys(client).await);
    let state = Arc::new(AppState { backend, keys });

    let app = Router::new()
        .route("/", get(list_buckets))
        .route("/*rest", any(handle_request))
        .with_state(Arc::clone(&state))
        .layer(middleware::from_fn_with_state(
            Arc::clone(&state),
            is_req_authenticated,
        ))
        .layer(middleware::from_fn(is_req_limited));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(Into::into)
}

async fn refresh_keys(
    mut client: s3_iam::iam::iam_client::IamClient<tonic::transport::Channel>,
) -> HashMap<String, s3_iam::iampb::iam::Key> {
    let request = tonic::Request::new(StreamKeysRequest::default());
    let mut stream = client.stream_keys(request).await.unwrap().into_inner();
    let mut keys = HashMap::new();
    while let Some(resp) = stream.message().await.unwrap() {
        if let Some(key) = resp.key {
            keys.insert(key.access_key.clone(), key);
        }
    }
    tracing::debug!(keys = ?keys, "Refreshed keys");
    keys
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

async fn handle_request(
    Host(host): Host,
    Path(rest): Path<String>,
    Query(query): Query<HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Response {
    let root_host = env::var("S3_HOST").unwrap_or_else(|_| DEFAULT_S3_HOST.to_string());

    tracing::debug!(host = ?host, rest = ?rest, query = ?query, "Handling request");

    // Handle path style requests
    if root_host == host {
        let bucket = rest.split('/').next().unwrap().to_string();
        let rest = rest.split('/').skip(1).collect::<Vec<_>>().join("/");
        let response = handle_bucket_request(bucket, rest, query).await;
        match response {
            Ok(_) => return axum::response::IntoResponse::into_response("".to_string()),
            Err(error) => return axum::response::IntoResponse::into_response(error),
        }
    }
    // Handle virtual host style requests
    else {
        let bucket = host.split('.').next().unwrap().to_string();
        let response = handle_bucket_request(bucket, rest, query).await;
        match response {
            Ok(_) => return axum::response::IntoResponse::into_response("".to_string()),
            Err(error) => return axum::response::IntoResponse::into_response(error),
        }
    }
}

async fn handle_bucket_request(
    bucket: String,
    rest: String,
    params: HashMap<String, String>,
) -> Result<(), S3Error> {
    Ok(())
}
