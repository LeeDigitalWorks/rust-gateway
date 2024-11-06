mod authz;
mod backend;
mod filter;
mod handler;
mod limiter;
mod router;
mod server;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let iam_addr = "http://0.0.0.0:8000";
    let client = s3_iam::iampb::iam::iam_client::IamClient::connect(iam_addr)
        .await
        .unwrap();

    let hosts = vec!["127.0.0.1:3000".to_string()];

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{port}");
    let server = server::Server::new(addr, hosts, client).await;
    server.start().await.unwrap();
}
