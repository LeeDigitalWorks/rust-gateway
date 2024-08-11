use aws_sdk_s3::types;

use crate::{credential, datatype};

pub trait ObjectLayer {
    // Bucket Operations
    fn make_bucket(
        &self,
        bucket: String,
        acl: datatype::Acl,
        credential: credential::Credential,
    ) -> Result<(), anyhow::Error>;
    fn set_bucket_logging(
        &self,
        bucket: String,
        config: types::BucketLoggingStatus,
    ) -> Result<(), anyhow::Error>;
    fn get_bucket_logging(
        &self,
        bucket: String,
    ) -> Result<types::BucketLoggingStatus, anyhow::Error>;
    fn set_bucket_lifecycle(
        &self,
        bucket: String,
        config: types::BucketLifecycleConfiguration,
        credential: credential::Credential,
    ) -> Result<(), anyhow::Error>;
    fn get_bucket_lifecycle(
        &self,
        bucket: String,
        credential: credential::Credential,
    ) -> Result<types::BucketLifecycleConfiguration, anyhow::Error>;
    fn delete_bucket_lifecycle(
        &self,
        bucket: String,
        credential: credential::Credential,
    ) -> Result<(), anyhow::Error>;
    fn set_bucket_acl(
        &self,
        bucket: String,
        policy: types::AccessControlPolicy,
        acl: datatype::Acl,
        credential: credential::Credential,
    ) -> Result<(), anyhow::Error>;
    fn get_bucket_acl(
        &self,
        bucket: String,
        credential: credential::Credential,
    ) -> Result<types::AccessControlPolicy, anyhow::Error>;
    fn set_bucket_cors(
        &self,
        bucket: String,
        cors: types::CorsConfiguration,
        credential: credential::Credential,
    ) -> Result<(), anyhow::Error>;
    fn set_bucket_versioning(
        &self,
        bucket: String,
        versioning: types::BucketVersioningStatus,
        credential: credential::Credential,
    ) -> Result<(), anyhow::Error>;
    fn delete_bucket_cors(
        &self,
        bucket: String,
        credential: credential::Credential,
    ) -> Result<(), anyhow::Error>;
    fn get_bucket_versioning(
        &self,
        bucket: String,
        credential: credential::Credential,
    ) -> Result<types::BucketVersioningStatus, anyhow::Error>;
    fn get_bucket_cors(
        &self,
        bucket: String,
        credential: credential::Credential,
    ) -> Result<types::CorsConfiguration, anyhow::Error>;
    fn get_bucket(
        &self,
        bucket: String,
        credential: credential::Credential,
    ) -> Result<(), anyhow::Error>; // TODO: Return internal bucket and new datatype
    fn list_buckets(
        &self,
        credential: credential::Credential,
    ) -> Result<Vec<String>, anyhow::Error>; // TODO: Return new datatype
    fn delete_bucket(
        &self,
        bucket: String,
        credential: credential::Credential,
    ) -> Result<(), anyhow::Error>;
    fn list_objects(
        &self,
        bucket: String,
        credential: credential::Credential,
        request: datatype::ListObjectsRequest,
    ) -> Result<Vec<String>, anyhow::Error>; // TODO: Return new datatype
    fn list_versioned_objects(
        &self,
        bucket: String,
        credential: credential::Credential,
        request: datatype::ListObjectsRequest,
    ) -> Result<Vec<String>, anyhow::Error>; // TODO: Return new datatype

    // Policy Operations
    fn set_bucket_policy(
        &self,
        bucket: String,
        policy: String, // TODO: Change to new datatype
        credential: credential::Credential,
    ) -> Result<(), anyhow::Error>;
    fn get_bucket_policy(
        &self,
        bucket: String,
        credential: credential::Credential,
    ) -> Result<String, anyhow::Error>; // TODO: Change to new datatype
    fn delete_bucket_policy(
        &self,
        bucket: String,
        credential: credential::Credential,
    ) -> Result<(), anyhow::Error>;

    // Website Operations
    fn set_bucket_website(
        &self,
        bucket: String,
        website: types::WebsiteConfiguration,
        credential: credential::Credential,
    ) -> Result<(), anyhow::Error>;
    fn get_bucket_website(
        &self,
        bucket: String,
        credential: credential::Credential,
    ) -> Result<types::WebsiteConfiguration, anyhow::Error>;
    fn delete_bucket_website(
        &self,
        bucket: String,
        credential: credential::Credential,
    ) -> Result<(), anyhow::Error>;

    // Encryption Operations
    fn set_bucket_encryption(
        &self,
        bucket: String,
        encryption: types::ServerSideEncryptionConfiguration,
        credential: credential::Credential,
    ) -> Result<(), anyhow::Error>;
    fn get_bucket_encryption(
        &self,
        bucket: String,
        credential: credential::Credential,
    ) -> Result<types::ServerSideEncryptionConfiguration, anyhow::Error>;
    fn delete_bucket_encryption(
        &self,
        bucket: String,
        credential: credential::Credential,
    ) -> Result<(), anyhow::Error>;

