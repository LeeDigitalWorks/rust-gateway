use std::collections::HashMap;

use config_file::FromConfigFile;
use once_cell::sync::Lazy;
use serde_derive::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub s3domain: Vec<String>, // domain names used for hostnames
    pub region: String,

    pub plugins: HashMap<String, PluginConfig>,
    pub ceph_map: HashMap<String, CephConfig>, // used to point to different ceph backends
    pub rgw_map: HashMap<String, RGWConfig>,   // used to point to different rgw backends
    pub s3_map: HashMap<String, S3Config>,     // used to point to different s3 backends

    pub log_path: String,
    pub access_log_path: String,
    pub access_log_format: String,
    pub bind_api_address: String,
    pub bind_tls_address: String,
    pub bind_admin_address: String,
    pub bind_admin_grpc_address: String,
    pub ssl_key_path: String,
    pub ssl_cert_path: String,

    pub instance_id: String,
    pub concurrent_request_limit: i64,
    pub debug_mode: bool,
    pub enable_pprof: bool,
    pub bind_pprof_address: String,
    pub admin_key: String,
    pub gc_thread: i64,
    pub lc_thread: i64,
    pub log_level: String,
    pub reserved_origins: String, // www.ccc.com,www.bbb.com,127.0.0.1
    pub meta_store: String,       // type of meta store to use
    pub db_info: String,          // connection string for the meta store
    pub keep_alive: bool,
    pub enable_compression: bool,

    // Cache
    pub redis_addr: String,
    pub redis_connection_number: i64,
    pub redis_password: String,
    pub meta_cache_type: i64,
    pub enable_data_cache: bool,
    pub redis_connection_timeout: i64,
    pub redis_read_timeout: i64,
    pub redis_write_timeout: i64,
    pub redis_keep_alive: i64,
    pub redis_max_idle: i64,
    pub redis_idle_timeout: i64,

    // DB Connection parameters
    pub db_max_open_conns: i64,
    pub db_max_idle_conns: i64,
    pub db_conn_max_lifetime: i64,

    // If the value is not 0, the cached ping detection will be turned on, and the interval is the number of seconds.
    pub cache_circuit_check_interval: i64,
    // This property sets the amount of seconds, after tripping the circuit,
    // to reject requests before allowing attempts again to determine if the circuit should again be closed.
    pub cache_circuit_close_sleep_window: i64,
    // This value is how may consecutive passing requests are required before the circuit is closed
    pub cache_circuit_close_required_count: i64,
    // This property sets the minimum number of requests in a rolling window that will trip the circuit.
    pub cache_circuit_open_threshold: i64,
    pub cache_circuit_exec_timeout: i64,
    pub cache_circuit_exec_max_concurrent: i64,

    pub download_buf_pool_size: i64,
    pub upload_min_chunk_size: i64,
    pub upload_max_chunk_size: i64,
}

#[derive(Debug, Default, Deserialize)]
pub struct PluginConfig {
    pub path: String,
    pub enable: bool,
    pub args: HashMap<String, String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct CephConfig {
    pub mon_hosts: String,
    pub user_name: String,
    pub user_key: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct RGWConfig {
    pub endpoint: String,
    pub zone_name: String,
    pub data_pool: String,
    pub index_pool: String,
    pub extra_pool: String,
    pub uid: String,
    pub access_key: String,
    pub secret_key: String,
    pub ceph_fsid: String,
    pub ceph_name: String,
    pub override_prefix: String,
    pub read_only: bool,
    pub default: bool,
}

#[derive(Debug, Default, Deserialize)]
pub struct S3Config {
    pub endpoint: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
    pub region: String,
}

pub const CONFIG: Lazy<Config> =
    Lazy::new(|| Config::from_config_file("config.toml").unwrap_or_default());
