use std::sync::Arc;

use actix_web::web;

use crate::{ObjectApiRouter, ObjectLayer};

pub trait ObjectHandlers {
    async fn head_object_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn copy_object_part_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn put_object_part_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn list_object_parts_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn complete_multipart_upload_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn new_multipart_upload_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn abort_multipart_upload_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn copy_object_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn restore_object_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn get_object_acl_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn append_object_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn put_object_metadata_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn put_object_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn post_object_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn get_object_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
    async fn delete_object_handler(
        req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse;
}

impl ObjectHandlers for ObjectApiRouter {
    async fn head_object_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        actix_web::HttpResponse::Ok().finish()
    }
    async fn copy_object_part_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn put_object_part_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn list_object_parts_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn complete_multipart_upload_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn new_multipart_upload_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn abort_multipart_upload_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn copy_object_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn restore_object_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn get_object_acl_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn append_object_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn put_object_metadata_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn put_object_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn post_object_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn get_object_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn delete_object_handler(
        _req: actix_web::HttpRequest,
        data: web::Data<Arc<dyn ObjectLayer>>,
    ) -> actix_web::HttpResponse {
        actix_web::HttpResponse::Ok().finish()
    }
}
