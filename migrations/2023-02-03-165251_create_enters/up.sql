-- Your SQL goes here
CREATE TABLE enters (
    id SERIAL PRIMARY KEY,
    room_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    user_name TEXT NOT NULL,
    medal TEXT,
    timestamp TIMESTAMPTZ NOT NULL
);

CREATE INDEX enters_index_room_id ON enters (room_id);

CREATE INDEX enters_index_user_id ON enters (user_id);