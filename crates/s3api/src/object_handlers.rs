use actix_web::Responder;

use crate::ObjectApiRouter;

pub trait ObjectHandlers {
    async fn head_object_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn copy_object_part_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn put_object_part_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn list_object_parts_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn complete_multipart_upload_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn new_multipart_upload_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn abort_multipart_upload_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn copy_object_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn restore_object_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn get_object_acl_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn append_object_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn put_object_metadata_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn put_object_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn post_object_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn get_object_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn delete_object_handler(req: actix_web::HttpRequest) -> impl Responder;
}

impl ObjectHandlers for ObjectApiRouter {
    async fn head_object_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }
    async fn copy_object_part_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn put_object_part_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn list_object_parts_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn complete_multipart_upload_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn new_multipart_upload_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn abort_multipart_upload_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn copy_object_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn restore_object_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn get_object_acl_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn append_object_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn put_object_metadata_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn put_object_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn post_object_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn get_object_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }

    async fn delete_object_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }
}
