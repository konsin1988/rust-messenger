-- Add migration script here
CREATE TABLE devices (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- device identification
    device_name TEXT,
    user_agent TEXT,

    -- auth/session
    refresh_token TEXT NOT NULL UNIQUE,
    last_seen_at TIMESTAMP,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP
);

CREATE INDEX idx_devices_user_id ON devices(user_id);
CREATE INDEX idx_devices_refresh_token ON devices(refresh_token);
