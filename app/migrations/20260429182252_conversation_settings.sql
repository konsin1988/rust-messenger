-- Add migration script here
CREATE TABLE conversation_settings (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    conversation_id UUID NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    muted BOOLEAN NOT NULL DEFAULT FALSE,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, conversation_id)
);

CREATE INDEX idx_cs_user_id
ON conversation_settings(user_id);

CREATE INDEX idx_cs_archived
ON conversation_settings(archived);
