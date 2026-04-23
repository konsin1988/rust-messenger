-- Add migration script here
CREATE TABLE contacts (
    id UUID PRIMARY KEY,

    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- the other user in the relationship
    contact_user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- relationship state
    status TEXT NOT NULL CHECK (
        status IN ('pending', 'accepted', 'blocked')
    ),

    -- who initiated the relationship
    requested_by UUID NOT NULL REFERENCES users(id),

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    -- prevent duplicate relationships
    UNIQUE (user_id, contact_user_id)
);

-- fast lookup: all contacts of a user
CREATE INDEX idx_contacts_user_id ON contacts(user_id);

-- reverse lookup (useful for mutual queries / requests)
CREATE INDEX idx_contacts_contact_user_id ON contacts(contact_user_id);

-- find pending requests quickly
CREATE INDEX idx_contacts_status ON contacts(status);
