use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
pub struct Bucket {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "CreationDate")]
    pub creation_date: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub key: String,
    pub size: usize,
    pub last_modified: DateTime<Utc>,
    pub etag: String,
    pub storage_class: StorageClass,
    pub metadata: ObjectMetadata,
    pub data: Vec<u8>,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
pub struct Owner {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "DisplayName")]
    pub display_name: String,
}

#[derive(Deserialize, Serialize, Default, Debug, Clone, PartialEq)]
pub struct BucketContainer {
    pub buckets: Vec<Bucket>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectMetadata {
    pub content_type: Option<String>,
    pub content_disposition: Option<String>,
    pub cache_control: Option<String>,
    pub user_metadata: HashMap<String, String>,
}

impl Default for ObjectMetadata {
    fn default() -> Self {
        ObjectMetadata {
            content_type: None,
            content_disposition: None,
            cache_control: None,
            user_metadata: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StorageClass {
    Standard,
    ReducedRedundancy,
    Glacier,
}

impl Default for StorageClass {
    fn default() -> Self {
        StorageClass::Standard
    }
}
