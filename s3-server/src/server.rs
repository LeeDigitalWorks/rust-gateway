use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;

use aws_sdk_s3::primitives::ByteStream;
use axum::body::Body;
use axum::extract::{ConnectInfo, Request};
use axum::response::{IntoResponse, Response};
use axum::routing::any;
use axum::{extract::State, Router};
use s3_core::S3Error;
use s3_iam::iam::StreamKeysRequest;
use sync_wrapper::SyncStream;
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::RwLock;

use crate::filter::{
    AuthenticationFilter, BucketFilter, Filter, FilterChain, ParserFilter, RateLimitFilter,
    RequestIdFilter, S3Data, SecretKeyFilter,
};
use crate::signature::{Key, SignatureValidator};

pub struct Server {
    pub addr: String,
    pub router: Router,
}

impl Server {
    pub async fn new(
        addr: String,
        hosts: Vec<String>,
        client: s3_iam::iam::iam_client::IamClient<tonic::transport::Channel>,
        fullstack: Arc<Box<crate::backend::FullstackBackend>>,
        redis_client: redis::cluster::ClusterClient,
        local_rate_limiter: governor::DefaultKeyedRateLimiter<String>,
    ) -> Self {
        let keys = Arc::new(RwLock::new(HashMap::new()));

        // Refresh keys
        tokio::spawn(Self::refresh_keys(client.clone(), keys.clone()));

        let filters: Vec<Box<dyn Filter>> = vec![
            Box::new(RequestIdFilter::new()),
            Box::new(AuthenticationFilter::new(SignatureValidator::new(
                keys.clone(),
            ))),
            Box::new(ParserFilter::new(hosts)),
            Box::new(RateLimitFilter::new(redis_client, local_rate_limiter)),
            Box::new(SecretKeyFilter::new(keys.clone())),
            Box::new(BucketFilter::new(fullstack.clone())),
        ];
        let filter_chain = Arc::new(FilterChain::new(filters));
        let app_state = Arc::new(AppState {
            fullstack,
            filter_chain,
        });

        let mut server = Server {
            addr,
            router: Router::new(),
        };

        let app = Router::new()
            .route("/", any(Self::handle_request))
            .route("/*rest", any(Self::handle_request))
            .with_state(app_state);

        server.router = app;
        server
    }

    pub async fn start(self) -> Result<(), Box<dyn Error>> {
        let listener = tokio::net::TcpListener::bind(&self.addr).await.unwrap();
        axum::serve(
            listener,
            self.router
                .into_make_service_with_connect_info::<SocketAddr>(),
        )
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
            let mut write_only = keys.write().await;
            *write_only = new_keys;
            drop(write_only);

            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }
    }

    async fn handle_request(
        State(state): State<Arc<AppState>>,
        ConnectInfo(addr): ConnectInfo<SocketAddr>,
        req: Request<Body>,
    ) -> Response {
        let req =
            req.map(|body| reqwest::Body::wrap_stream(SyncStream::new(body.into_data_stream())));

        let mut data = S3Data::new();
        data.req = req;
        data.req.headers_mut().insert(
            "x-real-ip",
            reqwest::header::HeaderValue::from_str(&addr.ip().to_string()).unwrap(),
        );
        match state.filter_chain.run_filters(&mut data).await {
            Ok(_) => {}
            Err(e) => {
                return e.into_response();
            }
        }

        // TODO: Route to the correct handler
        match data.action {
            s3_core::S3Action::ListBuckets => {
                return Self::list_buckets(&state, &mut data).await;
            }
            s3_core::S3Action::CreateBucket => {
                return Self::create_bucket(&state, &mut data).await;
            }
            s3_core::S3Action::DeleteBucket => {
                return Self::delete_bucket(&state, &mut data).await;
            }
            s3_core::S3Action::PutObject => {
                return Self::put_object(&state, &mut data).await;
            }
            s3_core::S3Action::GetObject => {
                return Self::get_object(&state, &mut data).await;
            }
            _ => {
                return axum::response::IntoResponse::into_response(S3Error::NotImplemented);
            }
        }
    }
}

pub struct AppState {
    pub fullstack: Arc<Box<crate::backend::FullstackBackend>>,
    pub filter_chain: Arc<FilterChain>,
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
