CREATE TABLE IF NOT EXISTS buckets (
    id UUID PRIMARY KEY,
    name VARCHAR(63) NOT NULL,
    user_id BIGINT NOT NULL,
    account_id TEXT NOT NULL,
    versioning SMALLINT NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    mode INT DEFAULT NULL,
    storageprofile JSONB DEFAULT NULL,
    acl JSONB DEFAULT NULL,
    cors JSONB DEFAULT NULL,
    lifecycle JSONB DEFAULT NULL,
    policy JSONB DEFAULT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS buckets_name_key ON buckets (name);

CREATE TABLE IF NOT EXISTS objects (
    id UUID PRIMARY KEY,
    bucket_id UUID NOT NULL,
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
    owner_display_name TEXT NOT NULL,
    owner_display_name_normalized TEXT NOT NULL,
    owner_type TEXT NOT NULL,
    acl JSONB DEFAULT NULL,
    policy JSONB DEFAULT NULL,
    etag TEXT NOT NULL
);