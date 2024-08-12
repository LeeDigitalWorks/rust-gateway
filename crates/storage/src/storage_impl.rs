use crate::Storage;
use aws_sdk_s3::types;

impl s3api::ObjectLayer for Storage {
    fn make_bucket(
        &self,
        bucket: String,
        acl: s3api::Acl,
        credential: s3api::Credential,
    ) -> Result<(), s3err::ApiErrorType> {
        todo!()
    }

    fn set_bucket_logging(
        &self,
        bucket: String,
        config: types::BucketLoggingStatus,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }

    fn get_bucket_logging(
        &self,
        bucket: String,
    ) -> Result<types::BucketLoggingStatus, anyhow::Error> {
        todo!()
    }

    fn set_bucket_lifecycle(
        &self,
        bucket: String,
        config: types::BucketLifecycleConfiguration,
        credential: s3api::Credential,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }

    fn get_bucket_lifecycle(
        &self,
        bucket: String,
        credential: s3api::Credential,
    ) -> Result<types::BucketLifecycleConfiguration, anyhow::Error> {
        todo!()
    }

    fn delete_bucket_lifecycle(
        &self,
        bucket: String,
        credential: s3api::Credential,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }

    fn set_bucket_acl(
        &self,
        bucket: String,
        policy: types::AccessControlPolicy,
        acl: s3api::Acl,
        credential: s3api::Credential,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }

    fn get_bucket_acl(
        &self,
        bucket: String,
        credential: s3api::Credential,
    ) -> Result<types::AccessControlPolicy, anyhow::Error> {
        todo!()
    }

    fn set_bucket_cors(
        &self,
        bucket: String,
        cors: types::CorsConfiguration,
        credential: s3api::Credential,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }

    fn set_bucket_versioning(
        &self,
        bucket: String,
        versioning: types::BucketVersioningStatus,
        credential: s3api::Credential,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }

    fn delete_bucket_cors(
        &self,
        bucket: String,
        credential: s3api::Credential,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }

    fn get_bucket_versioning(
        &self,
        bucket: String,
        credential: s3api::Credential,
    ) -> Result<types::BucketVersioningStatus, anyhow::Error> {
        todo!()
    }

    fn get_bucket_cors(
        &self,
        bucket: String,
        credential: s3api::Credential,
    ) -> Result<types::CorsConfiguration, anyhow::Error> {
        todo!()
    }

    fn get_bucket(
        &self,
        bucket: String,
        credential: s3api::Credential,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }

    fn get_bucket_info(
        &self,
        bucket: String,
        credential: s3api::Credential,
    ) -> Result<types::Bucket, s3err::ApiErrorType> {
        todo!()
    }

    fn list_buckets(&self, credential: s3api::Credential) -> Result<Vec<String>, anyhow::Error> {
        todo!()
    }

    fn delete_bucket(
        &self,
        bucket: String,
        credential: s3api::Credential,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }

    fn list_objects(
        &self,
        bucket: String,
        credential: s3api::Credential,
        request: s3api::ListObjectsRequest,
    ) -> Result<Vec<String>, anyhow::Error> {
        todo!()
    }

    fn list_versioned_objects(
        &self,
        bucket: String,
        credential: s3api::Credential,
        request: s3api::ListObjectsRequest,
    ) -> Result<Vec<String>, anyhow::Error> {
        todo!()
    }

    fn set_bucket_policy(
        &self,
        bucket: String,
        policy: String, // TODO: Change to new datatype
        credential: s3api::Credential,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }

    fn get_bucket_policy(
        &self,
        bucket: String,
        credential: s3api::Credential,
    ) -> Result<String, anyhow::Error> {
        todo!()
    }

    fn delete_bucket_policy(
        &self,
        bucket: String,
        credential: s3api::Credential,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }

    fn set_bucket_website(
        &self,
        bucket: String,
        website: types::WebsiteConfiguration,
        credential: s3api::Credential,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }

    fn get_bucket_website(
        &self,
        bucket: String,
        credential: s3api::Credential,
    ) -> Result<types::WebsiteConfiguration, anyhow::Error> {
        todo!()
    }

    fn delete_bucket_website(
        &self,
        bucket: String,
        credential: s3api::Credential,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }

    fn set_bucket_encryption(
        &self,
        bucket: String,
        encryption: types::ServerSideEncryptionConfiguration,
        credential: s3api::Credential,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }

    fn get_bucket_encryption(
        &self,
        bucket: String,
        credential: s3api::Credential,
    ) -> Result<types::ServerSideEncryptionConfiguration, anyhow::Error> {
        todo!()
    }

    fn delete_bucket_encryption(
        &self,
        bucket: String,
        credential: s3api::Credential,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }

