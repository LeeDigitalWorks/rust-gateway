-- Enum for supported storage providers
CREATE TYPE storage_provider AS ENUM ('AWS', 'GCP', 'AZURE', 'DO', 'CEPH');

-- Storage backends configuration
CREATE TABLE storage_backends (
    id UUID PRIMARY KEY,
    name VARCHAR(63) NOT NULL,
    provider storage_provider NOT NULL,
    credentials JSONB NOT NULL,
    endpoint TEXT,  -- For CEPH or custom endpoints
    region TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    is_active BOOLEAN NOT NULL DEFAULT true,
    config JSONB,  -- Provider-specific configuration
    UNIQUE(name)
);

-- Enhanced buckets table
CREATE TABLE buckets (
    id UUID PRIMARY KEY,
    name VARCHAR(63) NOT NULL,
    user_id BIGINT NOT NULL,
    account_id TEXT NOT NULL,
    backend_id UUID NOT NULL REFERENCES storage_backends(id),
    versioning SMALLINT NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    mode INT DEFAULT NULL,
    storageprofile JSONB DEFAULT NULL,
    acl JSONB DEFAULT NULL,
    cors JSONB DEFAULT NULL,
    lifecycle JSONB DEFAULT NULL,
    policy JSONB DEFAULT NULL,
    backend_specific_name TEXT,  -- Native name in the backend (e.g., CEPH bucket name)
    migration_status JSONB DEFAULT NULL,  -- Track ongoing migrations
    UNIQUE(name)
);

-- Enhanced objects table
CREATE TABLE objects (
    id UUID PRIMARY KEY,
    bucket_id UUID NOT NULL REFERENCES buckets(id),
    name TEXT NOT NULL,
    size BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    version_id UUID NOT NULL,
    version_number BIGINT NOT NULL,
    content_type TEXT NOT NULL,
    content_encoding TEXT,
    content_disposition TEXT,
    content_language TEXT,
    cache_control TEXT,
    metadata JSONB,
    storage_class TEXT NOT NULL DEFAULT 'STANDARD',
    owner_id TEXT NOT NULL,
    acl JSONB DEFAULT NULL,
    policy JSONB DEFAULT NULL,
    etag TEXT NOT NULL,
    backend_specific_name TEXT,  -- Native name in the backend (e.g., CEPH object name)
    backend_specific_id TEXT,    -- Backend-specific identifier
    is_latest BOOLEAN NOT NULL DEFAULT true,
    deleted_at TIMESTAMP WITH TIME ZONE,
    restore_expires_at TIMESTAMP WITH TIME ZONE  -- For glacier/archive restoration
);

-- Table for multipart uploads
CREATE TABLE multipart_uploads (
    id UUID PRIMARY KEY,
    bucket_id UUID NOT NULL REFERENCES buckets(id),
    object_name TEXT NOT NULL,
    upload_id TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE,
    metadata JSONB,
    storage_class TEXT NOT NULL DEFAULT 'STANDARD',
    backend_upload_id TEXT,  -- Backend-specific multipart upload ID
    status TEXT NOT NULL DEFAULT 'in_progress',
    UNIQUE(bucket_id, object_name, upload_id)
);

-- Table for multipart parts
CREATE TABLE multipart_parts (
    id UUID PRIMARY KEY,
    multipart_upload_id UUID NOT NULL REFERENCES multipart_uploads(id),
    part_number INT NOT NULL,
    size BIGINT NOT NULL,
    etag TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    backend_specific_name TEXT,  -- Backend-specific part identifier
    status TEXT NOT NULL DEFAULT 'uploaded',
    UNIQUE(multipart_upload_id, part_number)
);

-- Table for lifecycle rules and their execution status
CREATE TABLE lifecycle_rules (
    id UUID PRIMARY KEY,
    bucket_id UUID NOT NULL REFERENCES buckets(id),
    rule_id TEXT NOT NULL,
    rule_config JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_executed_at TIMESTAMP WITH TIME ZONE,
    next_execution_at TIMESTAMP WITH TIME ZONE,
    status TEXT NOT NULL DEFAULT 'active',
    UNIQUE(bucket_id, rule_id)
);

-- Table for tracking backend migrations
CREATE TABLE backend_migrations (
    id UUID PRIMARY KEY,
    bucket_id UUID NOT NULL REFERENCES buckets(id),
    source_backend_id UUID NOT NULL REFERENCES storage_backends(id),
    target_backend_id UUID NOT NULL REFERENCES storage_backends(id),
    started_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMP WITH TIME ZONE,
    status TEXT NOT NULL DEFAULT 'in_progress',
    progress JSONB,  -- Track migration progress
    error TEXT
);

-- Indexes for common queries and performance
CREATE INDEX idx_objects_bucket_name ON objects(bucket_id, name);
CREATE INDEX idx_objects_bucket_version ON objects(bucket_id, version_id);
CREATE INDEX idx_multipart_uploads_bucket ON multipart_uploads(bucket_id);
CREATE INDEX idx_lifecycle_rules_bucket ON lifecycle_rules(bucket_id);
CREATE INDEX idx_backend_migrations_bucket ON backend_migrations(bucket_id);
CREATE INDEX idx_objects_backend_specific ON objects(backend_specific_name);
