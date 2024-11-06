use std::collections::HashMap;

mod server;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let env = std::env::var("ENV").unwrap_or_else(|_| "development".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let addr = format!("0.0.0.0:{port}");

    tracing::info!("Starting S3 IAM server on {}", addr);
    tracing::info!("Environment: {}", env);

    if env == "development" {
        let mut keys = HashMap::new();

        keys.insert(
            "RGTEST".to_string(),
            s3_iam::iam::Key {
                name: "TestKey".to_string(),
                user_id: 1,
                access_key: "RGTEST".to_string(),
                secret_key: "RGSECRET".to_string(),
                ..Default::default()
            },
        );

        let server = server::S3IAMServer::with_keys(keys);
        server::start_server(&addr, server).await.unwrap();
    } else {
        let server = server::S3IAMServer::new();
        server::start_server(&addr, server).await.unwrap();
    }
}
