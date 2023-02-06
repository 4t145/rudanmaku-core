-- Your SQL goes here
CREATE TABLE superchats (
    id SERIAL PRIMARY KEY,
    room_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    user_name TEXT NOT NULL,
    price INTEGER NOT NULL,
    message TEXT NOT NULL,
    medal TEXT,
    timestamp TIMESTAMPTZ NOT NULL
);

CREATE INDEX superchats_index_room_id
ON superchats (room_id);

CREATE INDEX superchats_index_user_id
ON superchats (user_id);