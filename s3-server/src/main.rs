use std::str::FromStr;

use tonic::transport::Endpoint;
use tracing_subscriber::EnvFilter;

mod authz;
mod backend;
mod config;
mod filter;
mod handler;
mod limiter;
mod router;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config.toml".to_string());
    let config = config::Config::from_file(&config_path)?;

    let level = tracing::Level::from_str(&config.log_level)?;

    let env_filter = EnvFilter::builder()
        .with_default_directive(level.into())
        .parse(format!("{}=debug,warn", env!("CARGO_CRATE_NAME")))?;
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .try_init()
        .unwrap();

    let endpoint = Endpoint::from_str(&config.iam_address)?;
    let client = s3_iam::iampb::iam::iam_client::IamClient::new(endpoint.connect_lazy());

    tracing::info!("Starting server on {}", &config.bind_api_address);
    let server = server::Server::new(config.bind_api_address, config.s3domain, client).await;
    Ok(server.start().await?)
}
