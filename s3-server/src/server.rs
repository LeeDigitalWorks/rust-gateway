use std::collections::HashMap;
use std::env;
use std::io::Error;
use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::{FromRequest, Host, Path, Query, Request};
use axum::http::HeaderMap;
use axum::response::Response;
use axum::routing::any;
use axum::{extract::State, routing::get, Router};
use s3_backend::memory;
use s3_core::S3Error;
use s3_iam::iam::StreamKeysRequest;
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::RwLock;

use crate::authz::Authz;
use crate::filter::{
    run_filters, AuthenticationFilter, Filter, ParserFilter, RequestIdFilter, S3Data,
};

const DEFAULT_S3_HOST: &str = "127.0.0.1:3000";

pub struct AppState {
    pub backend: Arc<dyn s3_backend::Backend>,
    pub keys: Arc<HashMap<String, s3_iam::iampb::iam::Key>>,
    pub filters: Vec<Box<dyn Filter>>,
}

pub async fn start_server(
    addr: &str,
    hosts: Vec<String>,
    client: s3_iam::iam::iam_client::IamClient<tonic::transport::Channel>,
) -> Result<(), Error> {
    let backend = Arc::new(memory::InMemoryBackend::new());
    let keys = Arc::new(refresh_keys(client.clone()).await);
    let filters: Vec<Box<dyn Filter>> = vec![
        Box::new(RequestIdFilter::new()),
        Box::new(AuthenticationFilter::new(Authz::new(client))),
        Box::new(ParserFilter::new(hosts)),
    ];
    let state = Arc::new(RwLock::new(AppState {
        backend,
        keys,
        filters,
    }));

    let app = Router::new()
        .route("/", any(handle_request))
        .with_state(Arc::clone(&state));

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

async fn handle_request(State(state): State<Arc<RwLock<AppState>>>, req: Request) -> Response {
    let root_host = env::var("S3_HOST").unwrap_or_else(|_| DEFAULT_S3_HOST.to_string());

    let (parts, body) = req.into_parts();
    let body = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(body) => body,
        Err(_) => {
            return axum::response::IntoResponse::into_response(S3Error::InvalidRequest);
        }
    };
    let request = Request::<axum::body::Bytes>::from_parts(parts, body);

    let mut data = S3Data::new();
    data.req = request;
    let mut filters = &mut state.write().await.filters;
    let response = run_filters(&mut filters, &mut data).await;

    if let Err(error) = response {
        return axum::response::IntoResponse::into_response(error);
    }

    axum::response::IntoResponse::into_response("".to_string())
}
