mod server;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{port}");
    server::start_server(&addr).await.unwrap();
}
