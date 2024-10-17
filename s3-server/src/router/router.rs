use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

use axum::{body::Bytes, http::Request};
use s3_core::S3Request;

pub struct Router {
    hosts: Vec<String>,
    route_matcher: RouteMatcher,
}

impl Router {
    pub fn new(hosts: Vec<String>) -> Self {
        Router {
            hosts,
            route_matcher: RouteMatcher::new(),
        }
    }

    pub fn match_result(&self, req: &axum::http::Request<axum::body::Bytes>) -> S3Request {
        let req_host = req.uri().host().unwrap_or("").to_string();

        for host in &self.hosts {
            // Trim host from req host
            let prefix = req_host.trim_end_matches(host);

            // Path based routing
            if prefix.is_empty() {
                // bucket is the first part of the path
                let path = req.uri().path();
                let parts = path.split('/').collect::<Vec<&str>>();
                if parts.len() > 1 {
                    let bucket = parts[1];
                    let key = parts[2..].join("/");
                    return self.match_route(host, bucket, &key, req);
                }
            } else {
                // Host based routing
                let bucket = prefix.trim_start_matches('.');
                let key = req.uri().path().trim_start_matches('/');
                return self.match_route(host, bucket, key, req);
            }
        }
        S3Request::Unknown
    }

    fn match_route(
        &self,
        host: &str,
        bucket: &str,
        key: &str,
        req: &axum::http::Request<axum::body::Bytes>,
    ) -> S3Request {
        let method = req.method().as_str();
        let mut matcher = &self.route_matcher.key;
        if key == "" {
            matcher = &self.route_matcher.bucket;
            if bucket == "" {
                matcher = &self.route_matcher.root;
            }
        }

        if let Some(routes) = matcher.get(method) {
            for route in routes {
                if route.match_route(req) {
                    return route.operation.clone();
                }
            }
        }

        S3Request::Unknown
    }
}

struct RouteMatcher {
    // bucket routing -- host style
    bucket: HashMap<String, Vec<Route>>,
    // key routing -- path style
    key: HashMap<String, Vec<Route>>,
    // root routing -- domain style
    root: HashMap<String, Vec<Route>>,
}

impl RouteMatcher {
    pub fn new() -> Self {
        let mut matcher = RouteMatcher {
            bucket: HashMap::new(),
            key: HashMap::new(),
            root: HashMap::new(),
        };

        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Request::CreateSession,
                arguments: vec![has_query("session")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Request::GetBucketAccelerateConfiguration,
                arguments: vec![has_query("accelerate")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Request::GetBucketAcl,
                arguments: vec![has_query("acl")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Request::GetBucketAnalyticsConfiguration,
                arguments: vec![has_query("analytics"), has_query("id")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Request::GetBucketCors,
                arguments: vec![has_query("cors")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Request::GetBucketEncryption,
                arguments: vec![has_query("encryption")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Request::GetBucketInventoryConfiguration,
                arguments: vec![has_query("inventory"), has_query("id")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Request::GetBucketLifecycleConfiguration,
                arguments: vec![has_query("lifecycle")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Request::GetBucketLocation,
                arguments: vec![has_query("location")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Request::GetBucketLogging,
                arguments: vec![has_query("logging")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Request::GetBucketMetricsConfiguration,
                arguments: vec![has_query("metrics"), has_query("id")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Request::GetBucketNotificationConfiguration,
                arguments: vec![has_query("notification")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Request::GetBucketOwnershipControls,
                arguments: vec![has_query("ownershipControls")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Request::GetBucketPolicy,
                arguments: vec![has_query("policy")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Request::GetBucketReplication,
                arguments: vec![has_query("replication")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Request::GetBucketRequestPayment,
                arguments: vec![has_query("requestPayment")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Request::GetBucketTagging,
                arguments: vec![has_query("tagging")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Request::GetBucketVersioning,
                arguments: vec![has_query("versioning")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Request::GetBucketWebsite,
                arguments: vec![has_query("website")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Request::ListBucketAnalyticsConfigurations,
                arguments: vec![has_query("analytics")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Request::ListBucketInventoryConfigurations,
                arguments: vec![has_query("inventory")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Request::ListBucketMetricsConfigurations,
                arguments: vec![has_query("metrics")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Request::ListMultipartUploads,
                arguments: vec![has_query("uploads")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Request::ListObjects,
                arguments: vec![],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Request::ListObjectsV2,
                arguments: vec![has_query_value("list-type", "2")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Request::ListObjectVersions,
                arguments: vec![has_query("versions")],
            },
        );

        matcher.add_root_route(
            "GET",
            Route {
                operation: S3Request::ListBuckets,
                arguments: vec![],
            },
        );

        matcher
    }

    pub fn add_bucket_route(&mut self, method: &str, route: Route) {
        self.bucket
            .entry(method.to_string())
            .or_insert_with(Vec::new)
            .push(route);
    }

    pub fn add_key_route(&mut self, method: &str, route: Route) {
        self.key
            .entry(method.to_string())
            .or_insert_with(Vec::new)
            .push(route);
    }

    pub fn add_root_route(&mut self, method: &str, route: Route) {
        self.root
            .entry(method.to_string())
            .or_insert_with(Vec::new)
            .push(route);
    }
}

struct Route {
    operation: S3Request,
    arguments: Vec<HasArguments>,
}

impl Route {
    pub fn new() -> Self {
        Route {
            operation: S3Request::Unknown,
            arguments: vec![],
        }
    }

    pub fn match_route(&self, req: &axum::http::Request<axum::body::Bytes>) -> bool {
        self.arguments.iter().all(|arg| arg(&req))
    }
}

type HasArguments = Box<dyn Fn(&Request<Bytes>) -> bool + Send + Sync>;

fn has_query(name: &str) -> HasArguments {
    let name = name.to_string();
    Box::new(move |req: &axum::http::Request<axum::body::Bytes>| {
        let query = req.uri().query().unwrap_or("");
        query.contains(&name)
    })
}

fn has_query_value(name: &str, value: &str) -> HasArguments {
    let name = name.to_string();
    let value = value.to_string();
    Box::new(move |req: &axum::http::Request<axum::body::Bytes>| {
        let query = req
            .uri()
            .query()
            .unwrap_or("")
            .split('&')
            .map(|param| {
                let mut parts = param.split('=');
                let key = parts.next().unwrap().to_string();
                let val = parts.next().unwrap_or("").to_string();
                (key, val)
            })
            .collect::<BTreeMap<String, String>>();
        query.get(&name).map(|v| v == &value).unwrap_or(false)
    })
}

fn has_header(name: &str) -> HasArguments {
    let name = name.to_string();
    Box::new(move |req: &axum::http::Request<axum::body::Bytes>| req.headers().get(&name).is_some())
}

fn has_header_value(name: &str, value: &str) -> HasArguments {
    let name = name.to_string();
    let value = value.to_string();
    Box::new(move |req: &axum::http::Request<axum::body::Bytes>| {
        req.headers()
            .get(&name)
            .map(|v| v.to_str().unwrap_or("") == value)
            .unwrap_or(false)
    })
}
