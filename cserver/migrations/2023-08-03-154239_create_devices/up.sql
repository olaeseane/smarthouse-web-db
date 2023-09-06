-- Your SQL goes here
CREATE TABLE devices (
    id VARCHAR NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    room_id VARCHAR,
    FOREIGN KEY (room_id) REFERENCES rooms (id) ON DELETE CASCADE
);