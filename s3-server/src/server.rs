use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

use axum::extract::Request;
use axum::response::Response;
use axum::routing::any;
use axum::{extract::State, Router};
use s3_core::S3Error;
use s3_iam::iam::StreamKeysRequest;
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::RwLock;

use crate::authz::{Authz, Key};
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
        let backend = Arc::new(crate::backend::InMemoryBackend::new());
        let keys = Arc::new(RwLock::new(HashMap::new()));

        // Refresh keys
        tokio::spawn(Self::refresh_keys(client.clone(), keys.clone()));

        let filters: Vec<Box<dyn Filter>> = vec![
            Box::new(RequestIdFilter::new()),
            Box::new(AuthenticationFilter::new(Authz::new(keys.clone()))),
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

    pub async fn start(self) -> Result<(), Box<dyn Error>> {
        let listener = tokio::net::TcpListener::bind(&self.addr).await.unwrap();
        tracing::debug!("listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, self.router)
            .with_graceful_shutdown(shutdown_signal())
            .await
            .map_err(Into::into)
    }

    async fn refresh_keys(
        mut client: s3_iam::iam::iam_client::IamClient<tonic::transport::Channel>,
        keys: Arc<RwLock<HashMap<String, Key>>>,
    ) {
        loop {
            let request = tonic::Request::new(StreamKeysRequest::default());
            let stream = client.stream_keys(request).await;
            if stream.is_err() {
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                continue;
            }
            let mut new_keys = HashMap::new();
            let mut stream = stream.unwrap().into_inner();
            while let Some(resp) = stream.message().await.unwrap() {
                if let Some(key) = resp.key {
                    new_keys.insert(
                        key.access_key.clone(),
                        Key {
                            access_key: key.access_key,
                            secret_key: key.secret_key,
                            user_id: key.user_id,
                        },
                    );
                }
            }
            tracing::debug!(keys = ?new_keys, "Refreshed keys");
            let mut write_only = keys.write().await;
            *write_only = new_keys;
            drop(write_only);

            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }
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
                return Self::list_buckets(&state, data).await;
            }
            s3_core::S3Action::CreateBucket => {
                return Self::create_bucket(&state, data).await;
            }
            s3_core::S3Action::DeleteBucket => {
                return Self::delete_bucket(&state, data).await;
            }
            _ => {
                return axum::response::IntoResponse::into_response(S3Error::NotImplemented);
            }
        }
    }
}

pub struct AppState {
    pub backend: Arc<dyn crate::backend::Indexer>,
    pub keys: Arc<RwLock<HashMap<String, Key>>>,
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
