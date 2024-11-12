use std::{num::NonZero, str::FromStr, sync::Arc, vec};

use aws_config::Region;
use aws_sdk_s3::config::{Credentials, SharedCredentialsProvider};
use config::Config;
use tonic::transport::Endpoint;
use tracing_subscriber::EnvFilter;

mod backend;
mod config;
mod filter;
mod handler;
mod router;
mod server;
mod signature;

fn create_storage(config: &Config) -> Box<crate::backend::storage::storage::StorageBackend> {
    let access_key_id = std::env::var("AWS_ACCESS_KEY_ID").unwrap_or_default();
    let secret_access_key = std::env::var("AWS_SECRET_ACCESS_KEY").unwrap_or_default();

    tracing::debug!("Using AWS credentials: {}", access_key_id);

    let sdk_config = aws_config::SdkConfig::builder()
        .region(Region::new(config.region.clone()))
        .endpoint_url("https://sfo3.digitaloceanspaces.com")
        .credentials_provider(SharedCredentialsProvider::new(Credentials::new(
            access_key_id,
            secret_access_key,
            None,
            None,
            "",
        )))
        .build();

    let storage =
        backend::storage::storage::StorageBackend::new(aws_sdk_s3::Client::new(&sdk_config));
    Box::new(storage)
}

fn create_indexer(config: &Config) -> Result<Box<crate::backend::FullstackBackend>, String> {
    match config.meta_store.as_str() {
        "postgresdb" => {
            let conn_str = config
                .postgresdb_info
                .as_ref()
                .ok_or_else(|| "Missing postgresdb_info")?;
            let pool = sqlx::postgres::PgPoolOptions::new()
                .max_connections(10240)
                .max_lifetime(std::time::Duration::from_secs(300))
                .connect_lazy(&conn_str)
                .map_err(|e| format!("Failed to connect to postgres: {}", e))?;

            let postgres = Box::new(backend::db_reader::Database::new(pool));
            let storage = create_storage(config);
            Ok(Box::new(backend::FullstackBackend::new(postgres, storage)))
        }
        _ => Err(format!("Unknown meta_store: {}", config.meta_store)),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config.toml".to_string());
    let config = config::Config::from_file(&config_path)?;

    let level = tracing::Level::from_str(&config.log_level)?;
    let mut filter_string = "".to_string();
    if config.debug_mode {
        filter_string = format!("{}=debug", env!("CARGO_CRATE_NAME"));
    }

    let env_filter = EnvFilter::builder()
        .with_default_directive(level.into())
        .parse(filter_string)?;
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .try_init()
        .unwrap();

    let endpoint = Endpoint::from_str(&config.iam_address)?;
    let iam_client = s3_iam::iampb::iam::iam_client::IamClient::new(endpoint.connect_lazy());

    let backend = create_indexer(&config)?;

    let redis_client = redis::cluster::ClusterClientBuilder::new(vec![config.redis_address])
        .username(config.redis_username.clone())
        .password(config.redis_password.clone())
        .connection_timeout(std::time::Duration::from_secs(config.redis_connect_timeout))
        .response_timeout(std::time::Duration::from_secs(config.redis_read_timeout))
        .build()?;
    let local_rate_limiter: governor::DefaultKeyedRateLimiter<String> =
        governor::RateLimiter::keyed(governor::Quota::per_second(NonZero::new(10).unwrap()));

    tracing::info!("Starting server on {}", &config.bind_api_address);
    let server = server::Server::new(
        config.bind_api_address,
        config.s3domain,
        iam_client,
        Arc::new(backend),
        redis_client,
        local_rate_limiter,
    )
    .await;
    Ok(server.start().await?)
}
