use crate::object_interface::ObjectLayer;

pub struct ObjectApiRouter {
    pub object_layer: Box<dyn ObjectLayer>,
}

impl ObjectApiRouter {
    pub fn new(object_layer: Box<dyn ObjectLayer>) -> Self {
        Self { object_layer }
    }

    pub fn register_routes(&self, config: &mut actix_web::web::ServiceConfig) {
        let main_config = config.service(actix_web::web::scope("/"));

        

    }
}
