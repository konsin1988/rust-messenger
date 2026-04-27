CREATE TABLE IF NOT EXISTS files (
    file_id UUID PRIMARY KEY,
    bucket TEXT NOT NULL,
    object_key TEXT NOT NULL,
    size BIGINT NOT NULL,
    mime_type TEXT NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    uploaded_by UUID,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE
);

-- Prevent duplicate objects in same bucket
CREATE UNIQUE INDEX IF NOT EXISTS idx_files_bucket_object
ON files (bucket, object_key);

-- query by uploader
CREATE INDEX IF NOT EXISTS idx_files_uploaded_by
ON files (uploaded_by);

-- filter active files
CREATE INDEX IF NOT EXISTS idx_files_not_deleted
ON files (is_deleted);
