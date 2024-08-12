use serde_derive::Serialize;

#[derive(Serialize)]
#[serde(rename = "LocationConstraint")]
pub struct LocationResponse {
    // #[serde(rename = "@xmlns")]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // xmlns: Option<String>,
    
    #[serde(rename = "$value")]
    pub location: String,
}

pub struct ListObjectsResponse {
    pub common_prefixes: Vec<CommonPrefix>,
    pub delimiter: String,
    pub encoding_type: String,
    pub is_truncated: bool,
    pub max_keys: i64,
    pub key_count: i64,
    pub prefix: String,
    pub bucket_name: String,

    // v1 specific
    pub marker: String,
    pub next_marker: String,

    // v2 specific
    pub continuation_token: String,
    pub next_continuation_token: String,
    pub start_after: String,

    pub contents: Vec<Object>,
}

pub struct VersionedListObjectsResponse {
    pub contents: Vec<VersionedObject>,
    pub common_prefixes: Vec<CommonPrefix>,
    pub delimiter: String,
    pub encoding_type: String,
    pub is_truncated: bool,
    pub key_count: i64,
    pub max_keys: i64,
    pub prefix: String,
    pub bucket_name: String,
    pub key_marker: String,
    pub next_key_marker: String,
    pub version_id_marker: String,
    pub next_version_id_marker: String,
}

pub struct ListObjectsRequest {
    pub versioned: bool,
    pub version: i64,
    pub delimiter: String,
    pub encoding_type: String,
    pub max_keys: i64,
    pub prefix: String,

    // v1 specific
    pub marker: String,

    // v2 specific
    pub continuation_token: String,
    pub start_after: String,
    pub fetch_owner: bool,

    // versioned specific
    pub key_marker: String,
    pub version_id_marker: String,
}

pub struct ListUploadsRequest {
    pub delimiter: String,
    pub encoding_type: String,
    pub key_marker: String,
    pub max_uploads: i64,
    pub prefix: String,
    pub upload_id_marker: String,
}

pub struct ListPartsRequest {
    pub encoding_type: String,
    pub upload_id: String,
    pub max_parts: i64,
    pub part_number_marker: i64,
}

pub struct Part {
    pub part_number: i64,
    pub etag: String,
    pub last_modified: String,
    pub size: i64,
}

pub struct ListPartsResponse {
    pub bucket: String,
    pub key: String,
    pub upload_id: String,
    pub encoding_type: String,

    pub initiator: Initiator,
    pub owner: Owner,

    pub storage_class: String,

    pub part_number_marker: i64,
    pub next_part_number_marker: i64,
    pub max_parts: i64,
    pub is_truncated: bool,

    pub parts: Vec<Part>,
}

pub struct ListBucketsResponse {
    pub owner: Owner,
    pub buckets: Vec<Bucket>,
}

pub struct Upload {
    pub key: String,
    pub upload_id: String,
    pub intiator: Initiator,
    pub owner: Owner,
    pub storage_class: String,
    pub initiated: String, // time string of the format "YYYY-MM-DDTHH:MM:SS.000Z"
}

pub struct CommonPrefix {
    pub prefix: String,
}

pub struct Bucket {
    pub name: String,
    pub creation_date: String,
}

pub struct Object {
    pub key: String,
    pub last_modified: String,
    pub etag: String,
    pub size: i64,
    pub storage_class: String,
    pub owner: Owner,
}

pub struct VersionedObject {
    pub key: String,
    pub version_id: String,
    pub is_latest: bool,
    pub last_modified: String,
    pub etag: String,
    pub size: i64,
    pub storage_class: String,
    pub owner: Owner,
}

pub struct CopyObjectResponse {
    pub last_modified: String,
    pub etag: String,
}

pub struct RenameObjectResponse {
    pub last_modified: String,
}

pub struct CopyObjectPartResponse {
    pub etag: String,
    pub last_modified: String,
}

pub struct Initiator(Owner);

pub struct Owner {
    pub id: String,
    pub display_name: String,
}

pub struct InitiateMultipartUploadResponse {
    pub bucket: String,
    pub key: String,
    pub upload_id: String,
}

pub struct CompleteMultipartUploadRequest {
    pub bucket: String,
    pub key: String,
    pub location: String,
    pub etag: String,
}

pub struct PostResponse {
    pub location: String,
    pub bucket: String,
    pub key: String,
    pub etag: String,
}

pub struct DeleteError {
    pub code: String,
    pub message: String,
    pub key: String,
    pub version_id: String,
}

