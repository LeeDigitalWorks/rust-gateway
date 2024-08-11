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

    scope
}

impl ObjectApiRouter {
    pub fn new(object_layer: Box<dyn ObjectLayer>) -> Self {
        Self { object_layer }
    }

    pub fn register_routes(&self, config: &mut actix_web::web::ServiceConfig) {
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
