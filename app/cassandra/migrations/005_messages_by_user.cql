CREATE TABLE IF NOT EXISTS messenger.messages_by_user (
    user_id UUID,
    conversation_id UUID,
    last_message_id TIMEUUID,
    last_message_text TEXT,
    last_message_at TIMESTAMP,

    PRIMARY KEY (user_id, last_message_at, conversation_id)
) WITH CLUSTERING ORDER BY (last_message_at DESC);
