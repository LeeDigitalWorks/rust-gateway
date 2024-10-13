#[macro_use]
extern crate serde_derive;

pub mod types;
pub mod request;
pub mod error;
pub mod response;

pub use types::{Bucket, Object, ObjectMetadata, StorageClass};
pub use request::S3Request;
pub use error::{S3Error, error_to_http_status};
