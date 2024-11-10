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
    pub size: i64,
    pub owner_id: i64,
    pub version_id: uuid::Uuid,
    pub is_latest: bool,
    pub is_delete_marker: bool,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub etag: String,
}

#[derive(Debug, Default)]
pub struct Multipart {
    pub bucket_id: uuid::Uuid,
    pub object_id: uuid::Uuid,
    pub upload_id: uuid::Uuid,
}

#[derive(Debug, Default)]
pub struct Part {
    pub bucket_id: uuid::Uuid,
}
