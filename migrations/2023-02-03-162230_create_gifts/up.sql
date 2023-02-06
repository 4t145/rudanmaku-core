-- Your SQL goes here
CREATE TABLE gifts (
    id SERIAL PRIMARY KEY,
    room_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    user_name TEXT NOT NULL,
    gift_id INTEGER NOT NULL,
    gift_name TEXT NOT NULL,
    gift_price INTEGER NOT NULL, 
    gift_count INTEGER NOT NULL,
    gift_paid BOOLEAN NOT NULL,
    medal TEXT,
    timestamp TIMESTAMPTZ NOT NULL
);

CREATE INDEX gifts_index_room_id
ON gifts (room_id);

CREATE INDEX gifts_index_user_id
ON gifts (user_id);