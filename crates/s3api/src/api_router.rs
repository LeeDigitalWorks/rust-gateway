use crate::object_interface::ObjectLayer;

pub struct ObjectApiHandlers {
    pub object_layer: Box<dyn ObjectLayer>,
}

impl ObjectApiHandlers {
    pub fn new(object_layer: Box<dyn ObjectLayer>) -> Self {
        Self { object_layer }
    }

    pub fn register_routes(&self, config: &mut actix_web::web::ServiceConfig) {}
}
