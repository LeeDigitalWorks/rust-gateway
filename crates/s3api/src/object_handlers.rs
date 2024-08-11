use actix_web::Responder;

use crate::ObjectApiRouter;

pub trait ObjectHandlers {
    async fn head_object_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn copy_object_part_handler(req: actix_web::HttpRequest) -> impl Responder;
}

impl ObjectHandlers for ObjectApiRouter {
    async fn head_object_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }
    async fn copy_object_part_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }
}
