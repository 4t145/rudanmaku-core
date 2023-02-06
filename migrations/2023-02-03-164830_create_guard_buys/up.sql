-- Your SQL goes here
CREATE TABLE guard_buys (
    id SERIAL PRIMARY KEY,
    room_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    user_name TEXT NOT NULL,
    guards_level SMALLINT NOT NULL,
    price INTEGER NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL
);

CREATE INDEX guard_buys_index_room_id
ON guard_buys (room_id);

CREATE INDEX guard_buys_index_user_id
ON guard_buys (user_id);