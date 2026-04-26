CREATE TABLE IF NOT EXISTS messenger.read_receipts (
    conversation_id UUID,
    user_id UUID,
    last_read_message_id TIMEUUID,
    read_at TIMESTAMP,

    PRIMARY KEY ((conversation_id), user_id)
);
