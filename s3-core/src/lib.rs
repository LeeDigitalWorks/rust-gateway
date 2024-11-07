#[macro_use]
extern crate serde_derive;

pub mod error;
pub mod request;
pub mod response;
pub mod types;
pub mod util;
pub mod versioning;

pub use error::S3Error;
pub use request::S3Action;
pub use types::{Bucket, Object, ObjectMetadata, StorageClass};
pub use util::*;
