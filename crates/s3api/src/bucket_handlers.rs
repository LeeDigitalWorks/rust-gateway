use std::sync::Arc;

use actix_web::{web, Responder};

use crate::{
    api_header, api_response, signature, Credential, LocationResponse, ObjectApiRouter, ObjectLayer,
};

pub trait BucketHandlers {
    async fn get_bucket_location_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
        bytes: web::Bytes,
    ) -> actix_web::HttpResponse;
    async fn list_multipart_uploads_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn get_bucket_versioning_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn list_versioned_objects_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn put_bucket_acl_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn get_bucket_acl_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn put_bucket_versioning_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn put_bucket_cors_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn get_bucket_cors_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn delete_bucket_cors_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn put_bucket_logging_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn get_bucket_logging_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn put_bucket_policy_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn get_bucket_policy_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn delete_bucket_policy_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn put_bucket_lifecycle_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn get_bucket_lifecycle_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn delete_bucket_lifecycle_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn get_bucket_website_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn put_bucket_website_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn delete_bucket_website_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn put_bucket_encryption_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn get_bucket_encryption_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn delete_bucket_encryption_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn head_bucket_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn delete_bucket_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn delete_multiple_objects_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn put_bucket_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn list_objects_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn list_buckets_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
}

impl BucketHandlers for ObjectApiRouter {
    async fn get_bucket_location_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
        bytes: web::Bytes,
    ) -> actix_web::HttpResponse {
        let bucket_name = req.match_info().get("bucket").unwrap();

        let credential = Credential::new();

        // Parse Signature validation
        match signature::get_request_auth_type(&req) {
            signature::AuthType::Anonymous => {
                if let Err(e) = data.get_bucket_info(bucket_name.to_string(), credential) {
                    return api_response::write_error_response(&req, e);
                }

                api_response::write_success_response(api_header::encode_response(
                    &LocationResponse {
                        location: helper::CONFIG.region.clone(),
                    },
                ))
            }
            signature::AuthType::SignedV4
            | signature::AuthType::PresignedV4
            | signature::AuthType::PresignedV2
            | signature::AuthType::SignedV2 => {
                let credential = signature::is_req_authenticated(&req, bytes)
                    .map_err(|e| {
                        api_response::write_error_response(&req, e);
                    })
                    .unwrap();

                if let Err(e) = data.get_bucket_info(bucket_name.to_string(), credential) {
                    return api_response::write_error_response(&req, e);
                }

                api_response::write_success_response(api_header::encode_response(
                    &LocationResponse {
                        location: helper::CONFIG.region.clone(),
                    },
                ))
            }
            _ => api_response::write_error_response(&req, s3err::ApiErrorCode::ErrAccessDenied),
        }
    }

    async fn list_multipart_uploads_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        api_response::write_success_response(Vec::new())
    }

    async fn get_bucket_versioning_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        api_response::write_success_response(Vec::new())
    }

    async fn list_versioned_objects_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        api_response::write_success_response(Vec::new())
    }

    async fn put_bucket_acl_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        api_response::write_success_response(Vec::new())
    }

    async fn get_bucket_acl_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        api_response::write_success_response(Vec::new())
    }

    async fn put_bucket_versioning_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        api_response::write_success_response(Vec::new())
    }

    async fn put_bucket_cors_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        api_response::write_success_response(Vec::new())
    }

    async fn get_bucket_cors_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        api_response::write_success_response(Vec::new())
    }

    async fn delete_bucket_cors_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        api_response::write_success_response(Vec::new())
    }

    async fn put_bucket_logging_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        api_response::write_success_response(Vec::new())
    }

    async fn get_bucket_logging_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        api_response::write_success_response(Vec::new())
    }

    async fn put_bucket_policy_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        api_response::write_success_response(Vec::new())
    }

    async fn get_bucket_policy_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        api_response::write_success_response(Vec::new())
    }

    async fn delete_bucket_policy_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        api_response::write_success_response(Vec::new())
    }

    async fn put_bucket_lifecycle_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        api_response::write_success_response(Vec::new())
    }

    async fn get_bucket_lifecycle_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        api_response::write_success_response(Vec::new())
    }

    async fn delete_bucket_lifecycle_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        api_response::write_success_response(Vec::new())
    }

    async fn get_bucket_website_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        api_response::write_success_response(Vec::new())
    }

    async fn put_bucket_website_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        api_response::write_success_response(Vec::new())
    }

    async fn delete_bucket_website_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        api_response::write_success_response(Vec::new())
    }

    async fn put_bucket_encryption_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        api_response::write_success_response(Vec::new())
    }

    async fn get_bucket_encryption_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        api_response::write_success_response(Vec::new())
    }

    async fn delete_bucket_encryption_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        api_response::write_success_response(Vec::new())
    }

    async fn head_bucket_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        api_response::write_success_response(Vec::new())
    }

    async fn delete_bucket_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        api_response::write_success_response(Vec::new())
    }

    async fn delete_multiple_objects_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        api_response::write_success_response(Vec::new())
    }

    async fn put_bucket_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        api_response::write_success_response(Vec::new())
    }

    async fn list_objects_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        api_response::write_success_response(Vec::new())
    }

    async fn list_buckets_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        api_response::write_success_response(Vec::new())
    }
}
