mod server;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let iam_addr = "http://0.0.0.0:8000";
    let client = s3_iam::iampb::iam::iam_client::IamClient::connect(iam_addr)
        .await
        .unwrap();

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{port}");
    server::start_server(&addr, client).await.unwrap();
}
