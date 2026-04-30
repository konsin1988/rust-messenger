CREATE TABLE IF NOT EXISTS attachments (
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
CREATE UNIQUE INDEX IF NOT EXISTS idx_attachments_bucket_object
ON attachments (bucket, object_key);

-- query by uploader
CREATE INDEX IF NOT EXISTS idx_attachments_uploaded_by
ON attachments (uploaded_by);

-- filter active attachments
CREATE INDEX IF NOT EXISTS idx_attachments_not_deleted
ON attachments (is_deleted);
