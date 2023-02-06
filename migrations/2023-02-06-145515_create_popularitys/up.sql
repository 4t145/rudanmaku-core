-- Your SQL goes here
CREATE TABLE popularitys (
    id SERIAL PRIMARY KEY,
    count INTEGER NOT NULL,
    room_id BIGINT NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL
);

CREATE INDEX popularitys_index_room_id ON popularitys (room_id);
