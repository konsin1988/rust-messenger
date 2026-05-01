-- Add migration script here
CREATE TABLE conversation_participants (
    conversation_id UUID NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role TEXT NOT NULL CHECK (
        role IN ('admin', 'member')
    ),
    joined_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (conversation_id, user_id)
);

CREATE INDEX idx_cp_user_id
ON conversation_participants(user_id);

CREATE INDEX idx_cp_role
ON conversation_participants(role);
