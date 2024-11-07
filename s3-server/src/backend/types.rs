#[derive(Debug, Default)]
pub struct Bucket {
    pub id: uuid::Uuid,
    pub name: String,
    pub user_id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Default)]
pub struct Object {
    pub bucket_id: uuid::Uuid,
    pub key: String,
    pub version: uuid::Uuid,
    pub is_latest: bool,
    pub is_delete_marker: bool,
    pub size: u64,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub etag: String,
}
