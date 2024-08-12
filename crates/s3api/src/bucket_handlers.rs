use std::sync::Arc;

use actix_web::{web, Responder};

use crate::{api_header, api_response, Credential, LocationResponse, ObjectApiRouter, ObjectLayer};

pub trait BucketHandlers {
    async fn get_bucket_location_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn list_multipart_uploads_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn get_bucket_versioning_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn list_versioned_objects_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn put_bucket_acl_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn get_bucket_acl_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn put_bucket_versioning_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn put_bucket_cors_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn get_bucket_cors_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn delete_bucket_cors_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn put_bucket_logging_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn get_bucket_logging_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn put_bucket_policy_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn get_bucket_policy_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn delete_bucket_policy_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn put_bucket_lifecycle_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn get_bucket_lifecycle_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn delete_bucket_lifecycle_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn get_bucket_website_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn put_bucket_website_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn delete_bucket_website_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn put_bucket_encryption_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn get_bucket_encryption_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn delete_bucket_encryption_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn head_bucket_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn delete_bucket_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn delete_multiple_objects_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn put_bucket_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn list_objects_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn list_buckets_handler(req: actix_web::HttpRequest) -> impl Responder;
}

impl BucketHandlers for ObjectApiRouter {
    async fn get_bucket_location_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        let bucket_name = req.match_info().get("bucket").unwrap();

        let credential = Credential::new();

        // Parse Signature validation

        if let Err(e) = data.get_bucket_info(bucket_name.to_string(), credential) {
            return api_response::write_error_response(req, e);
        }

        api_response::write_success_response(api_header::encode_response(&LocationResponse {
            location: helper::CONFIG.region.clone(),
        }))
    }

    async fn list_multipart_uploads_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn get_bucket_versioning_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn list_versioned_objects_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn put_bucket_acl_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn get_bucket_acl_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn put_bucket_versioning_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn put_bucket_cors_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn get_bucket_cors_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn delete_bucket_cors_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn put_bucket_logging_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn get_bucket_logging_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn put_bucket_policy_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn get_bucket_policy_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn delete_bucket_policy_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn put_bucket_lifecycle_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn get_bucket_lifecycle_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn delete_bucket_lifecycle_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn get_bucket_website_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn put_bucket_website_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn delete_bucket_website_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn put_bucket_encryption_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn get_bucket_encryption_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn delete_bucket_encryption_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn head_bucket_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn delete_bucket_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn delete_multiple_objects_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn put_bucket_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn list_objects_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn list_buckets_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }
}
