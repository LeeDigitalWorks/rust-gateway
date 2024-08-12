use actix_web::guard::GuardContext;
use regex::Regex;

use crate::{object_interface::ObjectLayer, BucketHandlers, ObjectHandlers};

pub struct ObjectApiRouter {
    pub object_layer: Box<dyn ObjectLayer>,
}

fn queries(key: &'static str, value: &'static str) -> impl Fn(&GuardContext) -> bool {
    let re = Regex::new(value).unwrap();
    move |ctx| {
        ctx.head().uri.query().map_or(false, |q| {
            let query = q.split('&').find(|q| q.starts_with(key));
            query.map_or(false, |q| re.is_match(q))
        })
    }
}

fn headers_regexp(key: &'static str, value: &'static str) -> impl Fn(&GuardContext) -> bool {
    let re = Regex::new(value).unwrap();
    move |ctx| {
        ctx.head().headers.contains_key(key)
            && re.is_match(ctx.head().headers().get(key).unwrap().to_str().unwrap())
    }
}

fn configure_scope(scope: actix_web::Scope) -> actix_web::Scope {
    let mut scope = scope;
    // Object Operations
    // HeadObject
    scope = scope.guard(actix_web::guard::Head()).route(
        "/{object:.+}",
        actix_web::web::head().to(ObjectApiRouter::head_object_handler),
    );
    // PutObjectPart - Copy
    scope = scope
        .guard(actix_web::guard::Put())
        .guard(actix_web::guard::fn_guard(queries(
            "partNumber",
            "{partNumber:[0-9]+}",
        )))
        .guard(actix_web::guard::fn_guard(queries(
            "uploadId",
            "{uploadId:.+}",
        )))
        .guard(actix_web::guard::fn_guard(headers_regexp(
            "X-Amz-Copy-Source",
            ".*?(/).*?",
        )))
        .route(
            "/{object:.+}",
            actix_web::web::put().to(ObjectApiRouter::copy_object_part_handler),
        );
    // PutObjectPart
    scope = scope
        .guard(actix_web::guard::Put())
        .guard(actix_web::guard::fn_guard(queries(
            "partNumber",
            "{partNumber:[0-9]+}",
        )))
        .guard(actix_web::guard::fn_guard(queries(
            "uploadId",
            "{uploadId:.+}",
        )))
        .route(
            "/{object:.+}",
            actix_web::web::put().to(ObjectApiRouter::put_object_part_handler),
        );
    // ListObjectParts
    scope = scope
        .guard(actix_web::guard::Get())
        .guard(actix_web::guard::fn_guard(queries(
            "uploadId",
            "{uploadId:.+}",
        )))
        .route(
            "/{object:.+}",
            actix_web::web::get().to(ObjectApiRouter::list_object_parts_handler),
        );
    // CompleteMultipartUpload
    scope = scope
        .guard(actix_web::guard::Post())
        .guard(actix_web::guard::fn_guard(queries(
            "uploadId",
            "{uploadId:.+}",
        )))
        .route(
            "/{object:.+}",
            actix_web::web::post().to(ObjectApiRouter::complete_multipart_upload_handler),
        );
    // NewMultipartUpload
    scope = scope.guard(actix_web::guard::Post()).route(
        "/{object:.+}",
        actix_web::web::post().to(ObjectApiRouter::new_multipart_upload_handler),
    );
    // AbortMultipartUpload
    scope = scope
        .guard(actix_web::guard::Delete())
        .guard(actix_web::guard::fn_guard(queries(
            "uploadId",
            "{uploadId:.+}",
        )))
        .route(
            "/{object:.+}",
            actix_web::web::delete().to(ObjectApiRouter::abort_multipart_upload_handler),
        );
    // CopyObject
    scope = scope
        .guard(actix_web::guard::Put())
        .guard(actix_web::guard::fn_guard(headers_regexp(
            "X-Amz-Copy-Source",
            ".*?(/).*?",
        )))
        .route(
            "/{object:.+}",
            actix_web::web::put().to(ObjectApiRouter::copy_object_handler),
        );
    // RenameObject
    scope = scope
        .guard(actix_web::guard::Put())
        .guard(actix_web::guard::fn_guard(headers_regexp(
            "X-Amz-Copy-Source",
            ".*?(/).*?",
        )))
        .route(
            "/{object:.+}",
            actix_web::web::put().to(ObjectApiRouter::copy_object_handler),
        );
    // RestoreObject
    scope = scope
        .guard(actix_web::guard::Post())
        .guard(actix_web::guard::fn_guard(queries("restore", "")))
        .route(
            "/{object:.+}",
            actix_web::web::post().to(ObjectApiRouter::restore_object_handler),
        );
    // GetObjectAcl
    scope = scope
        .guard(actix_web::guard::Get())
        .guard(actix_web::guard::fn_guard(queries("acl", "")))
        .route(
            "/{object:.+}",
            actix_web::web::get().to(ObjectApiRouter::get_object_acl_handler),
        );
    // AppendObject
    scope = scope
        .guard(actix_web::guard::Post())
        .guard(actix_web::guard::fn_guard(queries("append", "")))
        .route(
            "/{object:.+}",
            actix_web::web::post().to(ObjectApiRouter::append_object_handler),
        );
    // PutObjectMetadata
    scope = scope
        .guard(actix_web::guard::Put())
        .guard(actix_web::guard::fn_guard(queries("meta", "")))
        .route(
            "/{object:.+}",
            actix_web::web::put().to(ObjectApiRouter::put_object_metadata_handler),
        );
    // PutObject
    scope = scope.guard(actix_web::guard::Put()).route(
        "/{object:.+}",
        actix_web::web::put().to(ObjectApiRouter::put_object_handler),
    );
    // PostObject
    scope = scope
        .guard(actix_web::guard::Post())
        .guard(actix_web::guard::fn_guard(headers_regexp(
            "Content-Type",
            "multipart/form-data*",
        )))
        .route(
            "/",
            actix_web::web::post().to(ObjectApiRouter::post_object_handler),
        );
    // GetObject
    scope = scope.guard(actix_web::guard::Get()).route(
        "/{object:.+}",
        actix_web::web::get().to(ObjectApiRouter::get_object_handler),
    );
    // DeleteObject
    scope = scope.guard(actix_web::guard::Delete()).route(
        "/{object:.+}",
        actix_web::web::delete().to(ObjectApiRouter::delete_object_handler),
    );

    // Bucket Operations
    // GetBucketLocation
    scope = scope
        .guard(actix_web::guard::Get())
        .guard(
            actix_web::guard::fn_guard(queries("location", "")), // location query
        )
        .route(
            "/",
            actix_web::web::get().to(ObjectApiRouter::get_bucket_location_handler),
        );
    // ListMultipartUploads
    scope = scope
        .guard(actix_web::guard::Get())
        .guard(actix_web::guard::fn_guard(queries("uploads", "")))
        .route(
            "/",
            actix_web::web::get().to(ObjectApiRouter::list_multipart_uploads_handler),
        );
    // GetBucketVersioning
    scope = scope
        .guard(actix_web::guard::Get())
        .guard(actix_web::guard::fn_guard(queries("versioning", "")))
        .route(
            "/",
            actix_web::web::get().to(ObjectApiRouter::get_bucket_versioning_handler),
        );
    // ListVersionedObjects
    scope = scope
        .guard(actix_web::guard::Get())
        .guard(actix_web::guard::fn_guard(queries("versions", "")))
        .route(
            "/",
            actix_web::web::get().to(ObjectApiRouter::list_versioned_objects_handler),
        );
    // PutBucketAcl
    scope = scope
        .guard(actix_web::guard::Put())
        .guard(actix_web::guard::fn_guard(queries("acl", "")))
        .route(
            "/",
            actix_web::web::put().to(ObjectApiRouter::put_bucket_acl_handler),
        );
    // GetBucketAcl
    scope = scope
        .guard(actix_web::guard::Get())
        .guard(actix_web::guard::fn_guard(queries("acl", "")))
        .route(
            "/",
            actix_web::web::get().to(ObjectApiRouter::get_bucket_acl_handler),
        );
    // PutBucketVersioning
    scope = scope
        .guard(actix_web::guard::Put())
        .guard(actix_web::guard::fn_guard(queries("versioning", "")))
        .route(
            "/",
            actix_web::web::put().to(ObjectApiRouter::put_bucket_versioning_handler),
        );
    // PutBucketCors
    scope = scope
        .guard(actix_web::guard::Put())
        .guard(actix_web::guard::fn_guard(queries("cors", "")))
        .route(
            "/",
            actix_web::web::put().to(ObjectApiRouter::put_bucket_cors_handler),
        );
    // GetBucketCors
    scope = scope
        .guard(actix_web::guard::Get())
        .guard(actix_web::guard::fn_guard(queries("cors", "")))
        .route(
            "/",
            actix_web::web::get().to(ObjectApiRouter::get_bucket_cors_handler),
        );
    // DeleteBucketCors
    scope = scope
        .guard(actix_web::guard::Delete())
        .guard(actix_web::guard::fn_guard(queries("cors", "")))
        .route(
            "/",
            actix_web::web::delete().to(ObjectApiRouter::delete_bucket_cors_handler),
        );
    // PutBucketLogging
    scope = scope
        .guard(actix_web::guard::Put())
        .guard(actix_web::guard::fn_guard(queries("logging", "")))
        .route(
            "/",
            actix_web::web::put().to(ObjectApiRouter::put_bucket_logging_handler),
        );
    // GetBucketLogging
    scope = scope
        .guard(actix_web::guard::Get())
        .guard(actix_web::guard::fn_guard(queries("logging", "")))
        .route(
            "/",
            actix_web::web::get().to(ObjectApiRouter::get_bucket_logging_handler),
        );
    // PutBucketPolicy
    scope = scope
        .guard(actix_web::guard::Put())
        .guard(actix_web::guard::fn_guard(queries("policy", "")))
        .route(
            "/",
            actix_web::web::put().to(ObjectApiRouter::put_bucket_policy_handler),
        );
    // GetBucketPolicy
    scope = scope
        .guard(actix_web::guard::Get())
        .guard(actix_web::guard::fn_guard(queries("policy", "")))
        .route(
            "/",
            actix_web::web::get().to(ObjectApiRouter::get_bucket_policy_handler),
        );
    // DeleteBucketPolicy
    scope = scope
        .guard(actix_web::guard::Delete())
        .guard(actix_web::guard::fn_guard(queries("policy", "")))
        .route(
            "/",
            actix_web::web::delete().to(ObjectApiRouter::delete_bucket_policy_handler),
        );
    // PutBucketLifecycle
    scope = scope
        .guard(actix_web::guard::Put())
        .guard(actix_web::guard::fn_guard(queries("lifecycle", "")))
        .route(
            "/",
            actix_web::web::put().to(ObjectApiRouter::put_bucket_lifecycle_handler),
        );
    // GetBucketLifecycle
    scope = scope
        .guard(actix_web::guard::Get())
        .guard(actix_web::guard::fn_guard(queries("lifecycle", "")))
        .route(
            "/",
            actix_web::web::get().to(ObjectApiRouter::get_bucket_lifecycle_handler),
        );
    // DeleteBucketLifecycle
    scope = scope
        .guard(actix_web::guard::Delete())
        .guard(actix_web::guard::fn_guard(queries("lifecycle", "")))
        .route(
            "/",
            actix_web::web::delete().to(ObjectApiRouter::delete_bucket_lifecycle_handler),
        );
    // GetBucketWebsite
    scope = scope
        .guard(actix_web::guard::Get())
        .guard(actix_web::guard::fn_guard(queries("website", "")))
        .route(
            "/",
            actix_web::web::get().to(ObjectApiRouter::get_bucket_website_handler),
        );
    // PutBucketWebsite
    scope = scope
        .guard(actix_web::guard::Put())
        .guard(actix_web::guard::fn_guard(queries("website", "")))
        .route(
            "/",
            actix_web::web::put().to(ObjectApiRouter::put_bucket_website_handler),
        );
    // DeleteBucketWebsite
    scope = scope
        .guard(actix_web::guard::Delete())
        .guard(actix_web::guard::fn_guard(queries("website", "")))
        .route(
            "/",
            actix_web::web::delete().to(ObjectApiRouter::delete_bucket_website_handler),
        );
    // PutBucketEncryption
    scope = scope
        .guard(actix_web::guard::Put())
        .guard(actix_web::guard::fn_guard(queries("encryption", "")))
        .route(
            "/",
            actix_web::web::put().to(ObjectApiRouter::put_bucket_encryption_handler),
        );
    // GetBucketEncryption
    scope = scope
        .guard(actix_web::guard::Get())
        .guard(actix_web::guard::fn_guard(queries("encryption", "")))
        .route(
            "/",
            actix_web::web::get().to(ObjectApiRouter::get_bucket_encryption_handler),
        );
    // DeleteBucketEncryption
    scope = scope
        .guard(actix_web::guard::Delete())
        .guard(actix_web::guard::fn_guard(queries("encryption", "")))
        .route(
            "/",
            actix_web::web::delete().to(ObjectApiRouter::delete_bucket_encryption_handler),
        );
    // HeadBucket
    scope = scope.guard(actix_web::guard::Head()).route(
        "/",
        actix_web::web::head().to(ObjectApiRouter::head_bucket_handler),
    );
    // DeleteBucket
    scope = scope.guard(actix_web::guard::Delete()).route(
        "/",
        actix_web::web::delete().to(ObjectApiRouter::delete_bucket_handler),
    );
    // PutBucket
    scope = scope.guard(actix_web::guard::Put()).route(
        "/",
        actix_web::web::put().to(ObjectApiRouter::put_bucket_handler),
    );
    // ListObjects
    scope = scope.guard(actix_web::guard::Get()).route(
        "/",
        actix_web::web::get().to(ObjectApiRouter::list_objects_handler),
    );
    // DeleteMultipleObjects
    scope = scope
        .guard(actix_web::guard::Post())
        .guard(actix_web::guard::fn_guard(queries("delete", "")))
        .route(
            "/",
            actix_web::web::post().to(ObjectApiRouter::delete_multiple_objects_handler),
        );

    scope
}

impl ObjectApiRouter {
    pub fn new(object_layer: Box<dyn ObjectLayer>) -> Self {
        Self { object_layer }
    }

    pub fn register_routes(&self, config: &mut actix_web::web::ServiceConfig) {
        // add list buckets at the root
        config.service(
            actix_web::web::scope("/")
                .guard(actix_web::guard::Get())
                .route(
                    "/",
                    actix_web::web::get().to(ObjectApiRouter::list_buckets_handler),
                ),
        );

        helper::CONFIG.s3domain.iter().for_each(|domain| {
            // add a router for each domain - vhost and path style
            // path style -- domain.name/bucket_name/object_name
            let mut path_config =
                actix_web::web::scope("/{bucket}").guard(actix_web::guard::Host(domain));
            // vhost style -- bucket_name.domain.name/object_name
            let mut vhost_config = actix_web::web::scope("/")
                .guard(actix_web::guard::Host(format!("{{bucket}}.{}", domain)));

            path_config = configure_scope(path_config);
            vhost_config = configure_scope(vhost_config);

            config.service(path_config);
            config.service(vhost_config);
        });
    }
}
