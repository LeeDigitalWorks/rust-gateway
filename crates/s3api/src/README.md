# s3api

Uses AmazonS3.xsd to generate a Rust API for Amazon S3.
Contains s3api related types and functions and a router to route to a backend.

## Generating a new xsd
```bash
go install github.com/xuri/xgen/cmd/xgen@latest
xgen -i AmazonS3.xsd -o s3api_xsd_generated.rs
```
