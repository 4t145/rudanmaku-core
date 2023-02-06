-- Your SQL goes here
CREATE TABLE danmakus (
    id SERIAL PRIMARY KEY,
    room_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    user_name TEXT NOT NULL,
    message TEXT NOT NULL,
    essential_message TEXT NOT NULL,
    is_emoticon BOOLEAN NOT NULL,
    flag INTEGER NOT NULL,
    medal TEXT,
    timestamp TIMESTAMPTZ NOT NULL
);

CREATE INDEX danmakus_index_room_id
ON danmakus (room_id);

CREATE INDEX danmakus_index_user_id
ON danmakus (user_id);