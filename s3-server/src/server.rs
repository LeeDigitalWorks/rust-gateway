use std::collections::HashMap;
use std::io::Error;
use std::sync::Arc;

use axum::extract::Request;
use axum::response::Response;
use axum::routing::any;
use axum::{extract::State, Router};
use s3_backend::memory;
use s3_core::S3Error;
use s3_iam::iam::StreamKeysRequest;
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::RwLock;

use crate::authz::Authz;
use crate::filter::{
    run_filters, AuthenticationFilter, Filter, ParserFilter, RequestIdFilter, S3Data,
    SecretKeyFilter,
};

pub struct Server {
    pub addr: String,
    pub app_state: Arc<RwLock<AppState>>,
    pub router: Router,
}

impl Server {
    pub async fn new(
        addr: String,
        hosts: Vec<String>,
        client: s3_iam::iam::iam_client::IamClient<tonic::transport::Channel>,
    ) -> Self {
        let backend = Arc::new(memory::InMemoryBackend::new());
        let keys = Arc::new(Self::refresh_keys(client.clone()).await);

        let filters: Vec<Box<dyn Filter>> = vec![
            Box::new(RequestIdFilter::new()),
            Box::new(AuthenticationFilter::new(Authz::new(client))),
            Box::new(ParserFilter::new(hosts)),
            Box::new(SecretKeyFilter::new()),
        ];
        let app_state = Arc::new(RwLock::new(AppState {
            backend,
            keys,
            filters,
        }));

        let mut server = Server {
            addr,
            app_state: Arc::clone(&app_state),
            router: Router::new(),
        };

        let app = Router::new()
            .route("/", any(Self::handle_request))
            .route("/*rest", any(Self::handle_request))
            .with_state(Arc::clone(&app_state));

        server.router = app;
        server
    }

    pub async fn start(self) -> Result<(), Error> {
        let listener = tokio::net::TcpListener::bind(&self.addr).await.unwrap();
        tracing::debug!("listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, self.router)
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

    async fn handle_request(State(state): State<Arc<RwLock<AppState>>>, req: Request) -> Response {
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
        let mut write_only = state.write().await;
        let filters = &mut write_only.filters;
        let response = run_filters(filters, &mut data).await;
        if let Err(error) = response {
            return axum::response::IntoResponse::into_response(error);
        }
        drop(write_only);

        // TODO: Route to the correct handler
        match data.action {
            s3_core::S3Action::ListBuckets => {
                return Self::list_buckets(&state).await;
            }
            s3_core::S3Action::CreateBucket => {
                return Self::create_bucket(&state, data.bucket_name).await;
            }
            s3_core::S3Action::DeleteBucket => {
                return Self::delete_bucket(&state, data.bucket_name).await;
            }
            _ => {
                return axum::response::IntoResponse::into_response(S3Error::NotImplemented);
            }
        }
    }
}

pub struct AppState {
    pub backend: Arc<dyn s3_backend::Backend>,
    pub keys: Arc<HashMap<String, s3_iam::iampb::iam::Key>>,
    pub filters: Vec<Box<dyn Filter>>,
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
