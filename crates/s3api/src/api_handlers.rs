use crate::ObjectApiRouter;

pub trait ObjectApiHandlers {
    fn get_bucket_location_handler(req: actix_web::HttpRequest) -> actix_web::HttpResponse;
}

impl ObjectApiHandlers for ObjectApiRouter {
    fn get_bucket_location_handler(req: actix_web::HttpRequest) -> actix_web::HttpResponse {
        todo!()
    }
}