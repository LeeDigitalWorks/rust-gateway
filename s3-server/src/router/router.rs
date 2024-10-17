use std::collections::{BTreeMap, HashMap};

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
        matcher.add_bucket_route(
            "HEAD",
            Route {
                operation: S3Request::HeadBucket,
                arguments: vec![],
            },
        );
        matcher.add_bucket_route(
            "OPTIONS",
            Route {
                operation: S3Request::OptionsPreflight,
                arguments: vec![
                    has_header("Access-Control-Request-Method"),
                    has_header("Origin"),
                ],
            },
        );
        matcher.add_bucket_route(
            "DELETE",
            Route {
                operation: S3Request::DeleteBucketAnalyticsConfiguration,
                arguments: vec![has_query("analytics"), has_query("id")],
            },
        );
        matcher.add_bucket_route(
            "DELETE",
            Route {
                operation: S3Request::DeleteBucketCors,
                arguments: vec![has_query("cors")],
            },
        );
        matcher.add_bucket_route(
            "DELETE",
            Route {
                operation: S3Request::DeleteBucketEncryption,
                arguments: vec![has_query("encryption")],
            },
        );
        matcher.add_bucket_route(
            "DELETE",
            Route {
                operation: S3Request::DeleteBucketInventoryConfiguration,
                arguments: vec![has_query("inventory"), has_query("id")],
            },
        );
        matcher.add_bucket_route(
            "DELETE",
            Route {
                operation: S3Request::DeleteBucketLifecycle,
                arguments: vec![has_query("lifecycle")],
            },
        );
        matcher.add_bucket_route(
            "DELETE",
            Route {
                operation: S3Request::DeleteBucketMetricsConfiguration,
                arguments: vec![has_query("metrics"), has_query("id")],
            },
        );
        matcher.add_bucket_route(
            "DELETE",
            Route {
                operation: S3Request::DeleteBucketOwnershipControls,
                arguments: vec![has_query("ownershipControls")],
            },
        );
        matcher.add_bucket_route(
            "DELETE",
            Route {
                operation: S3Request::DeleteBucketPolicy,
                arguments: vec![has_query("policy")],
            },
        );
        matcher.add_bucket_route(
            "DELETE",
            Route {
                operation: S3Request::DeleteBucketReplication,
                arguments: vec![has_query("replication")],
            },
        );
        matcher.add_bucket_route(
            "DELETE",
            Route {
                operation: S3Request::DeleteBucketTagging,
                arguments: vec![has_query("tagging")],
            },
        );
        matcher.add_bucket_route(
            "DELETE",
            Route {
                operation: S3Request::DeleteBucketWebsite,
                arguments: vec![has_query("website")],
            },
        );
        matcher.add_bucket_route(
            "DELETE",
            Route {
                operation: S3Request::DeleteBucketIntelligentTieringConfiguration,
                arguments: vec![has_query("intelligent-tiering"), has_query("id")],
            },
        );
        matcher.add_bucket_route(
            "DELETE",
            Route {
                operation: S3Request::DeletePublicAccessBlock,
                arguments: vec![has_query("publicAccessBlock")],
            },
        );
        matcher.add_bucket_route(
            "DELETE",
            Route {
                operation: S3Request::DeleteBucket,
                arguments: vec![],
            },
        );
        matcher.add_bucket_route(
            "POST",
            Route {
                operation: S3Request::DeleteObjects,
                arguments: vec![has_query("delete")],
            },
        );
        matcher.add_bucket_route(
            "POST",
            Route {
                operation: S3Request::PostObject,
                arguments: vec![has_header_value("Content-Type", "multipart/form-data")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Request::PutBucketAccelerateConfiguration,
                arguments: vec![has_query("accelerate")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Request::PutBucketAcl,
                arguments: vec![has_query("acl")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Request::PutBucketAnalyticsConfiguration,
                arguments: vec![has_query("analytics"), has_query("id")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Request::PutBucketCors,
                arguments: vec![has_query("cors")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Request::PutBucketEncryption,
                arguments: vec![has_query("encryption")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Request::PutBucketInventoryConfiguration,
                arguments: vec![has_query("inventory"), has_query("id")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Request::PutBucketLifecycleConfiguration,
                arguments: vec![has_query("lifecycle")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Request::PutBucketLogging,
                arguments: vec![has_query("logging")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Request::PutBucketMetricsConfiguration,
                arguments: vec![has_query("metrics"), has_query("id")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Request::PutBucketNotificationConfiguration,
                arguments: vec![has_query("notification")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Request::PutBucketOwnershipControls,
                arguments: vec![has_query("ownershipControls")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Request::PutBucketPolicy,
                arguments: vec![has_query("policy")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Request::PutBucketReplication,
                arguments: vec![has_query("replication")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Request::PutBucketRequestPayment,
                arguments: vec![has_query("requestPayment")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Request::PutBucketTagging,
                arguments: vec![has_query("tagging")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Request::PutBucketVersioning,
                arguments: vec![has_query("versioning")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Request::PutBucketWebsite,
                arguments: vec![has_query("website")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Request::PutBucketIntelligentTieringConfiguration,
                arguments: vec![has_query("intelligent-tiering"), has_query("id")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Request::PutPublicAccessBlock,
                arguments: vec![has_query("publicAccessBlock")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Request::CreateBucket,
                arguments: vec![],
            },
        );

        matcher.add_key_route(
            "GET",
            Route {
                operation: S3Request::GetObject,
                arguments: vec![],
            },
        );
        matcher.add_key_route(
            "GET",
            Route {
                operation: S3Request::GetObjectAcl,
                arguments: vec![has_query("acl")],
            },
        );
        matcher.add_key_route(
            "GET",
            Route {
                operation: S3Request::GetObjectAttributes,
                arguments: vec![has_query("attributes")],
            },
        );
        matcher.add_key_route(
            "GET",
            Route {
                operation: S3Request::GetObjectLegalHold,
                arguments: vec![has_query("legal-hold")],
            },
        );
        matcher.add_key_route(
            "GET",
            Route {
                operation: S3Request::GetObjectLockConfiguration,
                arguments: vec![has_query("lock")],
            },
        );
        matcher.add_key_route(
            "GET",
            Route {
                operation: S3Request::GetObjectRetention,
                arguments: vec![has_query("retention")],
            },
        );
        matcher.add_key_route(
            "GET",
            Route {
                operation: S3Request::GetObjectTagging,
                arguments: vec![has_query("tagging")],
            },
        );
        matcher.add_key_route(
            "GET",
            Route {
                operation: S3Request::GetObjectTorrent,
                arguments: vec![has_query("torrent")],
            },
        );
        matcher.add_key_route(
            "GET",
            Route {
                operation: S3Request::ListParts,
                arguments: vec![has_query("uploadId")],
            },
        );
        matcher.add_key_route(
            "HEAD",
            Route {
                operation: S3Request::HeadObject,
                arguments: vec![],
            },
        );
        matcher.add_key_route(
            "DELETE",
            Route {
                operation: S3Request::DeleteObject,
                arguments: vec![],
            },
        );
        matcher.add_key_route(
            "DELETE",
            Route {
                operation: S3Request::DeleteObjectTagging,
                arguments: vec![has_query("tagging")],
            },
        );
        matcher.add_key_route(
            "DELETE",
            Route {
                operation: S3Request::AbortMultipartUpload,
                arguments: vec![has_query("uploadId")],
            },
        );
        matcher.add_key_route(
            "POST",
            Route {
                operation: S3Request::PostObject,
                arguments: vec![has_header_value("Content-Type", "multipart/form-data")],
            },
        );
        matcher.add_key_route(
            "POST",
            Route {
                operation: S3Request::CompleteMultipartUpload,
                arguments: vec![has_query("uploadId")],
            },
        );
        matcher.add_key_route(
            "POST",
            Route {
                operation: S3Request::RestoreObject,
                arguments: vec![has_query("restore")],
            },
        );
        matcher.add_key_route(
            "POST",
            Route {
                operation: S3Request::CreateMultipartUpload,
                arguments: vec![has_query("uploads")],
            },
        );
        matcher.add_key_route(
            "POST",
            Route {
                operation: S3Request::SelectObjectContent,
                arguments: vec![has_query("select"), has_query_value("select-type", "2")],
            },
        );
        matcher.add_key_route(
            "PUT",
            Route {
                operation: S3Request::CopyObject,
                arguments: vec![has_header("x-amz-copy-source")],
            },
        );
        matcher.add_key_route(
            "PUT",
            Route {
                operation: S3Request::PutObject,
                arguments: vec![],
            },
        );
        matcher.add_key_route(
            "PUT",
            Route {
                operation: S3Request::PutObjectAcl,
                arguments: vec![has_query("acl")],
            },
        );
        matcher.add_key_route(
            "PUT",
            Route {
                operation: S3Request::PutObjectLegalHold,
                arguments: vec![has_query("legal-hold")],
            },
        );
        matcher.add_key_route(
            "PUT",
            Route {
                operation: S3Request::PutObjectLockConfiguration,
                arguments: vec![has_query("lock")],
            },
        );
        matcher.add_key_route(
            "PUT",
            Route {
                operation: S3Request::PutObjectRetention,
                arguments: vec![has_query("retention")],
            },
        );
        matcher.add_key_route(
            "PUT",
            Route {
                operation: S3Request::PutObjectTagging,
                arguments: vec![has_query("tagging")],
            },
        );
        matcher.add_key_route(
            "PUT",
            Route {
                operation: S3Request::UploadPart,
                arguments: vec![has_query("uploadId"), has_query("partNumber")],
            },
        );
        matcher.add_key_route(
            "PUT",
            Route {
                operation: S3Request::UploadPartCopy,
                arguments: vec![
                    has_query("uploadId"),
                    has_query("partNumber"),
                    has_header("x-amz-copy-source"),
                ],
            },
        );
        matcher.add_key_route(
            "OPTIONS",
            Route {
                operation: S3Request::OptionsPreflight,
                arguments: vec![
                    has_header("Access-Control-Request-Method"),
                    has_header("Origin"),
                ],
            },
        );

        matcher.add_root_route(
            "GET",
            Route {
                operation: S3Request::ListBuckets,
                arguments: vec![],
            },
        );
        matcher.add_root_route(
            "PUT",
            Route {
                operation: S3Request::WriteGetObjectResponse,
                arguments: vec![
                    has_query("x-amz-request-route"),
                    has_query("x-amz-request-token"),
                ],
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
