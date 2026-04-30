-- Add migration script here
CREATE TABLE conversations (
    id UUID PRIMARY KEY,
    type TEXT NOT NULL CHECK (
        type IN ('private', 'group')
    ),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES users(id)
);

CREATE INDEX idx_conversations_created_at
ON conversations(created_at);
