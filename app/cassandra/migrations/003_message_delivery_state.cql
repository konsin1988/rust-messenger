CREATE TABLE IF NOT EXISTS messenger.message_delivery_state (
    message_id UUID,
    user_id UUID,
    delivered_at TIMESTAMP,

    PRIMARY KEY ((message_id), user_id)
);
