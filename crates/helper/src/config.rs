use std::collections::HashMap;

use once_cell::sync::Lazy;

#[derive(Debug, Default)]
pub struct Config {
    pub domain: String, // domain name used for hostnames
    pub region: String,

    pub s3_map: HashMap<String, S3Config>, // used to point to different s3 backends
}

#[derive(Debug, Default)]
pub struct S3Config {
    pub endpoint: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
    pub region: String,
}

static CONFIG: Lazy<Config> = Lazy::new(|| Config {
    domain: "example.com".to_string(),
    ..Default::default()
});