    fn get_object(
        &self,
        bucket: String,
        object: String,
        start_offset: i64,
        length: i64,
        writer: &mut dyn std::io::Write,
        credential: s3api::Credential,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }

    fn get_object_info(
        &self,
        bucket: String,
        object: String,
        credential: s3api::Credential,
    ) -> Result<types::Object, anyhow::Error> {
        todo!()
    }

    fn put_object(
        &self,
        bucket: String,
        object: String,
        reader: &mut dyn std::io::Read,
        length: i64,
        metadata: types::MetadataEntry,
        acl: s3api::Acl,
        // sse: s3api::SseRequest,
        // storage_class: s3api::StorageClass,
        credential: s3api::Credential,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }

    fn append_object(
        &self,
        bucket: String,
        object: String,
        reader: &mut dyn std::io::Read,
        length: i64,
        offset: i64,
        metadata: types::MetadataEntry,
        acl: s3api::Acl,
        // sse: s3api::SseRequest,
        // storage_class: s3api::StorageClass,
        credential: s3api::Credential,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }

    fn copy_object(
        &self,
        source_object: String,
        target_object: String,
        source: &mut dyn std::io::Read,
        is_metadata_only: bool,
        // sse: s3api::SseRequest,
        credential: s3api::Credential,
    ) -> Result<s3api::CopyObjectResponse, anyhow::Error> {
        todo!()
    }

    fn rename_object(
        &self,
        source_object: String,
        target_object: String,
        credential: s3api::Credential,
    ) -> Result<s3api::RenameObjectResponse, anyhow::Error> {
        todo!()
    }

    fn put_object_meta(
        &self,
        bucket: String,
        object: String,
        credential: s3api::Credential,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }

    fn set_object_acl(
        &self,
        bucket: String,
        object: String,
        acl: s3api::Acl,
        policy: types::AccessControlPolicy,
        version: String,
        credential: s3api::Credential,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }

    fn get_object_acl(
        &self,
        bucket: String,
        object: String,
        version: String,
        credential: s3api::Credential,
    ) -> Result<types::AccessControlPolicy, anyhow::Error> {
        todo!()
    }

    fn delete_object(
        &self,
        bucket: String,
        object: String,
        version: String,
        credential: s3api::Credential,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }

    fn list_multipart_uploads(
        &self,
        bucket: String,
        credential: s3api::Credential,
        request: s3api::ListUploadsRequest,
    ) -> Result<Vec<String>, anyhow::Error> {
        todo!()
    }

    fn new_multipart_upload(
        &self,
        bucket: String,
        object: String,
        metadata: types::MetadataEntry,
        acl: s3api::Acl,
        // sse: s3api::SseRequest,
        // storage_class: s3api::StorageClass,
        credential: s3api::Credential,
    ) -> Result<String, anyhow::Error> {
        todo!()
    }

    fn put_object_part(
        &self,
        bucket: String,
        object: String,
        upload_id: String,
        part_number: i64,
        reader: &mut dyn std::io::Read,
        md5: String,
        length: i64,
        // sse: s3api::SseRequest,
        credential: s3api::Credential,
    ) -> Result<String, anyhow::Error> {
        todo!()
    }

    fn copy_object_part(
        &self,
        bucket: String,
        object: String,
        upload_id: String,
        part_number: i64,
        source: &mut dyn std::io::Read,
        is_metadata_only: bool,
        // sse: s3api::SseRequest,
        credential: s3api::Credential,
    ) -> Result<String, anyhow::Error> {
        todo!()
    }

    fn list_object_parts(
        &self,
        bucket: String,
        object: String,
        upload_id: String,
        credential: s3api::Credential,
        request: s3api::ListPartsRequest,
    ) -> Result<Vec<String>, anyhow::Error> {
        todo!()
    }

    fn complete_multipart_upload(
        &self,
        bucket: String,
        object: String,
        upload_id: String,
        parts: Vec<s3api::Part>,
        credential: s3api::Credential,
    ) -> Result<String, anyhow::Error> {
        todo!()
    }

    fn abort_multipart_upload(
        &self,
        bucket: String,
        object: String,
        upload_id: String,
        credential: s3api::Credential,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }

    fn get_freezer(
        &self,
        bucket: String,
        object: String,
        version: String,
    ) -> Result<String, anyhow::Error> {
        todo!()
    }

    fn get_freezer_status(
        &self,
        freezer: String,
        object: String,
        version: String,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }

    fn create_freezer(&self, freezer: String) -> Result<(), anyhow::Error> {
        todo!()
    }

    fn update_freezer_date(
        &self,
        freezer: String,
        date: i64,
        is_increment: bool,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }
}
