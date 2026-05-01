-- Add migration script here
CREATE TABLE blocks (
    blocker_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    blocked_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (blocker_id, blocked_id)
);

CREATE INDEX idx_blocks_blocked_id
ON blocks(blocked_id);

CREATE INDEX idx_blocks_blocker_id
ON blocks(blocker_id);

ALTER TABLE blocks
ADD CONSTRAINT no_self_block
CHECK (blocker_id <> blocked_id);
