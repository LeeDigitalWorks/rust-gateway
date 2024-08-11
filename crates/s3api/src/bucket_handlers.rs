use actix_web::Responder;

use crate::ObjectApiRouter;

pub trait BucketHandlers {
    async fn get_bucket_location_handler(req: actix_web::HttpRequest) -> impl Responder;
    async fn list_multipart_uploads_handler(req: actix_web::HttpRequest) -> impl Responder;
}

impl BucketHandlers for ObjectApiRouter {
    async fn get_bucket_location_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }
    async fn list_multipart_uploads_handler(_req: actix_web::HttpRequest) -> impl Responder {
        actix_web::HttpResponse::Ok().finish()
    }
}