    // Object operations
    fn get_object(
        &self,
        bucket: String,
        object: String,
        start_offset: i64,
        length: i64,
        writer: &mut dyn std::io::Write,
        credential: credential::Credential,
    ) -> Result<(), anyhow::Error>;
    fn get_object_info(
        &self,
        bucket: String,
        object: String,
        credential: credential::Credential,
    ) -> Result<types::Object, anyhow::Error>; // TODO: replace with new datatype
    fn put_object(
        &self,
        bucket: String,
        object: String,
        reader: &mut dyn std::io::Read,
        length: i64,
        metadata: types::MetadataEntry,
        acl: datatype::Acl,
        // sse: datatype::SseRequest,
        // storage_class: datatype::StorageClass,
        credential: credential::Credential,
    ) -> Result<(), anyhow::Error>;
    fn append_object(
        &self,
        bucket: String,
        object: String,
        reader: &mut dyn std::io::Read,
        length: i64,
        offset: i64,
        metadata: types::MetadataEntry,
        acl: datatype::Acl,
        // sse: datatype::SseRequest,
        // storage_class: datatype::StorageClass,
        credential: credential::Credential,
    ) -> Result<(), anyhow::Error>;
    fn copy_object(
        &self,
        source_object: String,
        target_object: String,
        source: &mut dyn std::io::Read,
        is_metadata_only: bool,
        // sse: datatype::SseRequest,
        credential: credential::Credential,
    ) -> Result<datatype::CopyObjectResponse, anyhow::Error>;
    fn rename_object(
        &self,
        source_object: String,
        target_object: String,
        credential: credential::Credential,
    ) -> Result<datatype::RenameObjectResponse, anyhow::Error>;
    fn put_object_meta(
        &self,
        bucket: String,
        object: String,
        credential: credential::Credential,
    ) -> Result<(), anyhow::Error>;
    fn set_object_acl(
        &self,
        bucket: String,
        object: String,
        acl: datatype::Acl,
        policy: types::AccessControlPolicy,
        version: String,
        credential: credential::Credential,
    ) -> Result<(), anyhow::Error>;
    fn get_object_acl(
        &self,
        bucket: String,
        object: String,
        version: String,
        credential: credential::Credential,
    ) -> Result<types::AccessControlPolicy, anyhow::Error>;
    fn delete_object(
        &self,
        bucket: String,
        object: String,
        version: String,
        credential: credential::Credential,
    ) -> Result<(), anyhow::Error>;

    // Multipart operations
    fn list_multipart_uploads(
        &self,
        bucket: String,
        credential: credential::Credential,
        request: datatype::ListUploadsRequest,
    ) -> Result<Vec<String>, anyhow::Error>; // TODO: Return new datatype
    fn new_multipart_upload(
        &self,
        bucket: String,
        object: String,
        metadata: types::MetadataEntry,
        acl: datatype::Acl,
        // sse: datatype::SseRequest,
        // storage_class: datatype::StorageClass,
        credential: credential::Credential,
    ) -> Result<String, anyhow::Error>;
    fn put_object_part(
        &self,
        bucket: String,
        object: String,
        upload_id: String,
        part_number: i64,
        reader: &mut dyn std::io::Read,
        md5: String,
        length: i64,
        // sse: datatype::SseRequest,
        credential: credential::Credential,
    ) -> Result<String, anyhow::Error>;
    fn copy_object_part(
        &self,
        bucket: String,
        object: String,
        upload_id: String,
        part_number: i64,
        source: &mut dyn std::io::Read,
        is_metadata_only: bool,
        // sse: datatype::SseRequest,
        credential: credential::Credential,
    ) -> Result<String, anyhow::Error>;
    fn list_object_parts(
        &self,
        bucket: String,
        object: String,
        upload_id: String,
        credential: credential::Credential,
        request: datatype::ListPartsRequest,
    ) -> Result<Vec<String>, anyhow::Error>; // TODO: Return new datatype
    fn complete_multipart_upload(
        &self,
        bucket: String,
        object: String,
        upload_id: String,
        parts: Vec<datatype::Part>,
        credential: credential::Credential,
    ) -> Result<String, anyhow::Error>;
    fn abort_multipart_upload(
        &self,
        bucket: String,
        object: String,
        upload_id: String,
        credential: credential::Credential,
    ) -> Result<(), anyhow::Error>;

    // Freezer operations
    fn get_freezer(
        &self,
        bucket: String,
        object: String,
        version: String,
    ) -> Result<String, anyhow::Error>;
    fn get_freezer_status(
        &self,
        freezer: String,
        object: String,
        version: String,
    ) -> Result<(), anyhow::Error>;
    fn create_freezer(&self, freezer: String) -> Result<(), anyhow::Error>;
    fn update_freezer_date(
        &self,
        freezer: String,
        date: i64,
        is_increment: bool,
    ) -> Result<(), anyhow::Error>;
}
