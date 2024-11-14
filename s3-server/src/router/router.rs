use std::collections::{BTreeMap, HashMap};

use axum::{body::Bytes, http::Request};
use s3_core::S3Action;

pub struct Result {
    pub action: S3Action,
    pub bucket: String,
    pub key: String,
    pub host: String,
}

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

    pub fn match_result(&self, req: &axum::http::Request<Bytes>) -> Result {
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

        Result {
            action: S3Action::Unknown,
            bucket: "".to_string(),
            key: "".to_string(),
            host: "".to_string(),
        }
    }

    fn match_route(
        &self,
        host: &str,
        bucket: &str,
        key: &str,
        req: &axum::http::Request<Bytes>,
    ) -> Result {
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
                    return Result {
                        action: route.operation.clone(),
                        bucket: bucket.to_string(),
                        key: key.to_string(),
                        host: host.to_string(),
                    };
                }
            }
        }

        Result {
            action: S3Action::Unknown,
            bucket: bucket.to_string(),
            key: key.to_string(),
            host: host.to_string(),
        }
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
                operation: S3Action::CreateSession,
                arguments: vec![has_query("session")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Action::GetBucketAccelerateConfiguration,
                arguments: vec![has_query("accelerate")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Action::GetBucketAcl,
                arguments: vec![has_query("acl")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Action::GetBucketAnalyticsConfiguration,
                arguments: vec![has_query("analytics"), has_query("id")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Action::GetBucketCors,
                arguments: vec![has_query("cors")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Action::GetBucketEncryption,
                arguments: vec![has_query("encryption")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Action::GetBucketInventoryConfiguration,
                arguments: vec![has_query("inventory"), has_query("id")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Action::GetBucketLifecycleConfiguration,
                arguments: vec![has_query("lifecycle")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Action::GetBucketLocation,
                arguments: vec![has_query("location")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Action::GetBucketLogging,
                arguments: vec![has_query("logging")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Action::GetBucketMetricsConfiguration,
                arguments: vec![has_query("metrics"), has_query("id")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Action::GetBucketNotificationConfiguration,
                arguments: vec![has_query("notification")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Action::GetBucketOwnershipControls,
                arguments: vec![has_query("ownershipControls")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Action::GetBucketPolicy,
                arguments: vec![has_query("policy")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Action::GetBucketReplication,
                arguments: vec![has_query("replication")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Action::GetBucketRequestPayment,
                arguments: vec![has_query("requestPayment")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Action::GetBucketTagging,
                arguments: vec![has_query("tagging")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Action::GetBucketVersioning,
                arguments: vec![has_query("versioning")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Action::GetBucketWebsite,
                arguments: vec![has_query("website")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Action::ListBucketAnalyticsConfigurations,
                arguments: vec![has_query("analytics")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Action::ListBucketInventoryConfigurations,
                arguments: vec![has_query("inventory")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Action::ListBucketMetricsConfigurations,
                arguments: vec![has_query("metrics")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Action::ListMultipartUploads,
                arguments: vec![has_query("uploads")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Action::ListObjects,
                arguments: vec![],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Action::ListObjectsV2,
                arguments: vec![has_query_value("list-type", "2")],
            },
        );
        matcher.add_bucket_route(
            "GET",
            Route {
                operation: S3Action::ListObjectVersions,
                arguments: vec![has_query("versions")],
            },
        );
        matcher.add_bucket_route(
            "HEAD",
            Route {
                operation: S3Action::HeadBucket,
                arguments: vec![],
            },
        );
        matcher.add_bucket_route(
            "OPTIONS",
            Route {
                operation: S3Action::OptionsPreflight,
                arguments: vec![
                    has_header("Access-Control-Request-Method"),
                    has_header("Origin"),
                ],
            },
        );
        matcher.add_bucket_route(
            "DELETE",
            Route {
                operation: S3Action::DeleteBucketAnalyticsConfiguration,
                arguments: vec![has_query("analytics"), has_query("id")],
            },
        );
        matcher.add_bucket_route(
            "DELETE",
            Route {
                operation: S3Action::DeleteBucketCors,
                arguments: vec![has_query("cors")],
            },
        );
        matcher.add_bucket_route(
            "DELETE",
            Route {
                operation: S3Action::DeleteBucketEncryption,
                arguments: vec![has_query("encryption")],
            },
        );
        matcher.add_bucket_route(
            "DELETE",
            Route {
                operation: S3Action::DeleteBucketInventoryConfiguration,
                arguments: vec![has_query("inventory"), has_query("id")],
            },
        );
        matcher.add_bucket_route(
            "DELETE",
            Route {
                operation: S3Action::DeleteBucketLifecycle,
                arguments: vec![has_query("lifecycle")],
            },
        );
        matcher.add_bucket_route(
            "DELETE",
            Route {
                operation: S3Action::DeleteBucketMetricsConfiguration,
                arguments: vec![has_query("metrics"), has_query("id")],
            },
        );
        matcher.add_bucket_route(
            "DELETE",
            Route {
                operation: S3Action::DeleteBucketOwnershipControls,
                arguments: vec![has_query("ownershipControls")],
            },
        );
        matcher.add_bucket_route(
            "DELETE",
            Route {
                operation: S3Action::DeleteBucketPolicy,
                arguments: vec![has_query("policy")],
            },
        );
        matcher.add_bucket_route(
            "DELETE",
            Route {
                operation: S3Action::DeleteBucketReplication,
                arguments: vec![has_query("replication")],
            },
        );
        matcher.add_bucket_route(
            "DELETE",
            Route {
                operation: S3Action::DeleteBucketTagging,
                arguments: vec![has_query("tagging")],
            },
        );
        matcher.add_bucket_route(
            "DELETE",
            Route {
                operation: S3Action::DeleteBucketWebsite,
                arguments: vec![has_query("website")],
            },
        );
        matcher.add_bucket_route(
            "DELETE",
            Route {
                operation: S3Action::DeleteBucketIntelligentTieringConfiguration,
                arguments: vec![has_query("intelligent-tiering"), has_query("id")],
            },
        );
        matcher.add_bucket_route(
            "DELETE",
            Route {
                operation: S3Action::DeletePublicAccessBlock,
                arguments: vec![has_query("publicAccessBlock")],
            },
        );
        matcher.add_bucket_route(
            "DELETE",
            Route {
                operation: S3Action::DeleteBucket,
                arguments: vec![],
            },
        );
        matcher.add_bucket_route(
            "POST",
            Route {
                operation: S3Action::DeleteObjects,
                arguments: vec![has_query("delete")],
            },
        );
        matcher.add_bucket_route(
            "POST",
            Route {
                operation: S3Action::PostObject,
                arguments: vec![has_header_value("Content-Type", "multipart/form-data")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Action::PutBucketAccelerateConfiguration,
                arguments: vec![has_query("accelerate")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Action::PutBucketAcl,
                arguments: vec![has_query("acl")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Action::PutBucketAnalyticsConfiguration,
                arguments: vec![has_query("analytics"), has_query("id")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Action::PutBucketCors,
                arguments: vec![has_query("cors")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Action::PutBucketEncryption,
                arguments: vec![has_query("encryption")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Action::PutBucketInventoryConfiguration,
                arguments: vec![has_query("inventory"), has_query("id")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Action::PutBucketLifecycleConfiguration,
                arguments: vec![has_query("lifecycle")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Action::PutBucketLogging,
                arguments: vec![has_query("logging")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Action::PutBucketMetricsConfiguration,
                arguments: vec![has_query("metrics"), has_query("id")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Action::PutBucketNotificationConfiguration,
                arguments: vec![has_query("notification")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Action::PutBucketOwnershipControls,
                arguments: vec![has_query("ownershipControls")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Action::PutBucketPolicy,
                arguments: vec![has_query("policy")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Action::PutBucketReplication,
                arguments: vec![has_query("replication")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Action::PutBucketRequestPayment,
                arguments: vec![has_query("requestPayment")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Action::PutBucketTagging,
                arguments: vec![has_query("tagging")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Action::PutBucketVersioning,
                arguments: vec![has_query("versioning")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Action::PutBucketWebsite,
                arguments: vec![has_query("website")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Action::PutBucketIntelligentTieringConfiguration,
                arguments: vec![has_query("intelligent-tiering"), has_query("id")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Action::PutPublicAccessBlock,
                arguments: vec![has_query("publicAccessBlock")],
            },
        );
        matcher.add_bucket_route(
            "PUT",
            Route {
                operation: S3Action::CreateBucket,
                arguments: vec![],
            },
        );

        matcher.add_key_route(
            "GET",
            Route {
                operation: S3Action::GetObject,
                arguments: vec![],
            },
        );
        matcher.add_key_route(
            "GET",
            Route {
                operation: S3Action::GetObjectAcl,
                arguments: vec![has_query("acl")],
            },
        );
        matcher.add_key_route(
            "GET",
            Route {
                operation: S3Action::GetObjectAttributes,
                arguments: vec![has_query("attributes")],
            },
        );
        matcher.add_key_route(
            "GET",
            Route {
                operation: S3Action::GetObjectLegalHold,
                arguments: vec![has_query("legal-hold")],
            },
        );
        matcher.add_key_route(
            "GET",
            Route {
                operation: S3Action::GetObjectLockConfiguration,
                arguments: vec![has_query("lock")],
            },
        );
        matcher.add_key_route(
            "GET",
            Route {
                operation: S3Action::GetObjectRetention,
                arguments: vec![has_query("retention")],
            },
        );
        matcher.add_key_route(
            "GET",
            Route {
                operation: S3Action::GetObjectTagging,
                arguments: vec![has_query("tagging")],
            },
        );
        matcher.add_key_route(
            "GET",
            Route {
                operation: S3Action::GetObjectTorrent,
                arguments: vec![has_query("torrent")],
            },
        );
        matcher.add_key_route(
            "GET",
            Route {
                operation: S3Action::ListParts,
                arguments: vec![has_query("uploadId")],
            },
        );
        matcher.add_key_route(
            "HEAD",
            Route {
                operation: S3Action::HeadObject,
                arguments: vec![],
            },
        );
        matcher.add_key_route(
            "DELETE",
            Route {
                operation: S3Action::DeleteObject,
                arguments: vec![],
            },
        );
        matcher.add_key_route(
            "DELETE",
            Route {
                operation: S3Action::DeleteObjectTagging,
                arguments: vec![has_query("tagging")],
            },
        );
        matcher.add_key_route(
            "DELETE",
            Route {
                operation: S3Action::AbortMultipartUpload,
                arguments: vec![has_query("uploadId")],
            },
        );
        matcher.add_key_route(
            "POST",
            Route {
                operation: S3Action::PostObject,
                arguments: vec![has_header_value("Content-Type", "multipart/form-data")],
            },
        );
        matcher.add_key_route(
            "POST",
            Route {
                operation: S3Action::CompleteMultipartUpload,
                arguments: vec![has_query("uploadId")],
            },
        );
        matcher.add_key_route(
            "POST",
            Route {
                operation: S3Action::RestoreObject,
                arguments: vec![has_query("restore")],
            },
        );
        matcher.add_key_route(
            "POST",
            Route {
                operation: S3Action::CreateMultipartUpload,
                arguments: vec![has_query("uploads")],
            },
        );
        matcher.add_key_route(
            "POST",
            Route {
                operation: S3Action::SelectObjectContent,
                arguments: vec![has_query("select"), has_query_value("select-type", "2")],
            },
        );
        matcher.add_key_route(
            "PUT",
            Route {
                operation: S3Action::CopyObject,
                arguments: vec![has_header("x-amz-copy-source")],
            },
        );
        matcher.add_key_route(
            "PUT",
            Route {
                operation: S3Action::PutObject,
                arguments: vec![],
            },
        );
        matcher.add_key_route(
            "PUT",
            Route {
                operation: S3Action::PutObjectAcl,
                arguments: vec![has_query("acl")],
            },
        );
        matcher.add_key_route(
            "PUT",
            Route {
                operation: S3Action::PutObjectLegalHold,
                arguments: vec![has_query("legal-hold")],
            },
        );
        matcher.add_key_route(
            "PUT",
            Route {
                operation: S3Action::PutObjectLockConfiguration,
                arguments: vec![has_query("lock")],
            },
        );
        matcher.add_key_route(
            "PUT",
            Route {
                operation: S3Action::PutObjectRetention,
                arguments: vec![has_query("retention")],
            },
        );
        matcher.add_key_route(
            "PUT",
            Route {
                operation: S3Action::PutObjectTagging,
                arguments: vec![has_query("tagging")],
            },
        );
        matcher.add_key_route(
            "PUT",
            Route {
                operation: S3Action::UploadPart,
                arguments: vec![has_query("uploadId"), has_query("partNumber")],
            },
        );
        matcher.add_key_route(
            "PUT",
            Route {
                operation: S3Action::UploadPartCopy,
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
                operation: S3Action::OptionsPreflight,
                arguments: vec![
                    has_header("Access-Control-Request-Method"),
                    has_header("Origin"),
                ],
            },
        );

        matcher.add_root_route(
            "GET",
            Route {
                operation: S3Action::ListBuckets,
                arguments: vec![],
            },
        );
        matcher.add_root_route(
            "PUT",
            Route {
                operation: S3Action::WriteGetObjectResponse,
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
    operation: S3Action,
    arguments: Vec<HasArguments>,
}

impl Route {
    pub fn match_route(&self, req: &axum::http::Request<Bytes>) -> bool {
        self.arguments.iter().all(|arg| arg(&req))
    }
}

type HasArguments = Box<dyn Fn(&Request<Bytes>) -> bool + Send + Sync>;

fn has_query(name: &str) -> HasArguments {
    let name = name.to_string();
    Box::new(move |req: &axum::http::Request<Bytes>| {
        let query = req.uri().query().unwrap_or("");
        query.contains(&name)
    })
}

fn has_query_value(name: &str, value: &str) -> HasArguments {
    let name = name.to_string();
    let value = value.to_string();
    Box::new(move |req: &axum::http::Request<Bytes>| {
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
    Box::new(move |req: &axum::http::Request<Bytes>| req.headers().get(&name).is_some())
}

fn has_header_value(name: &str, value: &str) -> HasArguments {
    let name = name.to_string();
    let value = value.to_string();
    Box::new(move |req: &axum::http::Request<Bytes>| {
        req.headers()
            .get(&name)
            .map(|v| v.to_str().unwrap_or("") == value)
            .unwrap_or(false)
    })
}
