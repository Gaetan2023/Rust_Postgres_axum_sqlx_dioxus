-- Add migration script here
CREATE TABLE IF NOT EXISTS posts(
id SERIAL PRIMARY KEY,
name TEXT NOT NULL,
email TEXT NOT NULL,
message TEXT NOT NULL );
