CREATE TABLE IF NOT EXISTS messenger.unread_counters (
    user_id UUID,
    conversation_id UUID,
    unread_count COUNTER,

    PRIMARY KEY (user_id, conversation_id)
);
