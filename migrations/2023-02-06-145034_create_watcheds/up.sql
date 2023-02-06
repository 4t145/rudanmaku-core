-- Your SQL goes here
CREATE TABLE watcheds (
    id SERIAL PRIMARY KEY,
    count INTEGER NOT NULL,
    room_id BIGINT NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL
);

CREATE INDEX watcheds_index_room_id ON watcheds (room_id)
